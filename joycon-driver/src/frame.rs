use byteorder::{ByteOrder, LittleEndian};

use common::has::Has;

use super::axis::Axis;
use super::button::Button;

pub struct InputFrame {
    pub buttons: ButtonFrame,
    pub axes: AxisFrame,
    pub motion: MotionFrame,
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

impl From<&[u8]> for InputFrame {
    fn from(buf: &[u8]) -> InputFrame {
        let buttons = if buf.len() >= 3 { &buf[0..3] } else { &[0; 3] };
        let axes = if buf.len() >= 9 { &buf[3..9] } else { &[0; 6] };
        // TODO We actually get 3 motion frames here, should probably average them
        let motion = if buf.len() >= 45 {
            &buf[9..45]
        } else {
            &[0; 36]
        };

        InputFrame {
            buttons: ButtonFrame::from(buttons),
            axes: AxisFrame::from(axes),
            motion: MotionFrame::from(motion),
        }
    }
}

#[derive(Default)]
pub struct ButtonFrame(pub u32);

impl From<&[u8]> for ButtonFrame {
    fn from(buf: &[u8]) -> ButtonFrame {
        ButtonFrame(LittleEndian::read_u24(buf))
    }
}

impl Has<Button> for ButtonFrame {
    fn has(&self, btn: Button) -> bool {
        self.0 & <u32>::from(btn) > 0
    }
}

pub struct AxisFrame {
    pub rx: u16,
    pub ry: u16,
    pub lx: u16,
    pub ly: u16,
}

// FIXME this needs to handle 4 axes per frame instead of 2
impl From<&[u8]> for AxisFrame {
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

impl From<&[u8]> for MotionFrame {
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
