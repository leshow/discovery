#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let half_period = 50_u16;

    let n = leds.len();
    for i in (0..n).into_iter().cycle() {
        leds[(i+1) % n].on();
        delay.delay_ms(half_period);
        leds[i].off();
        delay.delay_ms(half_period);

    }
    loop {}
}