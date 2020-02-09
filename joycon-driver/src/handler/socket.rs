use std::path::Path;

use crate::handler::Handler;

pub struct SocketHandler;

impl SocketHandler {
    pub fn new(path: &Path) -> Self {
        SocketHandler
    }
}

impl Handler for SocketHandler {}
