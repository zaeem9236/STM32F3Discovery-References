#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioa, rcc) = aux8::init();

    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit()); // input output port A enable in rcc register

    gpioa.moder.modify(|_, w| {
        w.moder1().output(); // configure portA pin1 as output
        w.moder0().input() // configure portA pin0 as input
    });

    loop {
        if gpioa.idr.read().idr0().bit_is_set() {
            // if portA pin 0 is high
            gpioa.odr.write(|w| {
                w.odr1().set_bit() // write portA pin 1 high
            });
        }else {
            gpioa.odr.write(|w| {
                w.odr1().clear_bit() // write portA pin 0 low
            });
        }
    }
}
