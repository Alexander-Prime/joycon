use std::path::Path;

use crate::driver::{DriverCommand, DriverEvent};
use crate::handler::{Handler, HandlerResult};

pub struct SocketHandler;

impl SocketHandler {
    pub fn new(path: &Path) -> Result<Self, ()> {
        Ok(SocketHandler)
    }
}

impl Handler for SocketHandler {
    fn read(&mut self) -> Option<DriverCommand> {
        None
    }

    fn write(&mut self, event: &DriverEvent) -> HandlerResult<()> {
        Ok(())
    }
}
