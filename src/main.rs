#![deny(unsafe_code)]
#![no_main]
#![no_std]

// this program is used to on an LED on PORTB pin 11 

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // enable the GPIOE peripheral
    rcc.ahbenr.modify(|_, w| w.iopben().set_bit());

    // configure the pins as outputs
    gpioe.moder.modify(|_, w| {
        w.moder11().output()
   
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr11().set_bit();
        
    });

    aux8::bkpt();

    loop {}
}