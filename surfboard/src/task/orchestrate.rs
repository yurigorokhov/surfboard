use defmt::{debug, error};
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
        let event = wait().await;
        process_event(event).await;
    }
}

async fn process_event(event: Events) {
    match event {
        Events::WifiConnected(addr) => {
            retrieve_data(DataRetrievalAction::TideChart).await;
        }
        Events::WifiDhcpError => {
            error!("Event: WifiDhcpError");
            let mut txt: String<30> = String::new();
            let _ = write!(txt, "DHCP error");
            display_update(DisplayAction::ShowStatusText(txt)).await;
        }
        Events::TideChartDataRetrieved => {
            debug!("Received tide predictions!");
            display_update(DisplayAction::DisplaySurfReport).await;
        }
    }
}
