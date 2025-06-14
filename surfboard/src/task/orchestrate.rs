use defmt::{error, info};
use heapless::String;
use surfboard_lib::draw::DisplayAction;

use crate::{
    system::event::{send_event, wait, Events},
    task::display::display_update,
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
            let mut txt: String<20> = String::new();
            let _ = write!(txt, "IP: {}", addr);
            display_update(DisplayAction::ShowStatusText(txt)).await;
        }
        Events::WifiDhcpError => {
            error!("Event: WifiDhcpError");
            let mut txt: String<20> = String::new();
            let _ = write!(txt, "DHCP error");
            display_update(DisplayAction::ShowStatusText(txt)).await;
        }
    }
}
