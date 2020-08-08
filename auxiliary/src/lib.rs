
//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::stm32f30x::{gpioa,rcc,tim6},
};

use f3::hal::stm32f30x::{self, GPIOA, RCC, TIM6};

pub fn init() -> (&'static gpioa::RegisterBlock, &'static rcc::RegisterBlock, &'static tim6::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOA::ptr(), &*RCC::ptr(), &*TIM6::ptr()) }
}

pub fn ITM_init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}