extern crate byteorder;
extern crate hidapi;
#[macro_use]
extern crate lazy_static;
extern crate common;
extern crate signal_hook;
extern crate termion;

mod axis;
mod button;
mod device;
mod driver;
mod frame;
mod id;
mod input;
mod output;

use std::time::Instant;

use common::log;
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use device::InputMode;
use driver::Driver;
use id::Product;

const PENDING_LEDS: [u8; 6] = [0b0011, 0b0101, 0b1010, 0b1100, 0b1010, 0b0101];

fn main() {
    let signals = match Signals::new(&[SIGINT, SIGTERM]) {
        Ok(signals) => signals,
        Err(e) => panic!(e),
    };

    let mut drivers = <Vec<Driver>>::with_capacity(2);

    match Driver::find(Product::JoyConL) {
        Ok(jc) => drivers.push(jc),
        Err(e) => log::e(e),
    }
    match Driver::find(Product::JoyConR) {
        Ok(jc) => drivers.push(jc),
        Err(e) => log::e(e),
    }

    // Print some basic device identity info
    for jc in drivers.iter_mut() {
        println!("Connected to {}", jc);
        if let Err(e) = jc.set_input_mode(InputMode::Full) {
            log::e(e);
        }
    }

    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    'main: loop {
        let led_index = (start_time.elapsed().subsec_nanos() / (1_000_000_000 / 6)) as usize;
        for jc in drivers.iter() {
            if let Err(e) = jc.set_leds(PENDING_LEDS[led_index]) {
                log::e(e);
            }
        }
        for jc in drivers.iter_mut() {
            if let Err(e) = jc.flush() {
                log::e(e);
            }
        }
        for signal in signals.pending() {
            match signal {
                SIGINT | SIGTERM => {
                    for jc in drivers.iter() {
                        if let Err(e) = jc.reset() {
                            println!("{}", e);
                        }
                    }
                    break 'main;
                }
                _ => unreachable!(),
            }
        }
    }
}
