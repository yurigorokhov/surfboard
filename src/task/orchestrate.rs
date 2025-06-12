use defmt::{error, info};
use heapless::String;

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
            display_update(crate::draw::DisplayAction::ShowStatusText(txt)).await;
        }
        Events::WifiError => {
            error!("Event: Wifi did not connect")
        }
    }
}
