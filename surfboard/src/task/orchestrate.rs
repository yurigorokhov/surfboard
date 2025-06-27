use defmt::{debug, error};
use embassy_futures::select::select;
use embassy_time::Timer;
use heapless::String;
use surfboard_lib::{data::DataRetrievalAction, draw::DisplayAction};

use crate::{
    system::event::{wait, Events},
    task::{display::display_update, wifi::retrieve_data},
};
use core::fmt::Write;

/// Main coordination task that implements the system's event loop
#[embassy_executor::task]
pub async fn start() {
    loop {
        let event_future = wait();
        let timeout = Timer::after_secs(3600);
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

async fn process_event(event: Events) {
    match event {
        Events::OrchestratorTimeout => {
            // update surf report on timeout
            retrieve_data(DataRetrievalAction::SurfReport).await;
        }
        Events::WifiConnected(_addr) => {
            retrieve_data(DataRetrievalAction::SurfReport).await;
        }
        Events::WifiDhcpError => {
            error!("Event: WifiDhcpError");
            let mut txt: String<30> = String::new();
            let _ = write!(txt, "DHCP error");
            display_update(DisplayAction::ShowStatusText(txt)).await;
        }
        Events::SurfReportRetrieved => {
            debug!("Received tide predictions!");
            display_update(DisplayAction::DisplaySurfReport).await;
        }
    }
}
