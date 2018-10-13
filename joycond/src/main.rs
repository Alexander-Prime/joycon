extern crate common;

mod daemon;

fn main() {
    match daemon::listen() {
        Err(e) => panic!(e),
        _ => (),
    }
}
