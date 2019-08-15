#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, prelude::*, Direction, I16x3};
use core::f32::consts::PI;
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    // pointing north: I16x3 { x: -187, y: -195, z: -534 }
    // rotate 90 I16x3 { x: 89, y: -243, z: -601 }
    // rotate 90 again I16x3 { x: 78, y: 71, z: -631 }
    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = f32::from(y).atan2(f32::from(x)); // radians

        let dir = if theta < -PI / 1.125 {
            Direction::North
        } else if theta < -PI / 1.5 {
            Direction::Northwest
        } else if theta < -PI / 3. {
            Direction::West
        } else if theta < -PI / 9. {
            Direction::Southwest
        } else if theta < PI / 9. {
            Direction::South
        } else if theta < PI / 3. {
            Direction::Southeast
        } else if theta < PI / 1.5 {
            Direction::East
        } else if theta < PI / 1.125 {
            Direction::Northeast
        } else {
            Direction::North
        };

        // iprintln!(&mut itm.stim[0], "{:?}", theta);
        // let dir = match (x > 0, y > 0) {
        //     (true, true) => Direction::Southeast,
        //     (true, false) => Direction::Southwest,
        //     (false, false) => Direction::Northwest,
        //     (false, true) => Direction::Northeast,
        // };
        for led in leds.iter_mut() {
            led.off();
        }
        leds[dir].on();
        // iprintln!(&mut itm.stim[0], "{:?}", (x, y));
        delay.delay_ms(100_u16);
    }
}
