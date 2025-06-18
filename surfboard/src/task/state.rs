use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use surfboard_lib::data::ProgramState;

pub static STATE_MANAGER_MUTEX: Mutex<CriticalSectionRawMutex, ProgramState> =
    Mutex::new(ProgramState { surf_report: None });
