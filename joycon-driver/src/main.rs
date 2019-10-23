mod device;
mod driver;
mod input;
mod output;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use common::log;

use crate::device::{id::Product, InputMode};
use crate::driver::Driver;

const PENDING_LEDS: u8 = 0b1111_0000;

fn main() {
    let signals = match Signals::new(&[SIGINT, SIGTERM]) {
        Ok(signals) => signals,
        Err(e) => panic!(e),
    };

    let mut driver = match Driver::find(Product::JoyConL)
        .or_else(|_| Driver::find(Product::JoyConR))
        .or_else(|_| Driver::find(Product::ProController))
    {
        Ok(driver) => driver,
        Err(_) => panic!("No Joy-Con or Switch Pro Controller devices found"),
    };

    if let Err(e) = driver
        .set_input_mode(InputMode::Full)
        .and_then(|_| driver.set_leds(PENDING_LEDS))
    {
        log::e(&format!("{:?}", e));
    }

    println!("Connected to {}", driver);

    'main: loop {
        if let Err(e) = driver.flush() {
            log::e(&format!("{:?}", e));
        }

        println!("{}", driver);

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
