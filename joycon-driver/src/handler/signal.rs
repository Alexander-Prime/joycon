use signal_hook::iterator::Signals;
use signal_hook::{SIGINT, SIGTERM};

use crate::driver::DriverCommand;
use crate::handler::Handler;

pub struct OsSignalHandler(Signals);

impl OsSignalHandler {
    pub fn new() -> Self {
        OsSignalHandler(Signals::new(&[SIGINT, SIGTERM]).unwrap())
    }
}

impl Handler for OsSignalHandler {
    fn read(&mut self) -> Option<DriverCommand> {
        for signal in self.0.pending() {
            match signal {
                SIGINT | SIGTERM => return Some(DriverCommand::Stop),
                _ => unreachable!(),
            }
        }
        None
    }
}
