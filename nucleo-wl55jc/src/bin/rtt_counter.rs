#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::SharedData;
use embassy_time::Timer;
use semihosting::process;
use {defmt_rtt as _, panic_probe as _};

const COUNT_TO: u32 = 10;

// STM32WL55 is dual-core; the CM4 application core initialises the HAL as the
// primary core and publishes the result through this shared region.
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init_primary(Default::default(), &SHARED_DATA);
    info!("counting up to {=u32} over RTT", COUNT_TO);

    for n in 0..=COUNT_TO {
        info!("count = {=u32}", n);
        Timer::after_millis(500).await;
    }

    info!("done, signalling success");
    process::exit(0);
}
