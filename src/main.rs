#[macro_use]
extern crate lazy_static;

extern crate hidapi;
extern crate termion;

mod controller;
mod endian;
mod log;

use std::thread::sleep;
use std::time::{Duration, Instant};

use termion::{color::*, style::{*, Reset as Clear}};

use controller::{JoyCon, id::ProductId};

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
    for jc in controllers.iter() {
        print_connected(jc);
    }

    let mut old_index: usize = 0;
    let start_time = Instant::now();

    // Show a moving LED pattern to confirm we're connected and running
    loop {
        let led_index = (start_time.elapsed().subsec_nanos() / (1_000_000_000 / 6)) as usize;
        if led_index != old_index {
            for jc in controllers.iter() {
                if let Err(e) = jc.set_leds(PENDING_LEDS[led_index]) {
                    log::e(&format!(
                        "Failed to set LEDs on [{}]; did it disconnect?",
                        jc.serial_number(),
                    ));
                    log::e(e);
                }
            }
            old_index = led_index;
        }
        sleep(TICK);
    }
    panic!("Main loop crashed. Check the logs.");
}

fn print_connected(joycon: &JoyCon) {
    let [bdy_r, bdy_g, bdy_b] = joycon.body_color();
    let [btn_r, btn_g, btn_b] = joycon.button_color();
    log::i(&format!(
        "Connected to {}{}{} Joy-Con(L) [{}] {}",
        Fg(Rgb(btn_r, btn_g, btn_b)),
        Bg(Rgb(bdy_r, bdy_g, bdy_b)),
        Bold,
        joycon.serial_number(),
        Clear
    ));
}
