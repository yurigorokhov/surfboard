use core::cell::RefCell;

use crate::{
    system::{drawing::DisplayAction, resources::ScreenResources},
    task::state::STATE_MANAGER_MUTEX,
};
use defmt::*;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    peripherals::SPI0,
    spi::{self, Blocking},
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Delay;
use embedded_graphics::prelude::*;
use epd_waveshare::epd7in5b_v3::Display7in5;
use epd_waveshare::epd7in5b_v3::Epd7in5;
use epd_waveshare::prelude::WaveshareDisplay;
use static_cell::StaticCell;

pub static DISPLAY_CHANNEL: Channel<CriticalSectionRawMutex, DisplayAction, 4> = Channel::new();

/// Requests a display update with the specified action
pub async fn display_update(display_action: DisplayAction) {
    DISPLAY_CHANNEL.send(display_action).await;
}

/// Blocks until next update request, returns the requested display action
async fn wait() -> DisplayAction {
    DISPLAY_CHANNEL.receive().await
}

#[derive(Debug)]
pub enum ScreenError {
    SpiError,
}

pub trait Screen {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn draw(&mut self, buffer: &[u8]) -> Result<(), ScreenError>;
    fn draw_partial(&mut self, buffer: &[u8], x: u32, y: u32, width: u32, height: u32) -> Result<(), ScreenError>;
    fn sleep(&mut self) -> Result<(), ScreenError>;
    fn wake_up(&mut self) -> Result<(), ScreenError>;

    fn sleep_and_power_off(&mut self) -> Result<(), ScreenError> {
        self.sleep()?;
        self.power_off();
        Ok(())
    }
}

pub struct WaveshareScreen<'a, S>
where
    S: embedded_hal::spi::SpiDevice,
{
    spi_device: &'a mut S,
    power_output: Output<'a>,
    epd: Epd7in5<S, Input<'a>, Output<'a>, Output<'a>, Delay>,
    delay: Delay,
}

impl<'a, S> Screen for WaveshareScreen<'a, S>
where
    S: embedded_hal::spi::SpiDevice,
{
    fn power_on(&mut self) {
        if self.power_output.is_set_low() {
            self.power_output.set_high();
        }
    }

    fn power_off(&mut self) {
        if self.power_output.is_set_high() {
            self.power_output.set_low();
        }
    }

    fn sleep(&mut self) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .sleep(self.spi_device, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }

    fn wake_up(&mut self) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .wake_up(self.spi_device, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }

    fn draw(&mut self, buffer: &[u8]) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .update_and_display_frame(self.spi_device, buffer, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }

    fn draw_partial(&mut self, buffer: &[u8], x: u32, y: u32, width: u32, height: u32) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .update_partial_frame2(self.spi_device, buffer, x, y, width, height, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }
}

#[embassy_executor::task]
pub async fn start(r: ScreenResources) {
    debug!("Initializing display");
    let mut display = init_display(r);
    display.sleep().expect("Failed to put screen to sleep");

    // clear display
    let mut canvas = Display7in5::default();
    canvas.clear(epd_waveshare::color::TriColor::White).unwrap();

    loop {
        // Wait for the next display update request and clear the display
        let display_action = wait().await;
        if display_action == DisplayAction::DrawImage {
            canvas.clear(epd_waveshare::color::TriColor::White).unwrap();
        }

        display.wake_up().expect("Failed to wake up");
        {
            let state_guard = STATE_MANAGER_MUTEX.lock().await;
            display_action
                .draw(&mut canvas, &*state_guard)
                .expect("Failed to draw splash screen");
        }
        display.draw(canvas.buffer()).expect("Failed to draw on screen");
        display.sleep().expect("Failed to put screen to sleep");
    }
}

fn init_display(
    r: ScreenResources,
) -> WaveshareScreen<
    'static,
    SpiDeviceWithConfig<'static, NoopRawMutex, spi::Spi<'static, SPI0, Blocking>, Output<'static>>,
> {
    let mut display_config = spi::Config::default();
    display_config.frequency = 4_000_000u32;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;
    let spi = spi::Spi::new_blocking_txonly(r.spi, r.clk, r.mosi, display_config.clone());
    static BUS: StaticCell<Mutex<NoopRawMutex, RefCell<spi::Spi<'static, SPI0, Blocking>>>> = StaticCell::new();
    let spi_bus = &*BUS.init(Mutex::new(RefCell::new(spi)));

    static SPI_DEVICE: StaticCell<
        SpiDeviceWithConfig<NoopRawMutex, spi::Spi<'static, SPI0, Blocking>, Output<'static>>,
    > = StaticCell::new();
    let spi_device = &mut *SPI_DEVICE.init(SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(r.command_selection_pin, Level::Low),
        display_config,
    ));
    let display_reset = Output::new(r.display_reset_pin, Level::Low);
    let display_data_command = Output::new(r.display_data_command_pin, Level::Low);
    let display_busy = Input::new(r.display_busy_pin, Pull::Down);
    let mut display_power = Output::new(r.display_power_pin, Level::Low);
    display_power.set_high();
    debug!("display powered on");
    let epd = Epd7in5::new(
        spi_device,
        display_busy,
        display_data_command,
        display_reset,
        &mut Delay,
        None,
    )
    .expect("Display failed to initialize");
    debug!("display initialized");
    WaveshareScreen {
        epd,
        spi_device,
        power_output: display_power,
        delay: Delay {},
    }
}
