use std::io::{Error, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

use common::ipc::paths;
use common::log;
use common::types::Never;

//TODO This should really be Result<!, Error>, watch `never_type` feature progress
pub fn listen() -> Result<Never, Error> {
    let listener = match UnixListener::bind(paths::DAEMON_PATH) {
        Ok(l) => l,
        Err(e) => panic!("Couldn't open socket: {:?}", e),
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                log::e(&format!("Failed to connect: {:?}", e));
                continue;
            }
        };

        thread::spawn(move || {
            handle_connection(stream);
        });
    }

    //FIXME Use a real Error::new() here
    Err(Error::last_os_error())
}

fn handle_connection(mut stream: UnixStream) {
    stream.write(String::from("bytes").as_bytes());
}
