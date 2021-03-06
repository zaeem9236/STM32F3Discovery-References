
//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::bkpt;
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpiob, rcc}; // this line is accessing modules

use f3::hal::stm32f30x::{self, GPIOB, RCC}; // this line is accessing structs 

pub fn init() -> (&'static gpiob::RegisterBlock, &'static rcc::RegisterBlock) { //modules
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOB::ptr(), &*RCC::ptr()) } // structs
}