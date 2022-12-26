#![no_std]
#![no_main]


use cortex_m_rt::{exception, entry};
// use panic_halt as _;
use panic_semihosting as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use cortex_m_semihosting::{hprintln, hprint};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let _cp = pac::CorePeripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let _gpioa = p.GPIOA.split(&mut rcc.ahb);
    let _gpiob = p.GPIOB.split(&mut rcc.ahb);
    let _gpioc = p.GPIOC.split(&mut rcc.ahb);
    let _gpiod = p.GPIOD.split(&mut rcc.ahb);
    let _gpioe = p.GPIOE.split(&mut rcc.ahb);
    loop{}
}
