use crate::io::{Reader, Writer};

pub struct ConsoleReader;

impl ConsoleReader {
    pub fn open() -> Result<Self, ()> {
        Ok(ConsoleReader)
    }

    // TODO convert this to an async listener

    // let signals = match Signals::new(&[SIGINT, SIGTERM]) {
    //     Ok(signals) => signals,
    //     Err(e) => panic!(e),
    // };

    // 'main: loop {
    //     for signal in signals.pending() {
    //         match signal {
    //             SIGINT | SIGTERM => {
    //                 if let Err(e) = driver.reset() {
    //                     println!("{}", e);
    //                 }
    //                 break 'main;
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    // }
}

impl Reader for ConsoleReader {}

pub struct ConsoleWriter;

impl ConsoleWriter {
    pub fn open() -> Result<Self, ()> {
        Ok(ConsoleWriter)
    }
}

impl Writer for ConsoleWriter {}
