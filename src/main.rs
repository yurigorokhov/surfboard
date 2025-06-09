#![no_std]
#![no_main]

use core::cell::RefCell;
use core::str::FromStr;
use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_net::{
    new as new_stack, Config as NetConfig, DhcpConfig, Runner, Stack, StackResources,
};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::spi;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Delay, Duration, Instant, Timer};
use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::primitives::Circle;
use epd_waveshare::color::TriColor;
use epd_waveshare::prelude::WaveshareDisplay;
use rand_core::RngCore;
use static_cell::StaticCell;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use embedded_graphics::{
    pixelcolor::BinaryColor::On as Black,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use epd_waveshare::epd7in5b_v3::{Display7in5, Epd7in5};

use crate::display::{init_display, Screen};

use {defmt_rtt as _, panic_probe as _};

mod random;
use self::random::RngWrapper;

mod http;
use self::http::Client as HttpClient;
use self::http::ClientTrait;

mod display;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});

// #[cortex_m_rt::pre_init]
// unsafe fn before_main() {
//     // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
//     // before we hit main to avoid deadlocks when using a debugger
//     embassy_rp::pac::SIO.spinlock(31).write_value(1);
// }

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(runner: &'static mut Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());

    let mut display_device = init_display(
        p.SPI0, p.PIN_3, p.PIN_6, p.PIN_5, p.PIN_12, p.PIN_13, p.PIN_14, p.PIN_11,
    );

    display_device.power_on();

    // Configure screen
    let mut display = Display7in5::default();
    display
        .clear(epd_waveshare::color::TriColor::White)
        .unwrap();

    // Use embedded graphics for drawing a line
    let _ = Line::new(Point::new(10, 10), Point::new(300, 300))
        .into_styled(PrimitiveStyle::with_stroke(
            epd_waveshare::color::TriColor::Chromatic,
            5,
        ))
        .draw(&mut display);
    let _ = Circle::with_center(Point::new(300, 300), 50)
        .into_styled(PrimitiveStyle::with_stroke(
            epd_waveshare::color::TriColor::Black,
            5,
        ))
        .draw(&mut display);

    // Create a new character style.
    let character_style = MonoTextStyle::new(&FONT_10X20, TriColor::Black);

    // Create a new text style.
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(150))
        .build();

    // Create a text at position (20, 30) and draw it using the previously defined style.
    Text::with_text_style(
        "Hello from embedded Rust!",
        Point::new(20, 30),
        character_style,
        text_style,
    )
    .draw(&mut display)
    .unwrap();

    // Display updated fram
    info!("drawing");
    display_device
        .draw(&display.buffer())
        .expect("Failed to draw on screen");
    Timer::after(Duration::from_millis(2_000)).await;

    // Text::with_text_style(
    //     "- Yuri Gorokhov",
    //     Point::new(20, 50),
    //     MonoTextStyle::new(&FONT_10X20, TriColor::Chromatic),
    //     text_style,
    // )
    // .draw(&mut display)
    // .unwrap();

    // info!("drawing");
    // epd.update_and_display_frame(&mut display_spi, &display.buffer(), &mut delay)
    //     .expect("drawing frame");

    // // Set the EPD to sleep
    // Timer::after(Duration::from_millis(2_000)).await;
    // epd.sleep(&mut display_spi, &mut delay).unwrap();
    // display_device.power_off();
    // info!("display sleeping");

    // Configure PIO and CYW43
    // let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    // let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    // let pwr = Output::new(p.PIN_23, Level::Low);
    // let cs = Output::new(p.PIN_25, Level::High);
    // let mut pio = Pio::new(p.PIO0, Irqs);
    // let spi = PioSpi::new(
    //     &mut pio.common,
    //     pio.sm0,
    //     DEFAULT_CLOCK_DIVIDER,
    //     pio.irq0,
    //     cs,
    //     p.PIN_24,
    //     p.PIN_29,
    //     p.DMA_CH0,
    // );

    // static STATE: StaticCell<cyw43::State> = StaticCell::new();
    // let state = STATE.init(cyw43::State::new());
    // let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    // unwrap!(spawner.spawn(wifi_task(runner)));

    // control.init(clm).await;
    // control
    //     .set_power_management(cyw43::PowerManagementMode::Performance)
    //     .await;

    // let mut rand = RngWrapper::new();

    let delay = Duration::from_secs(1);
    // Timer::after(delay).await;

    // let wifi_ssid = env!("WIFI_SSID");
    // let wifi_password = env!("WIFI_PASSWORD");
    // let client_name: &str = "picow";

    // let mut dhcp_config = DhcpConfig::default();
    // dhcp_config.hostname = Some(heapless::String::from_str(client_name).unwrap());
    // let net_config = NetConfig::dhcpv4(dhcp_config);

    // static STACK: StaticCell<Stack<'static>> = StaticCell::new();
    // static RUNNER: StaticCell<Runner<'static, cyw43::NetDriver<'static>>> = StaticCell::new();

    // // Increase this if you start getting socket ring errors.
    // static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    // let (s, r) = new_stack(
    //     net_device,
    //     net_config,
    //     RESOURCES.init(StackResources::<5>::new()),
    //     rand.next_u64(),
    // );
    // let stack = &*STACK.init(s);
    // let runner = &mut *RUNNER.init(r);
    // let mac_addr = stack.hardware_address();
    // info!("Hardware configured. MAC Address is {}", mac_addr);

    // // Start networking services thread
    // unwrap!(spawner.spawn(net_task(runner)));

    // // join Wifi
    // info!("Joining Wifi {}", wifi_ssid);
    // control
    //     .join(wifi_ssid, JoinOptions::new(wifi_password.as_bytes()))
    //     .await
    //     .unwrap();

    // let start = Instant::now().as_millis();
    // loop {
    //     let elapsed = Instant::now().as_millis() - start;
    //     if elapsed > 15_000 {
    //         core::panic!("Couldn't get network up after 15 seconds");
    //     } else if stack.is_config_up() {
    //         info!("Network stack config completed after about {} ms", elapsed);
    //         break;
    //     } else {
    //         Timer::after_millis(10).await;
    //     }
    // }

    // match stack.config_v4() {
    //     Some(a) => info!("IP Address appears to be: {}", a.address),
    //     None => core::panic!("DHCP completed but no IP address was assigned!"),
    // }

    // let mut http_client = HttpClient::new(*stack, rand);
    // let response = http_client
    //     .send_request("http://gorokhov.io")
    //     .await
    //     .unwrap();
    // let response_str = heapless::String::from_utf8(response).unwrap();
    // info!("STRING: {}", response_str[0..100]);

    // control
    //     .set_power_management(cyw43::PowerManagementMode::Aggressive)
    //     .await;

    loop {
        info!("external LED on, onboard LED off!");
        // led.set_high();
        // control.gpio_set(0, false).await;
        Timer::after(delay).await;

        info!("external LED off, onboard LED on!");
        // led.set_low();
        // control.gpio_set(0, true).await;
        Timer::after(delay).await;
    }
}
