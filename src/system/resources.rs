//! Hardware Resource Management
//!
//! Manages and allocates hardware resources (pins, peripherals) to different system components.
//! This module ensures safe and organized access to the robot's hardware by:
//! - Defining clear ownership of hardware resources
//! - Preventing conflicts in hardware access
//! - Providing safe concurrent access to shared resources
//! source: https://github.com/1-rafael-1/simple-robot/blob/07c7618a40001ea0aaf8450438b71135c96af016/src/system/resources.rs#L1

use assign_resources::assign_resources;
use embassy_rp::peripherals;
use embassy_rp::{bind_interrupts, peripherals::*, pio::InterruptHandler as PioInterruptHandler};

assign_resources! {
    screen: ScreenResources {
        spi: SPI0,
        mosi: PIN_3,
        clk: PIN_6,
        command_selection_pin: PIN_5,
        display_power_pin: PIN_12,
        display_busy_pin: PIN_13,
        display_data_command_pin: PIN_14,
        display_reset_pin: PIN_11,
    },
    wifi: WifiResources {
        pio: PIO0,
        pwr: PIN_23,
        cs: PIN_25,
        dio: PIN_24,
        clk: PIN_29,
        dma: DMA_CH0,
    }
}

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});
