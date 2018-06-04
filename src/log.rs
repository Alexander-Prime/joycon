use std::time::{Duration, Instant};

use termion::{color::*, style::{*, Reset as Clear}};

static mut START_TIME: Option<Instant> = None;

pub fn i(message: &str) {
    println!(
        "{}{}(I){}{} {}",
        Fg(Green),
        Faint,
        timestamp(),
        Clear,
        message
    );
}

pub fn d(message: &str) {
    println!("{}(D){}{} {}", Fg(Yellow), timestamp(), Clear, message);
}

pub fn e(message: &str) {
    println!("{}{}(E){}{} {}", Fg(Red), Bold, timestamp(), Clear, message);
}

pub fn wtf(message: &str) {
    println!(
        "{}{}{}(!){}{} {}",
        Fg(White),
        Bg(Red),
        Bold,
        timestamp(),
        Clear,
        message
    );
}

pub fn buf(buf: &[u8]) -> String {
    if buf.len() == 0 {
        return String::from("[]");
    }

    let mut strings = <Vec<String>>::with_capacity(buf.len() + 2);
    strings.push("[".to_string());
    for &byte in &buf[..] {
        strings.push(format!(" {:02x}", &byte));
    }
    strings.push(" ]".to_string());

    strings.concat()
}

fn uptime_elapsed() -> Duration {
    unsafe {
        match START_TIME {
            Some(time) => time.elapsed(),
            None => {
                START_TIME = Some(Instant::now());
                uptime_elapsed()
            }
        }
    }
}

fn timestamp() -> String {
    let now = uptime_elapsed();
    format!("[{:8}.{:08}]", now.as_secs(), now.subsec_nanos() / 1000)
}
