mod device;
mod driver;
mod input;
mod io;
mod output;

use crate::driver::Driver;
use crate::io::console::{ConsoleReader, ConsoleWriter};

fn main() {
    let device_id = "98:B6:E9:75:53:36";

    let socket_path = format!("/var/run/joycond/{}.fifo", device_id);

    let console_reader = ConsoleReader::open().unwrap();
    let console_writer = ConsoleWriter::open().unwrap();

    let driver = Driver::for_serial_number(device_id)
        .with_reader(Box::new(console_reader))
        .with_writer(Box::new(console_writer))
        // .with_reader(socket_reader)
        // .with_writer(socket_writer)
        .build()
        .unwrap();

    driver.listen();
}
