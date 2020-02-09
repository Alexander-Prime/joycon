pub mod console;
pub mod signal;
pub mod socket;

use crate::driver::{DriverCommand, DriverEvent};

pub use self::console::ConsoleHandler;
pub use self::signal::OsSignalHandler;
pub use self::socket::SocketHandler;

pub trait Handler {
  fn read(&mut self) -> Option<DriverCommand> {
    None
  }
  fn write(&mut self, event: &DriverEvent) -> HandlerResult<()> {
    Ok(())
  }
}

pub type HandlerResult<T> = Result<T, HandlerError>;

#[derive(Debug)]
pub enum HandlerError {
  BuildFailed,
  Unknown,
}
