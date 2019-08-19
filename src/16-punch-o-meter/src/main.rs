#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux16::{entry, iprint, iprintln, prelude::*, I16x3, Instant, Sensitivity};

#[entry]
fn main() -> ! {
    let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = aux16::init();

    // extend sensing range to `[-12g, +12g]`
    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();
    loop {
        const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

        let I16x3 { x, .. } = lsm303dlhc.accel().unwrap();

        let x = f32::from(x) * SENSITIVITY;

        if x >= 0.5 {
            iprintln!(&mut itm.stim[0], "start testing {:?}", x);
            let mut max = 0.;
            let start = mono_timer.now();
            loop {
                let x_ = lsm303dlhc.accel().unwrap().x.abs();
                let newx = f32::from(x_) * SENSITIVITY;
                if newx > max {
                    max = newx;
                }
                if start.elapsed() >= 50 {
                    iprintln!(&mut itm.stim[0], "max acceleration {:?}", max);
                    break;
                }
            }
        }

        delay.delay_ms(100_u16);
    }
}
