use defmt::{debug, error, info};
use embassy_futures::select::select;
use embassy_time::Timer;
use heapless::String;

use crate::{
    system::{
        drawing::DisplayAction,
        event::{wait, Events, WifiAction},
    },
    task::{
        display::display_update,
        power::POWER_DOWN_SIGNAL,
        state::{self, ScreenConfiguration, STATE_MANAGER_MUTEX},
        wifi::retrieve_data,
    },
};
use core::fmt::Write;

/// Main coordination task that implements the system's event loop
#[embassy_executor::task]
pub async fn start() {
    loop {
        let event_future = wait();
        let timeout = Timer::after_secs(60);
        match select(event_future, timeout).await {
            embassy_futures::select::Either::First(event) => {
                process_event(event).await;
            }
            embassy_futures::select::Either::Second(_) => {
                process_event(Events::OrchestratorTimeout).await;
            }
        }
    }
}

async fn process_event<'a>(event: Events) {
    match event {
        Events::OrchestratorTimeout => {
            // tell wifi to power off
            retrieve_data(WifiAction::PowerOffWifi).await;
        }
        Events::WifiOff => POWER_DOWN_SIGNAL.signal(()),
        Events::WifiConnected(_addr) => {
            retrieve_data(WifiAction::LoadConfiguration).await;
        }
        Events::ConfigurationLoaded => {
            let screen_config: Option<ScreenConfiguration>;
            {
                let state_guard = STATE_MANAGER_MUTEX.lock().await;
                screen_config = Some(state_guard.config.as_ref().unwrap().screens.first().unwrap().clone());
            }
            retrieve_data(WifiAction::LoadScreen(screen_config.unwrap())).await;
        }
        Events::WifiDhcpError => {
            error!("Event: WifiDhcpError");
            let mut txt: String<30> = String::new();
            let _ = write!(txt, "DHCP error");
            display_update(DisplayAction::ShowStatusText(txt)).await;
        }
        Events::ScreenUpdateReceived => {
            display_update(DisplayAction::DrawImage).await;
        }
        Events::PowerButtonPressed => {
            let screen_config: Option<ScreenConfiguration>;
            {
                let state_guard = STATE_MANAGER_MUTEX.lock().await;
                screen_config = Some(state_guard.config.as_ref().unwrap().screens.first().unwrap().clone());
            }
            retrieve_data(WifiAction::LoadScreen(screen_config.unwrap())).await;
        }
    }
}
