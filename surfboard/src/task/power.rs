use defmt::info;
use embassy_futures::select::select;
use embassy_rp::gpio::Level;
use embassy_rp::peripherals::{PIN_23, WATCHDOG};
use embassy_rp::{
    clocks::dormant_sleep,
    gpio::{DormantWakeConfig, Input, Output, Pull},
    watchdog::Watchdog,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

use critical_section::with;
use embassy_time::Timer;

use crate::system::{
    event::{send_event, Events},
    resources::SleepResources,
};

pub static POWER_DOWN_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::task]
pub async fn start(sleep_resources: SleepResources) {
    let mut power_button = Input::new(sleep_resources.wake_pin, Pull::Up);
    let mut status_led = Output::new(sleep_resources.status_led, embassy_rp::gpio::Level::Low);
    status_led.set_high();
    loop {
        match select(power_button.wait_for_falling_edge(), POWER_DOWN_SIGNAL.wait()).await {
            embassy_futures::select::Either::First(_) => {
                send_event(Events::PowerButtonPressed).await;
                Timer::after_secs(1).await;
            }
            embassy_futures::select::Either::Second(_) => {
                info!("Going to sleep...");
                let wake_config = DormantWakeConfig {
                    edge_high: false,
                    edge_low: true,
                    level_high: false,
                    level_low: false,
                };
                status_led.set_low();

                with(|_cs| {
                    // Disable interrupts or other critical activity here
                    cortex_m::interrupt::disable();
                });

                // kill wifi power to save battery life
                let wifi_pwr = unsafe { PIN_23::steal() };
                Output::new(wifi_pwr, Level::Low).set_low();

                // set up wake on button
                let wake = power_button.dormant_wake(wake_config);
                dormant_sleep();
                core::mem::forget(wake);

                // reset chip on wake
                let w = unsafe { WATCHDOG::steal() };
                let mut watchdog = Watchdog::new(w);
                watchdog.trigger_reset();
            }
        }
    }
}
