#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

mod random;
use self::random::RngWrapper;

mod system;
use system::resources::*;

mod task;
use task::display;
use task::wifi;

mod draw;

mod http;
use self::http::Client as HttpClient;
use self::http::ClientTrait;

// #[cortex_m_rt::pre_init]
// unsafe fn before_main() {
//     // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
//     // before we hit main to avoid deadlocks when using a debugger
//     embassy_rp::pac::SIO.spinlock(31).write_value(1);
// }

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    spawner.spawn(display::start(r.screen)).unwrap();
    spawner.spawn(wifi::start(r.wifi, spawner)).unwrap();

    loop {
        debug!("Main Loop");
        Timer::after(Duration::from_secs(10)).await;
    }
}
