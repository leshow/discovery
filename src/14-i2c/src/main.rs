#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    loop {
        // Stage 1: Send the address of the register we want to read to the
        // magnetometer
        i2c1.cr2.write(|w| {
            // TODO Broadcast START
            w.start().set_bit();
            // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Write
            w.sadd1().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}
        // TODO Send the address of the register that we want to read: OUT_X_H_M
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));

        while i2c1.isr.read().tc().bit_is_clear() {}

        // Stage 2: Receive the contents of the register we asked for
        // TODO Broadcast RESTART
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Read
            w.sadd1().bits(MAGNETOMETER);
            w.rd_wrn().set_bit();
            w.nbytes().bits(6);
            w.autoend().set_bit()
        });
        let mut buf = [0u8; 6];
        for byte in &mut buf {
            // TODO Receive the contents of the register
            while i2c1.isr.read().rxne().bit_is_clear() {}
            // TODO Broadcast STOP
            *byte = i2c1.rxdr.read().rxdata().bits()
        }
        let x_h = u16::from(buf[0]);
        let x_l = u16::from(buf[1]);
        let z_h = u16::from(buf[2]);
        let z_l = u16::from(buf[3]);
        let y_h = u16::from(buf[4]);
        let y_l = u16::from(buf[5]);

        let x = ((x_h << 8) + x_l) as i16;
        let z = ((z_h << 8) + z_l) as i16;
        let y = ((y_h << 8) + y_l) as i16;

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
        delay.delay_ms(1_000_u16);
    }
}
