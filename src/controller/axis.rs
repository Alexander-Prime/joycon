use self::Axis::*;

pub enum Axis {
    X,
    Y,
}

pub enum ControllerAxis {
    Xl,
    Yl,
    Xr,
    Yr,
}

pub struct StickFrame {
    x: u16,
    y: u16,
}

impl<'a> From<&'a [u8]> for StickFrame {
    fn from(buf: &[u8]) -> StickFrame {
        StickFrame {
            x: buf[0] as u16 | ((buf[1] as u16 & 0xf) << 8),
            y: (buf[1] as u16 >> 4) | ((buf[2] as u16) << 4),
        }
    }
}

impl StickFrame {
    fn get(&self, axis: Axis) -> u16 {
        match axis {
            X => self.x,
            Y => self.y,
        }
    }
}
