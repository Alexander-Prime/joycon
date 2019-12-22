pub mod console;
pub mod socket;

use async_std::task::JoinHandle;

use crate::driver::DriverChannel;

pub use self::console::ConsoleService;
pub use self::socket::SocketService;

pub trait Service {
  fn start_service(&self, channel: &DriverChannel) -> JoinHandle<()>;
}
