use std::path::PathBuf;

pub static ROOT_PATH: &str = "/var/run/joycond/";
pub static DAEMON_PATH: &str = "/var/run/joycond/daemon.sock";
pub static PROXY_ROOT_PATH: &str = "/var/run/joycond/proxy/";

pub fn proxy_path(id: &str) -> PathBuf {
    let mut buf = PathBuf::from(PROXY_ROOT_PATH);
    buf.push(id);
    buf.push(".sock");
    buf
}
