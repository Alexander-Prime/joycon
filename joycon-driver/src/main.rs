mod data;
mod device;
mod driver;
mod handler;

use crate::driver::{Driver, DriverResult};
use crate::handler::{ConsoleHandler, OsSignalHandler, SocketHandler};

fn main() -> DriverResult<()> {
    let device_id = "98:B6:E9:75:53:36";

    let socket_path = format!("/var/run/joycond/{}.fifo", device_id);

    Driver::for_serial_number(device_id)
        .with(ConsoleHandler::new())
        .with(SocketHandler::new(std::path::Path::new(&socket_path)))
        .with(OsSignalHandler::new())
        .build()
        .and_then(|driver| driver.start())?;

    Ok(())
}
