use core::net::Ipv4Addr;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use heapless::String;

use crate::task::state::ScreenConfiguration;

/// Multi-producer, single-consumer event channel
///
/// Capacity of 10 events provides good balance between:
/// - Memory usage
/// - Event processing latency
/// - System responsiveness
pub static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Events, 5> = Channel::new();

/// Sends an event to the system channel
///
/// Events are queued if channel is full. If multiple events
/// occur simultaneously, they are processed in order of arrival.
pub async fn send_event(event: Events) {
    EVENT_CHANNEL.sender().send(event).await;
}

pub async fn send_error(error: &str) {
    send_event(Events::Error(String::try_from(error).expect("Failed to report error"))).await;
}

/// Receives the next event from the system channel
///
/// Called by the orchestrator task to process events sequentially.
/// Waits asynchronously if no events are available.
pub async fn wait() -> Events {
    EVENT_CHANNEL.receiver().receive().await
}

/// System-wide events that can occur during robot operation
#[derive(Debug, Clone)]
pub enum Events {
    WifiConnected(Ipv4Addr),
    Error(String<30>),
    WifiOff,
    DisplayOff,
    ConfigurationLoaded,
    ScreenDrawn(usize),
    ScreenLoaded(usize),
    OrchestratorTimeout,
    PowerButtonPressed,
}

#[derive(Debug, Clone)]
pub enum WifiCommand {
    LoadConfiguration,
    LoadScreen(usize, ScreenConfiguration),
    PowerOffWifi,
}
