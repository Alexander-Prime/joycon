use async_std::task;
use std::path::Path;

use crate::driver::DriverChannel;
use crate::handler::Handler;

pub struct SocketHandler;

impl SocketHandler {
    pub fn new(path: &Path) -> Result<Self, ()> {
        Ok(SocketHandler)
    }
}

impl Handler for SocketHandler {
    fn start(&self, channel: &DriverChannel) -> task::JoinHandle<()> {
        task::spawn(async {})
    }
}
