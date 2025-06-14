//! Event System
//!
//! Provides a centralized event handling system for inter-task communication.
//! Uses an async channel to coordinate events between different parts of the system.
//!
//! # Event Flow
//! 1. Tasks generate events (e.g., sensor readings, button presses)
//! 2. Events are sent through the channel
//! 3. The orchestrator task processes events and updates system state
//! 4. State changes trigger corresponding actions in other tasks
//!
//! # Channel Design
//! - Multi-producer: Any task can send events
//! - Single-consumer: Orchestrator task processes all events
//! - Bounded capacity: 10 events maximum to prevent memory exhaustion
//! - Async operation: Non-blocking event handling
//!
//! # Usage Example
//! ```rust
//! // Sending an event
//! event::send(Events::ButtonPressed(ButtonId::A)).await;
//!
//! // Receiving an event (in orchestrator)
//! let event = event::wait().await;
//! ```

use core::net::Ipv4Addr;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use surfboard_lib::data::TidePredictions;

/// Multi-producer, single-consumer event channel
///
/// Capacity of 10 events provides good balance between:
/// - Memory usage
/// - Event processing latency
/// - System responsiveness
pub static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Events, 10> = Channel::new();

/// Sends an event to the system channel
///
/// Events are queued if channel is full. If multiple events
/// occur simultaneously, they are processed in order of arrival.
pub async fn send_event(event: Events) {
    EVENT_CHANNEL.sender().send(event).await;
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
    WifiDhcpError,
    // TideChartDataRetrieved(TidePredictions),
}
