#![no_main]
#![no_std]

use aux9::{entry, nop, tim6};

// 8 Mhz is 8_000_000 cycles per second
// so this must delay 1Khz * ms
#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // set timer to go off after 'ms' ticks, 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));
    // CEN: enables counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());
    // busy wait on status register (sr)
    while !tim6.sr.read().uif().bit_is_set() {}
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // power on tim6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    // OPM sets one-pulse-mode
    // CEN disables counter (for now)
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // configer prescaler to have counter run @ 1 KHz
    // APB1_CLOCK = 8 MHz
    // apb / (psc + 1) = 1 KHz, psc = 7999
    // makes the counter increase every millisecond
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
