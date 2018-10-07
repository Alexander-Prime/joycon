use byteorder::{ByteOrder, LittleEndian};

use common::has::Has;

use super::axis::Axis;
use super::button::{Button, Button::*};

pub struct InputFrame {
    buttons: ButtonFrame,
    axes: AxisFrame,
    motion: MotionFrame,
}

impl InputFrame {
    pub fn new() -> InputFrame {
        InputFrame {
            buttons: ButtonFrame::new(),
            axes: AxisFrame::new(),
            motion: MotionFrame::new(),
        }
    }
}

pub struct ButtonFrame {
    y: bool,
    x: bool,
    b: bool,
    a: bool,
    right_sr: bool,
    right_sl: bool,
    r: bool,
    zr: bool,

    minus: bool,
    plus: bool,
    cr: bool,
    cl: bool,
    home: bool,
    capture: bool,

    down: bool,
    up: bool,
    right: bool,
    left: bool,
    left_sr: bool,
    left_sl: bool,
    l: bool,
    zl: bool,
}

impl<'a> From<&'a [u8]> for ButtonFrame {
    fn from(buf: &[u8]) -> ButtonFrame {
        ButtonFrame {
            y: buf.has(Y),
            x: buf.has(X),
            b: buf.has(B),
            a: buf.has(A),
            right_sr: buf.has(RightSr),
            right_sl: buf.has(RightSl),
            r: buf.has(R),
            zr: buf.has(Zr),

            minus: buf.has(Minus),
            plus: buf.has(Plus),
            cr: buf.has(Cr),
            cl: buf.has(Cl),
            home: buf.has(Home),
            capture: buf.has(Capture),

            down: buf.has(Down),
            up: buf.has(Up),
            right: buf.has(Right),
            left: buf.has(Left),
            left_sr: buf.has(LeftSr),
            left_sl: buf.has(LeftSl),
            l: buf.has(L),
            zl: buf.has(Zl),
        }
    }
}

impl ButtonFrame {
    pub fn new() -> ButtonFrame {
        ButtonFrame {
            y: false,
            x: false,
            b: false,
            a: false,
            right_sr: false,
            right_sl: false,
            r: false,
            zr: false,

            minus: false,
            plus: false,
            cr: false,
            cl: false,
            home: false,
            capture: false,

            down: false,
            up: false,
            right: false,
            left: false,
            left_sr: false,
            left_sl: false,
            l: false,
            zl: false,
        }
    }
}

impl<'a> Has<Button> for ButtonFrame {
    fn has(&self, btn: Button) -> bool {
        match btn {
            Y => self.y,
            X => self.x,
            B => self.b,
            A => self.a,
            RightSr => self.right_sr,
            RightSl => self.right_sl,
            R => self.r,
            Zr => self.zr,

            Minus => self.minus,
            Plus => self.plus,
            Cr => self.cr,
            Cl => self.cl,
            Home => self.home,
            Capture => self.capture,

            Down => self.down,
            Up => self.up,
            Right => self.right,
            Left => self.left,
            LeftSr => self.left_sr,
            LeftSl => self.left_sl,
            L => self.l,
            Zl => self.zl,
        }
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
