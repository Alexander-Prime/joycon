use async_std::task;
use std::path::Path;

use crate::driver::DriverChannel;
use crate::service::Service;

pub struct SocketService;

impl SocketService {
    pub fn new(path: &Path) -> Result<Self, ()> {
        Ok(SocketService)
    }
}

impl Service for SocketService {
    fn start_service(&self, channel: &DriverChannel) -> task::JoinHandle<()> {
        task::spawn(async {})
    }
}
