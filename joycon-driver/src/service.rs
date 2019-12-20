pub mod console;
pub mod socket;

use crate::driver::DriverChannel;
use async_std::task::JoinHandle;

pub use self::console::ConsoleService;
pub use self::socket::SocketService;

pub trait Service {
  fn start_service(&self, channel: &DriverChannel) -> JoinHandle<()>;
}
