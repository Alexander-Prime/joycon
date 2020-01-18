use async_std::task;

use crate::driver::DriverChannel;
use crate::handler::Handler;

pub struct ConsoleHandler;

impl ConsoleHandler {
    pub fn new() -> Result<Self, ()> {
        Ok(ConsoleHandler)
    }
}

impl Handler for ConsoleHandler {
    fn start(&self, channel: &DriverChannel) -> task::JoinHandle<()> {
        task::spawn(async {})
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
