use embassy_futures::select::select;
use embassy_time::Timer;

use crate::{
    system::{
        drawing::DisplayAction,
        event::{send_error, wait, Events, WifiAction},
    },
    task::{display::display_update, power::POWER_DOWN_SIGNAL, state::STATE_MANAGER_MUTEX, wifi::retrieve_data},
};

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
        // shutdown
        Events::OrchestratorTimeout => {
            retrieve_data(WifiAction::PowerOffWifi).await;
        }
        Events::WifiOff => POWER_DOWN_SIGNAL.signal(()),

        // startup
        Events::WifiConnected(_addr) => {
            retrieve_data(WifiAction::LoadConfiguration).await;
        }
        Events::ConfigurationLoaded => {
            let state_guard = STATE_MANAGER_MUTEX.lock().await;
            match state_guard.config.as_ref() {
                Some(config) => {
                    let screen_config = config.screens.first().unwrap().clone();
                    retrieve_data(WifiAction::LoadScreen(screen_config)).await;
                }
                None => {}
            }
        }
        Events::Error(msg) => {
            display_update(DisplayAction::ShowStatusText(msg)).await;
        }
        Events::ScreenUpdateReceived => {
            display_update(DisplayAction::DrawImage).await;
        }
        Events::PowerButtonPressed => {
            let state_guard = STATE_MANAGER_MUTEX.lock().await;
            match state_guard.config.as_ref() {
                Some(config) => {
                    let screen_config = config.screens.first().unwrap().clone();
                    retrieve_data(WifiAction::LoadScreen(screen_config)).await;
                }
                None => {
                    retrieve_data(WifiAction::LoadConfiguration).await;
                }
            }
        }
    }
}
