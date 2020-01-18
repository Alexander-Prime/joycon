mod device;
mod driver;
mod handler;
mod input;
mod output;

use crate::driver::Driver;
use crate::handler::ConsoleHandler;
use crate::handler::SocketHandler;

#[async_std::main]
async fn main() -> Result<(), ()> {
    let device_id = "98:B6:E9:75:53:36";

    let socket_path = format!("/var/run/joycond/{}.fifo", device_id);

    Driver::for_serial_number(device_id)
        .with(ConsoleHandler::new()?)
        .with(SocketHandler::new(std::path::Path::new(&socket_path))?)
        .build()
        .start()
        .await;

    Ok(())
}
