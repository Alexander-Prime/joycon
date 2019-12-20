mod device;
mod driver;
mod input;
mod output;
mod service;

use crate::driver::Driver;
use crate::service::ConsoleService;
use crate::service::SocketService;

#[async_std::main]
async fn main() -> Result<(), ()> {
    let device_id = "98:B6:E9:75:53:36";

    let socket_path = format!("/var/run/joycond/{}.fifo", device_id);

    Driver::for_serial_number(device_id)
        .with(ConsoleService::new()?)
        .with(SocketService::new(std::path::Path::new(&socket_path))?)
        .build()
        .start()
        .await;

    Ok(())
}
