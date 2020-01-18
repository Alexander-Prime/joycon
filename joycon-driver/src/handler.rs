pub mod console;
pub mod socket;

use async_std::task::JoinHandle;

use crate::driver::DriverChannel;

pub use self::console::ConsoleHandler;
pub use self::socket::SocketHandler;

pub trait Handler {
  fn start(&self, channel: &DriverChannel) -> JoinHandle<()>;
}
