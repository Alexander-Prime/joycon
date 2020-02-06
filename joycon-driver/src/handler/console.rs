use crate::driver::{DriverCommand, DriverEvent};
use crate::handler::{Handler, HandlerResult};

pub struct ConsoleHandler;

impl ConsoleHandler {
    pub fn new() -> Result<Self, ()> {
        Ok(ConsoleHandler)
    }
}

impl Handler for ConsoleHandler {
    fn read(&mut self) -> Option<DriverCommand> {
        None
    }

    fn write(&mut self, event: &DriverEvent) -> HandlerResult<()> {
        Ok(())
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
