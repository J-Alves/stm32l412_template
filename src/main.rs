#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32l4::stm32l412;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32l412::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());

    loop {
        // your code goes here
    }
}
