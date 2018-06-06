use self::ControllerAnalog::*;

pub enum ControllerAnalog {
    Xl,
    Yl,
    Xr,
    Yr,
}

impl ControllerAnalog {
    pub fn rawValue(&self, buf: &[u8]) -> u16 {
        match self {
            Xl | Xr => buf[0] as u16 | ((buf[1] as u16 & 0xf) << 8),
            Yl | Yr => (buf[1] as u16 >> 4) | ((buf[2] as u16) << 4),
        }
    }
}
