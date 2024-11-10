#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
    }
}
