#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_time::Timer;
use semihosting::process;
use {defmt_rtt as _, panic_probe as _};

const GREETING: &[u8] = b"hello from NUCLEO-F401RE over USART2\r\n";

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // USART2 is wired to the ST-LINK virtual COM port: PA2 (TX), PA3 (RX).
    let mut usart = Uart::new_blocking(p.USART2, p.PA3, p.PA2, Config::default()).unwrap();
    info!("writing greeting over USART2");

    usart.blocking_write(GREETING).unwrap();
    usart.blocking_flush().unwrap();
    Timer::after_millis(100).await;

    info!("done, signalling success");
    process::exit(0);
}
