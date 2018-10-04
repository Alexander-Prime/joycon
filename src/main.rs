extern crate byteorder;
extern crate hidapi;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate signal_hook;
extern crate termion;

mod controller;
mod has;
mod log;

use std::time::Instant;

use controller::hid::InputMode;
use controller::id::ProductId;
use controller::Controller;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

const PENDING_LEDS: [u8; 6] = [0b0011, 0b0101, 0b1010, 0b1100, 0b1010, 0b0101];

fn main() {
    let signals = match Signals::new(&[SIGINT, SIGTERM]) {
        Ok(signals) => signals,
        Err(e) => panic!(e),
    };

    let mut controllers = <Vec<Controller>>::with_capacity(2);

    match Controller::find(ProductId::JoyConL) {
        Ok(jc) => controllers.push(jc),
        Err(e) => log::e(e),
    }
    match Controller::find(ProductId::JoyConR) {
        Ok(jc) => controllers.push(jc),
        Err(e) => log::e(e),
    }

    // Print some basic device identity info
    for jc in controllers.iter_mut() {
        println!("Connected to {}", jc);
        if let Err(e) = jc.set_input_mode(InputMode::Full) {
            log::e(e);
        }
    }

    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    'main: loop {
        let led_index = (start_time.elapsed().subsec_nanos() / (1_000_000_000 / 6)) as usize;
        for jc in controllers.iter() {
            if let Err(e) = jc.set_leds(PENDING_LEDS[led_index]) {
                log::e(e);
            }
        }
        for jc in controllers.iter_mut() {
            if let Err(e) = jc.flush() {
                log::e(e);
            }
        }
        for signal in signals.pending() {
            match signal {
                SIGINT | SIGTERM => {
                    for jc in controllers.iter() {
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
