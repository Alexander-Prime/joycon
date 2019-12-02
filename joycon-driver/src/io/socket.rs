use std::path::Path;

pub struct SocketReader;

impl SocketReader {
    pub fn open(path: &Path) -> Result<Self, ()> {
        Ok(SocketReader)
    }
}

pub struct SocketWriter;

impl SocketWriter {
    pub fn open(path: &Path) -> Result<Self, ()> {
        Ok(SocketWriter)
    }
}
