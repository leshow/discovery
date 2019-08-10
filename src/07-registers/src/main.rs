#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // bs for bit set
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // br for bit reset
    gpioe.bsrr.write(|w| w.br9().set_bit());

    gpioe.bsrr.write(|w| w.br11().set_bit());
    loop {}
}
