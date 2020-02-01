pub mod console;
pub mod socket;

use crate::driver::{DriverCommand, DriverEvent};

pub use self::console::ConsoleHandler;
pub use self::socket::SocketHandler;

pub trait Handler {
  fn read(&mut self) -> Option<DriverCommand>;
  fn write(&mut self, event: DriverEvent) -> HandlerResult;
}

pub type HandlerResult = Result<(), ()>;
