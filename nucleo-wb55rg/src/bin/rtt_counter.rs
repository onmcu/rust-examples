#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
use semihosting::process;
use {defmt_rtt as _, panic_probe as _};

const COUNT_TO: u32 = 10;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());
    info!("counting up to {=u32} over RTT", COUNT_TO);

    for n in 0..=COUNT_TO {
        info!("count = {=u32}", n);
        Timer::after_millis(500).await;
    }

    info!("done, signalling success");
    process::exit(0);
}
