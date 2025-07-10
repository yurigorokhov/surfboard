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
    pub url: String<128>,
}

const SERVER_SIDE_IMAGE_BYTES: usize = 1024 * 24;

#[derive(Default)]
pub struct ProgramState {
    pub config: Option<Configuration>,
    pub screen_index: usize,
    buffer: Vec<u8, SERVER_SIDE_IMAGE_BYTES>,
}

impl ProgramState {
    pub fn next_screen(&mut self) {
        match &self.config {
            Some(config) => {
                if self.screen_index >= config.screens.len() - 1 {
                    self.screen_index = 0;
                } else {
                    self.screen_index += 1;
                }
            }
            None => {
                self.screen_index = 0;
            }
        }
    }

    pub fn get_mut_buffer(&mut self) -> &mut Vec<u8, SERVER_SIDE_IMAGE_BYTES> {
        &mut self.buffer
    }

    pub fn get_buffer(&self) -> &Vec<u8, SERVER_SIDE_IMAGE_BYTES> {
        &self.buffer
    }
}

pub static STATE_MANAGER_MUTEX: Mutex<CriticalSectionRawMutex, ProgramState> = Mutex::new(ProgramState {
    config: None,
    screen_index: 0,
    buffer: Vec::new(),
});
