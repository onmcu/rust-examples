#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use semihosting::process;
// PAC pulled in for its `rt` interrupt vector table and future peripheral use.
use cc13x2x7_cc26x2x7_pac as _;
use {defmt_rtt as _, panic_probe as _};

const COUNT_TO: u32 = 10;
// Default core clock after reset is the 48 MHz RCOSC; ~0.5 s per tick.
const CYCLES_PER_HALF_SEC: u32 = 48_000_000 / 2;

#[entry]
fn main() -> ! {
    info!("counting up to {=u32} over RTT", COUNT_TO);

    for n in 0..=COUNT_TO {
        info!("count = {=u32}", n);
        cortex_m::asm::delay(CYCLES_PER_HALF_SEC);
    }

    info!("done, signalling success");
    process::exit(0);
}
