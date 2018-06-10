#[macro_use]
extern crate lazy_static;
extern crate byteorder;

extern crate hidapi;
extern crate termion;

mod controller;
mod has;
mod log;

use std::time::Instant;

use controller::JoyCon;
use controller::hid::InputMode;
use controller::id::ProductId;

const PENDING_LEDS: [u8; 64] = [
    4, 14, 11, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 7, 13, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

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
    for jc in controllers.iter() {
        println!("Connected to {}", jc.identify());
        if let Err(e) = jc.set_input_mode(InputMode::Full) {
            log::e(e);
        }
    }

    let mut old_index: usize = 0;
    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    loop {
        let led_index = (start_time.elapsed().subsec_nanos() / (1_000_000_000 / 1)) as usize;
        for jc in controllers.iter_mut() {
            if let Err(e) = match jc.handle_input() {
                Ok(_) => if led_index != old_index {
                    jc.set_leds(PENDING_LEDS[led_index])
                } else {
                    Ok(1)
                },
                Err(e) => Err(e),
            } {
                log::e(e);
            } else {
                log::d(&jc.input_str());
            }
        }
        old_index = led_index;
    }
}
