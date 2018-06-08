#[macro_use]
extern crate lazy_static;

extern crate hidapi;
extern crate termion;

mod controller;
mod endian;
mod log;

use std::time::{Duration, Instant};

use controller::JoyCon;
use controller::hid::InputMode;
use controller::id::ProductId;

const PENDING_LEDS: [u8; 6] = [3, 5, 10, 12, 10, 5];
const TICK: Duration = Duration::from_millis(16);

fn main() {
    let mut controllers = <Vec<JoyCon>>::with_capacity(2);

    // These can be replaced with JoyCon::from_serial() for testing
    match JoyCon::find(ProductId::JoyConL) {
        Ok(jc) => controllers.push(jc),
        Err(e) => log::e(e),
    }
    match JoyCon::find(ProductId::JoyConR) {
        Ok(jc) => controllers.push(jc),
        Err(e) => log::e(e),
    }

    // Print some basic device identity info
    for jc in controllers.iter_mut() {
        println!("Connected to {}", jc.identify());
        jc.set_input_mode(InputMode::Full).expect("");
    }

    let mut old_index: usize = 0;
    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    loop {
        let led_index = (start_time.elapsed().subsec_nanos() / (1_000_000_000 / 6)) as usize;
        for jc in controllers.iter_mut() {
            jc.handle_input();
            if led_index != old_index {
                if let Err(e) = jc.set_leds(PENDING_LEDS[led_index]) {
                    log::e(&format!(
                        "Failed to set LEDs on [{}]; did it disconnect?",
                        jc.serial_number(),
                    ));
                    log::e(e);
                }
            }
        }
        old_index = led_index;
    }
}
