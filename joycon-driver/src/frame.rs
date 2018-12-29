use byteorder::{ByteOrder, LittleEndian};

use common::has::Has;

use super::axis::Axis;
use super::button::Button;

pub struct InputFrame {
    buttons: ButtonFrame,
    axes: AxisFrame,
    motion: MotionFrame,
}

impl InputFrame {
    pub fn new() -> InputFrame {
        InputFrame {
            buttons: Default::default(),
            axes: AxisFrame::new(),
            motion: MotionFrame::new(),
        }
    }
}

#[derive(Default)]
pub struct ButtonFrame([u8; 3]);

impl<'a> From<&'a [u8]> for ButtonFrame {
    fn from(buf: &[u8]) -> ButtonFrame {
        let mut buttons: [u8; 3] = Default::default();
        buttons.copy_from_slice(&buf);
        ButtonFrame(buttons)
    }
}

impl<'a> Has<Button> for ButtonFrame {
    fn has(&self, btn: Button) -> bool {
        (&self.0[..]).has(btn)
    }
}

impl<'a> Has<Button> for &'a [u8] {
    fn has(&self, btn: Button) -> bool {
        self[btn.byte_offset()] & btn.bit_mask() > 0
    }
}

pub struct AxisFrame {
    pub rx: u16,
    pub ry: u16,
    pub lx: u16,
    pub ly: u16,
}

// FIXME this needs to handle 4 axes per frame instead of 2
impl<'a> From<&'a [u8]> for AxisFrame {
    fn from(buf: &[u8]) -> AxisFrame {
        AxisFrame {
            rx: buf[0] as u16 | ((buf[1] as u16 & 0xf) << 8),
            ry: (buf[1] as u16 >> 4) | ((buf[2] as u16) << 4),
            lx: 0,
            ly: 0,
        }
    }
}

impl AxisFrame {
    pub fn new() -> AxisFrame {
        AxisFrame {
            // 12 bits each
            rx: 0x800,
            ry: 0x800,
            lx: 0x800,
            ly: 0x800,
        }
    }

    fn get(&self, axis: Axis) -> u16 {
        match axis {
            Axis::Rx => self.rx,
            Axis::Ry => self.ry,
            Axis::Lx => self.lx,
            Axis::Ly => self.ly,
        }
    }
}

pub struct MotionFrame {
    accelerometer: (u16, u16, u16),
    gyroscope: (u16, u16, u16),
}

impl MotionFrame {
    pub fn new() -> MotionFrame {
        MotionFrame {
            accelerometer: (0x8000, 0x8000, 0x8000),
            gyroscope: (0x8000, 0x8000, 0x8000),
        }
    }
}

impl<'a> From<&'a [u8]> for MotionFrame {
    fn from(buf: &[u8]) -> MotionFrame {
        MotionFrame {
            accelerometer: (
                LittleEndian::read_u16(&buf[0..2]),
                LittleEndian::read_u16(&buf[2..4]),
                LittleEndian::read_u16(&buf[4..6]),
            ),
            gyroscope: (
                LittleEndian::read_u16(&buf[6..8]),
                LittleEndian::read_u16(&buf[8..10]),
                LittleEndian::read_u16(&buf[10..12]),
            ),
        }
    }
}
