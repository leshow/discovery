#![allow(unused_macros)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use core::fmt::{self, Write};
use heapless::{consts, Vec};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => ($serial.write_fmt(format_args!($($arg)*)).ok());
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
        }
        Ok(())
    }
}

struct Echo {
    usart1: &'static mut usart1::RegisterBlock,
}

impl Echo {
    pub fn read_byte(&mut self) -> u8 {
        while self.usart1.isr.read().rxne().bit_is_clear() {}
        self.usart1.rdr.read().rdr().bits() as u8
    }
    pub fn write_byte(&mut self, data: u8) {
        while self.usart1.isr.read().txe().bit_is_clear() {}
        self.usart1.tdr.write(|w| w.tdr().bits(u16::from(data)))
    }
    pub fn write<'a, I: 'a>(&mut self, data: I)
    where
        I: Iterator<Item = &'a u8>,
    {
        for &b in data {
            self.write_byte(b);
        }
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    // send
    // let mut serial = SerialPort { usart1 };
    // uprintln!(serial, "Hello friend {}", 13 + 3);

    // echo server
    let mut echo = Echo { usart1 };
    let mut buf: Vec<u8, consts::U32> = Vec::new();
    'main: loop {
        buf.clear();
        // busy wait on rxne register -- tells us that
        loop {
            let data = echo.read_byte();
            iprintln!(&mut itm.stim[0], "read value {}", data as char);
            if data == b'\r' {
                iprintln!(&mut itm.stim[0], "got newline char");
                buf.reverse();
                break;
            }
            let res = buf.push(data);
            if res.is_err() {
                iprintln!(&mut itm.stim[0], "buffer full");
                continue 'main;
            }
        }
        // send data back to receiver
        echo.write(buf.iter().chain(&[b'\n', b'\r']));
    }

    // Original program before macro:
    // while usart1.isr.read().txe().bit_is_clear() {}
    // usart1.tdr.write(|w| w.tdr().bits(u16::from(b'\n')));
    // let instant = mono_timer.now();

    // // Send a single character
    // let text = "The quick brown fox jumps over the lazy dog.";
    // for &c in text.as_bytes() {
    //     // The status register (ISR) has a flag (TXE) that indicates
    //     // it's safe to write to TDR register
    //     while usart1.isr.read().txe().bit_is_clear() {}
    //     usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
    // }

    // let elapsed = instant.elapsed();

    // iprintln!(
    //     &mut itm.stim[0],
    //     "for loop took {} ticks ({} us)",
    //     elapsed,
    //     elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    // );
    // USART questions
    // Execute your program inside the debugger, statement by statement. What do you see?
    // It's able to write the string properly

    // Then execute the program again but in one go using the continue command. What happens this time?
    // after the first space character the bytes printed are not what I originally sent
    // see: Thequc bon o jms vr helzydg

    // Finally, build the program in release mode and, again, run it one go. What happens this time?
    // It just prints one character "T"

    loop {}
}
