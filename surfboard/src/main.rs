#![no_std]
#![no_main]

use defmt::*;
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

mod random;
use self::random::RngWrapper;

mod system;
use system::resources::*;

mod task;
use embassy_executor::Executor;
use embassy_rp::multicore::{Stack, spawn_core1};
use task::display;
use task::orchestrate;
use task::wifi;

mod http;
use self::http::Client as HttpClient;
use self::http::ClientTrait;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Program start");
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    // run display on separate thread
    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(display::start(r.screen))));
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.spawn(orchestrate::start()).unwrap();
        spawner.spawn(wifi::start(r.wifi, spawner)).unwrap();
    });
}
