use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

const NUM_SCREEN_CONFIGURATIONS: usize = 5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub screens: Vec<ScreenConfiguration, NUM_SCREEN_CONFIGURATIONS>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenConfiguration {
    pub key: String<32>,
    pub url: String<128>,
}

const SERVER_SIDE_IMAGE_BYTES: usize = 1024 * 24;

#[derive(Default)]
pub struct ProgramState {
    pub config: Option<Configuration>,
    pub server_side_image: Vec<u8, SERVER_SIDE_IMAGE_BYTES>,
}

pub static STATE_MANAGER_MUTEX: Mutex<CriticalSectionRawMutex, ProgramState> = Mutex::new(ProgramState {
    config: None,
    server_side_image: Vec::new(),
});
