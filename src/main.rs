#![no_std]
#![no_main]

// pick a panicking behavior
use cortex_m_rt::{exception, entry};

use panic_semihosting as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f3xx_hal::{self as hal, pac as pac, prelude::*};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let mut  cp = pac::CorePeripherals::take().unwrap();
    loop{}
}
