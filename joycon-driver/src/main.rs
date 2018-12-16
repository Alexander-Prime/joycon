extern crate arraydeque;
extern crate byteorder;
extern crate getopts;
extern crate hidapi;
extern crate signal_hook;
extern crate termion;

extern crate common;

mod axis;
mod button;
mod device;
mod driver;
mod frame;
mod id;
mod input;
mod output;

use std::time::Instant;

// use getopts::Options;
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use common::log;

use device::InputMode;
use driver::Driver;
use id::Product;

const PENDING_LEDS: [u8; 6] = [0b0011, 0b0101, 0b1010, 0b1100, 0b1010, 0b0101];

fn main() {
    let signals = match Signals::new(&[SIGINT, SIGTERM]) {
        Ok(signals) => signals,
        Err(e) => panic!(e),
    };

    let mut driver = match Driver::find(Product::JoyConR) {
        Ok(driver) => driver,
        Err(e) => panic!("{}", e),
    };

    // return;

    // let opts = Options::new();

    // let args: Vec<String> = std::env::args().collect();
    // let serial = match opts.parse(&args[1..]) {
    //     Ok(ref m) if m.free.is_empty() => panic!("Please supply a device serial ID"),
    //     Err(e) => panic!(e),
    //     Ok(m) => m.free[0].clone(),
    // };

    // let mut driver = match Driver::for_serial(&serial) {
    //     Ok(driver) => driver,
    //     Err(e) => panic!("{}", e),
    // };

    println!("Connected to {}", driver);
    if let Err(e) = driver.set_input_mode(InputMode::Full) {
        log::e(&format!("{:?}", e))
    };

    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    'main: loop {
        let led_index = ((start_time.elapsed().subsec_nanos() / (1_000_000_000 / 6)) % 6) as usize;
        if let Err(e) = driver.set_leds(PENDING_LEDS[led_index]) {
            log::e(&format!("{:?}", e));
        }

        if let Err(e) = driver.flush() {
            log::e(&format!("{:?}", e));
        }

        for signal in signals.pending() {
            match signal {
                SIGINT | SIGTERM => {
                    if let Err(e) = driver.reset() {
                        println!("{}", e);
                    }
                    break 'main;
                }
                _ => unreachable!(),
            }
        }
    }
}
