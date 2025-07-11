use embassy_futures::select::select;
use embassy_time::Timer;

use crate::{
    system::{
        drawing::DisplayCommand,
        event::{send_event, wait, Events, WifiCommand},
    },
    task::{
        display::display_command,
        power::POWER_DOWN_SIGNAL,
        state::{SCREEN_SAVER_SCREEN_IDX, STATE_MANAGER_MUTEX},
        wifi::wifi_command,
    },
};

/// Main coordination task that implements the system's event loop
#[embassy_executor::task]
pub async fn start() {
    loop {
        let event_future = wait();
        let timeout = Timer::after_secs(120);
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
        // shutdown sequence
        Events::OrchestratorTimeout => {
            // fetch screensaver
            let mut state_guard = STATE_MANAGER_MUTEX.lock().await;
            state_guard.screen_index = SCREEN_SAVER_SCREEN_IDX;
            if let Some(config) = &state_guard.config {
                if let Some(screen_saver) = &config.screen_saver {
                    wifi_command(WifiCommand::LoadScreen(SCREEN_SAVER_SCREEN_IDX, screen_saver.clone())).await;
                } else {
                    wifi_command(WifiCommand::PowerOffWifi).await
                }
            } else {
                wifi_command(WifiCommand::PowerOffWifi).await
            }
        }
        Events::ScreenDrawn(screen_idx) => {
            // if the screen-saver was drawn then we are part of the shutdown cycle
            if screen_idx == SCREEN_SAVER_SCREEN_IDX {
                wifi_command(WifiCommand::PowerOffWifi).await
            }
        }
        Events::WifiOff => display_command(DisplayCommand::DisplayPowerOff).await,
        Events::DisplayOff => POWER_DOWN_SIGNAL.signal(()),

        // startup
        Events::WifiConnected(_addr) => wifi_command(WifiCommand::LoadConfiguration).await,
        Events::ConfigurationLoaded => {
            let state_guard = STATE_MANAGER_MUTEX.lock().await;

            // if we are in shutdown cycle, we don't handle configuration changes
            if state_guard.screen_index != SCREEN_SAVER_SCREEN_IDX {
                // check if the screen being requested has already been loaded
                let screen_index = state_guard.screen_index;
                if let Some(_) = state_guard.get_buffer_for_screen(screen_index) {
                    send_event(Events::ScreenLoaded(screen_index)).await;
                } else if let Some(config) = &state_guard.config {
                    let screen_config = config.screens.iter().nth(state_guard.screen_index).unwrap().clone();
                    wifi_command(WifiCommand::LoadScreen(state_guard.screen_index, screen_config)).await;
                }

                // pre-fetch the next screen
                if let Some(next_screen_index) = state_guard.next_screen_idx() {
                    if let Some(config) = &state_guard.config {
                        if state_guard.get_buffer_for_screen(next_screen_index).is_none() {
                            let screen_config = config.screens.iter().nth(next_screen_index).unwrap().clone();
                            wifi_command(WifiCommand::LoadScreen(next_screen_index, screen_config)).await;
                        }
                    }
                }
            }
        }
        Events::Error(msg) => display_command(DisplayCommand::ShowStatusText(msg, 1)).await,
        Events::ScreenLoaded(screen_idx) => {
            let state_guard = STATE_MANAGER_MUTEX.lock().await;
            if state_guard.screen_index == screen_idx {
                display_command(DisplayCommand::DrawImage(screen_idx)).await;
            }
        }
        Events::PowerButtonPressed => {
            // switch to next screen
            let mut state_guard = STATE_MANAGER_MUTEX.lock().await;
            if state_guard.screen_index != SCREEN_SAVER_SCREEN_IDX {
                state_guard.move_to_next_screen();
                send_event(Events::ConfigurationLoaded).await;
            }
        }
    }
}
