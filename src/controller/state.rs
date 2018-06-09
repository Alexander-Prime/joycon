use std::convert::From;

use super::axis::ControllerAxis as Axis;
use super::button::ControllerButton as Button;

pub struct ControllerState {
    buffer: [u8; 10],
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState { buffer: [0; 10] }
    }

    pub fn button(&self, btn: Button) -> bool {
        self.buffer[btn.byteOffset()] & btn.bitMask() > 0
    }

    pub fn axis(&self, axis: Axis) -> u16 {
        let range = match axis {
            Axis::Xl | Axis::Yl => 6..8,
            Axis::Xr | Axis::Yr => 9..11,
        };
        axis.raw_value(&self.buffer[range])
    }
}

impl From<[u8; 10]> for ControllerState {
    fn from(buf: [u8; 10]) -> ControllerState {
        ControllerState { buffer: buf }
    }
}

fn buttons(buf: &[u8]) -> u32 {
    ((buf[3] as u32) << 16) & ((buf[4] as u32) << 8) & (buf[5] as u32)
}
