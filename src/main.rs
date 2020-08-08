#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::{entry,tim6,iprint,iprintln};

#[entry]
fn main() -> ! {
    let (gpioa, rcc, tim6) = aux8::init();
    let mut itm = aux8::ITM_init();
    let mut data_Log: u16 = 0;

    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit()); // make timer6 enable 

    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit()); // set timer6 as one pulse mode and clear conter register so it wont starts

    tim6.psc.write(|w| w.psc().bits(7_999)); // set prescaler 7999, it means that after 1000 tics, timer will increment

    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit()); // input output port A enable in rcc register

    gpioa.moder.modify(|_, w| {
        w.moder1().output(); // configure portA pin1 as output
        w.moder0().input() // configure portA pin0 as input
    });
    

    loop {
        data_Log = data_Log + 1;
        if gpioa.idr.read().idr0().bit_is_set() { // if portA pin 0 is high
            gpioa.odr.write(|w| {
                w.odr1().set_bit() // write portA pin 1 high
            });
            iprintln!(&mut itm.stim[0], "\r\nLog # {}, Environment state danger",data_Log); // print Alarming situation
        }else {
            gpioa.odr.write(|w| {
                w.odr1().clear_bit() // write portA pin 0 low
            });
            iprintln!(&mut itm.stim[0], "\r\nLog # {}, Environment state safe",data_Log); // print Alarming situation
        }
        
        delay(tim6,2000); // delay function calling at 2000ms delay

    }

    
}


fn delay(tim6: &tim6::RegisterBlock, ms: u16) { 
  
    tim6.arr.write(|w| w.arr().bits(ms)); // load register with incoming ms value
    
    tim6.cr1.modify(|_, w| w.cen().set_bit()); // enable counter register
    
    while !tim6.sr.read().uif().bit_is_set() {} // wait while update inteerupt flag is not equal to 1

    tim6.sr.modify(|_, w| w.uif().clear_bit()); // update interrupt flag clear
}
