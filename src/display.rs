use core::cell::RefCell;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    gpio::{Input, Level, Output, Pin, Pull},
    peripherals::SPI0,
    spi::{self, Blocking},
    Peripheral,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Delay;
use epd_waveshare::epd7in5b_v3::Epd7in5;
use epd_waveshare::prelude::WaveshareDisplay;
use static_cell::StaticCell;

#[derive(Debug)]
pub enum ScreenError {
    SpiError,
}

pub trait Screen {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn draw(&mut self, buffer: &[u8]) -> Result<(), ScreenError>;
    fn sleep(&mut self) -> Result<(), ScreenError>;
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
        self.power_output.set_high();
    }

    fn power_off(&mut self) {
        self.power_output.set_low();
    }

    fn sleep(&mut self) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .sleep(self.spi_device, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }

    fn draw(&mut self, buffer: &[u8]) -> Result<(), ScreenError> {
        Ok(self
            .epd
            .update_and_display_frame(self.spi_device, buffer, &mut self.delay)
            .map_err(|_| ScreenError::SpiError)?)
    }
}

pub fn init_display<MOSI, CLOCK>(
    spi: SPI0,
    mosi: MOSI,
    clk: CLOCK,
    command_selection_pin: impl Peripheral<P = impl Pin> + 'static,
    display_power_pin: impl Peripheral<P = impl Pin> + 'static,
    display_busy_pin: impl Peripheral<P = impl Pin> + 'static,
    display_data_command_pin: impl Peripheral<P = impl Pin> + 'static,
    display_reset_pin: impl Peripheral<P = impl Pin> + 'static,
) -> WaveshareScreen<
    'static,
    SpiDeviceWithConfig<'static, NoopRawMutex, spi::Spi<'static, SPI0, Blocking>, Output<'static>>,
>
where
    MOSI: spi::MosiPin<SPI0>,
    CLOCK: spi::ClkPin<SPI0>,
{
    let mut display_config = spi::Config::default();
    display_config.frequency = 4_000_000u32;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;
    let spi = spi::Spi::new_blocking_txonly(spi, clk, mosi, display_config.clone());
    static BUS: StaticCell<Mutex<NoopRawMutex, RefCell<spi::Spi<'static, SPI0, Blocking>>>> =
        StaticCell::new();
    let spi_bus = &*BUS.init(Mutex::new(RefCell::new(spi)));

    static SPI_DEVICE: StaticCell<
        SpiDeviceWithConfig<NoopRawMutex, spi::Spi<'static, SPI0, Blocking>, Output<'static>>,
    > = StaticCell::new();
    let spi_device = &mut *SPI_DEVICE.init(SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(command_selection_pin, Level::Low),
        display_config,
    ));
    let display_reset = Output::new(display_reset_pin, Level::Low);
    let display_data_command = Output::new(display_data_command_pin, Level::Low);
    let display_busy = Input::new(display_busy_pin, Pull::Down);
    let epd = Epd7in5::new(
        spi_device,
        display_busy,
        display_data_command,
        display_reset,
        &mut Delay,
        None,
    )
    .unwrap();
    WaveshareScreen {
        epd,
        spi_device,
        power_output: Output::new(display_power_pin, Level::Low),
        delay: Delay {},
    }
}
