use core::str::FromStr;
use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{new as new_stack, Config as NetConfig, DhcpConfig, Runner, Stack, StackResources};
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Instant, Timer};

use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::Pio;
use rand_core::RngCore;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use static_cell::StaticCell;

#[cfg(feature = "fake_responses")]
use crate::fake::fake_http::FakeHttpClient;
use crate::random::RngWrapper;
use crate::system::event::{send_error, send_event, Events, WifiAction};
use crate::system::net::{HttpClientProvider, HttpDataProvider, HttpJsonDataProvider};
use crate::system::resources::{Irqs, WifiResources};
use crate::task::state::{Configuration, STATE_MANAGER_MUTEX};

pub static DATA_REQUEST_CHANNEL: Channel<CriticalSectionRawMutex, WifiAction, 4> = Channel::new();

/// Requests a display update with the specified action
pub async fn retrieve_data(display_action: WifiAction) {
    DATA_REQUEST_CHANNEL.send(display_action).await;
}

/// Blocks until next update request, returns the requested display action
async fn wait() -> WifiAction {
    DATA_REQUEST_CHANNEL.receive().await
}

#[embassy_executor::task]
async fn wifi_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(runner: &'static mut Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn start(r: WifiResources, spawner: Spawner) {
    debug!("Initializing wifi");

    // Configure PIO and CYW43
    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");
    let pwr = Output::new(r.pwr, Level::Low);
    let cs = Output::new(r.cs, Level::High);

    {
        let mut pio = Pio::new(r.pio, Irqs);
        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            cs,
            r.dio,
            r.clk,
            r.dma,
        );

        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());

        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
        unwrap!(spawner.spawn(wifi_task(runner)));
        debug!("WiFi task started");

        control.init(clm).await;
        control
            .set_power_management(cyw43::PowerManagementMode::Performance)
            .await;

        let mut rand = RngWrapper::new();
        let wifi_ssid = env!("WIFI_SSID");
        let wifi_password = env!("WIFI_PASSWORD");
        let client_name: &str = "surfboard";

        let mut dhcp_config = DhcpConfig::default();
        dhcp_config.hostname = Some(heapless::String::from_str(client_name).unwrap());
        let net_config = NetConfig::dhcpv4(dhcp_config);

        static STACK: StaticCell<Stack<'static>> = StaticCell::new();

        static RUNNER: StaticCell<Runner<'static, cyw43::NetDriver<'static>>> = StaticCell::new();

        // Increase this if you start getting socket ring errors.
        static RESOURCES: StaticCell<StackResources<15>> = StaticCell::new();
        let (s, r) = new_stack(
            net_device,
            net_config,
            RESOURCES.init(StackResources::<15>::new()),
            rand.next_u64(),
        );
        let stack = &*STACK.init(s);

        let runner = &mut *RUNNER.init(r);
        let mac_addr = stack.hardware_address();
        debug!("Hardware configured. MAC Address is {}", mac_addr);

        // Start networking services thread
        unwrap!(spawner.spawn(net_task(runner)));

        // join Wifi
        info!("Joining Wifi {}", wifi_ssid);
        control
            .join(wifi_ssid, JoinOptions::new(wifi_password.as_bytes()))
            .await
            .unwrap();

        let start = Instant::now().as_millis();
        loop {
            let elapsed = Instant::now().as_millis() - start;
            if elapsed > 30_000 {
                core::panic!("Couldn't get network up after 30 seconds");
            } else if stack.is_config_up() {
                info!("Network stack config completed after about {} ms", elapsed);
                break;
            } else {
                Timer::after_millis(10).await;
            }
        }

        match stack.config_v4() {
            Some(a) => {
                info!("IP Address appears to be: {}", a.address);
                send_event(Events::WifiConnected(a.address.address())).await;
            }
            None => {
                send_error("DHCP error").await;
            }
        }
        debug!("Wifi setup!");

        // handle network actions
        loop {
            control
                .set_power_management(cyw43::PowerManagementMode::SuperSave)
                .await;

            let data_retrieval_action = wait().await;
            control.gpio_set(0, true).await;

            match data_retrieval_action {
                WifiAction::LoadConfiguration => {
                    control
                        .set_power_management(cyw43::PowerManagementMode::Performance)
                        .await;
                    #[cfg(feature = "fake_responses")]
                    let http_provider = FakeHttpClient::default();

                    #[cfg(not(feature = "fake_responses"))]
                    let http_provider = HttpClientProvider::new(*stack);

                    let config: Configuration = http_provider
                        .get_as_json("https://yurig-public.s3.us-east-1.amazonaws.com/config.json")
                        .await
                        .unwrap();

                    {
                        let mut state_guard = STATE_MANAGER_MUTEX.lock().await;
                        state_guard.config = Some(config);
                    }
                    send_event(Events::ConfigurationLoaded).await;
                }
                WifiAction::LoadScreen(screen_idx, screen_configuration) => {
                    control
                        .set_power_management(cyw43::PowerManagementMode::Performance)
                        .await;

                    #[cfg(feature = "fake_responses")]
                    let http_provider = FakeHttpClient::default();

                    #[cfg(not(feature = "fake_responses"))]
                    let http_provider = HttpClientProvider::new(*stack);

                    let mut state_guard = STATE_MANAGER_MUTEX.lock().await;

                    // get mutable buffer to store screen data into
                    let buffer = state_guard.get_mut_buffer_for_screen(screen_idx).unwrap();

                    match http_provider.get(screen_configuration.url.as_str(), buffer).await {
                        Some(_) => {
                            send_event(Events::ScreenLoaded(screen_idx)).await;
                        }
                        None => {
                            send_error("Failed to fetch screen").await;
                        }
                    }
                }
                WifiAction::PowerOffWifi => {
                    break;
                }
            }
            control.gpio_set(0, false).await;
        }
    }
    send_event(Events::WifiOff).await;
}
