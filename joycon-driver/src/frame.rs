use std::fmt;

use byteorder::{ByteOrder, LittleEndian};

use common::has::Has;
use common::range::map_range;

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
            axes: Default::default(),
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

#[derive(Copy, Clone)]
pub struct AxisFrame {
    pub lx: u16,
    pub ly: u16,
    pub rx: u16,
    pub ry: u16,
}

impl AxisFrame {
    pub fn to_calibrated(&self, cal: &[u8]) -> (f64, f64, f64, f64) {
        let l_cal = &cal[..9];
        let r_cal = &cal[9..];

        // Calibration data is layed out differently for left & right sticks :/
        let (lx, ly) =
            AxisFrame::calibrate_stick((self.lx, self.ly), &l_cal[..3], &l_cal[3..6], &l_cal[6..9]);
        let (rx, ry) =
            AxisFrame::calibrate_stick((self.rx, self.ry), &r_cal[6..9], &r_cal[..3], &r_cal[3..6]);
        (lx, ly, rx, ry)
    }

    fn calibrate_stick(
        raw: (u16, u16),
        cal_max: &[u8],
        cal_cen: &[u8],
        cal_min: &[u8],
    ) -> (f64, f64) {
        let (x, y) = raw;

        // // Extract 12-bit actual values
        let (max_x, max_y) = AxisFrame::u12_pair_from_bytes(&cal_max);
        let (cen_x, cen_y) = AxisFrame::u12_pair_from_bytes(&cal_cen);
        let (min_x, min_y) = AxisFrame::u12_pair_from_bytes(&cal_min);

        // // Align range around calibrated center
        let max_x: f64 = <f64>::from(cen_x) + <f64>::from(max_x);
        let max_y: f64 = <f64>::from(cen_y) + <f64>::from(max_y);
        let min_x: f64 = <f64>::from(cen_x) - <f64>::from(min_x);
        let min_y: f64 = <f64>::from(cen_y) - <f64>::from(min_y);

        // Normalize to -1..1
        let target_range = (-1f64, 1f64);
        (
            map_range(x.into(), (min_x, max_x), target_range),
            map_range(y.into(), (min_y, max_y), target_range),
        )
    }

    fn u12_pair_from_bytes(buf: &[u8]) -> (u16, u16) {
        let pair = LittleEndian::read_u24(buf);
        ((pair & 0xfff) as u16, (pair >> 12) as u16)
    }
}

impl From<&[u8]> for AxisFrame {
    fn from(buf: &[u8]) -> Self {
        let (lx, ly) = AxisFrame::u12_pair_from_bytes(&buf[..3]);
        let (rx, ry) = AxisFrame::u12_pair_from_bytes(&buf[3..6]);
        AxisFrame { lx, ly, rx, ry }
    }
}

impl Default for AxisFrame {
    fn default() -> Self {
        AxisFrame {
            // 12 bits each
            rx: 0x800,
            ry: 0x800,
            lx: 0x800,
            ly: 0x800,
        }
    }
}

impl fmt::Display for AxisFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "L[{:04}, {:04}] R[{:04}, {:04}]",
            self.lx, self.ly, self.rx, self.ry
        )
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
