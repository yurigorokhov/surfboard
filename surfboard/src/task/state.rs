use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::{LinearMap, String, Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub screens: Vec<ScreenConfiguration, NUM_SCREEN_CONFIGURATIONS>,
    pub screen_saver: Option<ScreenConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenConfiguration {
    pub url: String<128>,
}

const NUM_SCREEN_CONFIGURATIONS: usize = 5;
const NUM_SCREEN_BUFFERS: usize = 2;
pub const SERVER_SIDE_IMAGE_BYTES: usize = 1024 * 25;
pub const SCREEN_SAVER_SCREEN_IDX: usize = 127;

#[derive(Default)]
pub struct ProgramState {
    pub config: Option<Configuration>,
    pub screen_index: usize,

    // buffers for storing screens to be displayed
    buffers: [Vec<u8, SERVER_SIDE_IMAGE_BYTES>; NUM_SCREEN_BUFFERS],

    // mapping of screen_idx to buffer_idx
    pub buffer_map: LinearMap<usize, usize, NUM_SCREEN_BUFFERS>,

    // vector of buffers that can be forgotten
    pub can_forget: Vec<usize, NUM_SCREEN_BUFFERS>,
}

impl ProgramState {
    pub fn next_screen_idx(&self) -> Option<usize> {
        if let Some(config) = &self.config {
            Some((self.screen_index + 1) % &config.screens.len())
        } else {
            None
        }
    }

    pub fn move_to_next_screen(&mut self) {
        if let Some(idx) = self.next_screen_idx() {
            self.screen_index = idx;
        } else {
            self.screen_index = 0;
        }
    }

    pub fn forget_screen_idx(&mut self, screen_idx: usize) {
        if !self.can_forget.contains(&screen_idx) {
            self.can_forget.push(screen_idx).unwrap();
        }
    }

    fn find_empty_buffer(&mut self) -> Option<usize> {
        for i in 0..NUM_SCREEN_BUFFERS {
            let mut available = true;
            for j in self.buffer_map.values() {
                if i == *j {
                    available = false;
                    break;
                }
            }
            if available {
                return Some(i);
            }
        }

        // check any buffers that can reclaim!
        if let Some(screen_idx) = self.can_forget.pop() {
            if let Some(buffer_idx) = self.buffer_map.remove(&screen_idx) {
                self.buffers[buffer_idx].clear();
                return Some(buffer_idx);
            }
        }
        None
    }

    pub fn get_mut_buffer_for_screen(&mut self, screen_idx: usize) -> Option<&mut Vec<u8, SERVER_SIDE_IMAGE_BYTES>> {
        let empty_buffer_idx = self.find_empty_buffer()?;
        self.buffer_map.insert(screen_idx, empty_buffer_idx).unwrap();
        self.buffers[empty_buffer_idx].clear();
        Some(&mut self.buffers[empty_buffer_idx])
    }

    pub fn get_buffer_for_screen(&self, screen_idx: usize) -> Option<&Vec<u8, SERVER_SIDE_IMAGE_BYTES>> {
        let buffer_idx = *self.buffer_map.get(&screen_idx)?;
        Some(&self.buffers[buffer_idx])
    }
}

pub static STATE_MANAGER_MUTEX: Mutex<CriticalSectionRawMutex, ProgramState> = Mutex::new(ProgramState {
    config: None,
    screen_index: 0,
    buffers: [Vec::new(), Vec::new()],
    buffer_map: LinearMap::new(),
    can_forget: Vec::new(),
});
