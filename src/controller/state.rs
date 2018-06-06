use std::convert::From;

use super::button::ControllerButton;

pub struct ControllerState {
    buffer: [u8; 10],
}

impl ControllerState {
    pub fn button(&self, btn: ControllerButton) -> bool {
        self.buffer[btn.byteOffset()] & btn.bitMask() > 0
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

fn stick_h(buf: &[u8]) -> u16 {
    (buf[0] as u16) | ((buf[1] as u16 & 0xf) << 8)
}

fn stick_v(buf: &[u8]) -> u16 {
    ((buf[1] as u16) >> 4) | ((buf[2] as u16) << 4)
}
