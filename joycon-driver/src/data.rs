pub mod calibration;
pub mod raw;

use arrayref::array_refs;

use crate::data::calibration::{AccelCalibration, Calibration, GyroCalibration, StickCalibration};
use crate::data::raw::{InputReport, JoyconLeftButtons, JoyconRightButtons, SharedButtons};
use crate::device::id::Product;

pub struct StickFrame(pub f32, pub f32);

impl StickFrame {
    pub fn calibrated_from_raw(raw: &[u8; 3], calibration: &StickCalibration) -> Self {
        StickFrame(0.0, 0.0)
    }
}

pub struct AccelFrame(pub f32, pub f32, pub f32);

impl AccelFrame {
    pub fn calibrated_from_raw(raw: &[u8; 36], calibration: &AccelCalibration) -> Self {
        AccelFrame(0.0, 0.0, 0.0)
    }
}

pub struct GyroFrame(pub f32, pub f32, pub f32);

impl GyroFrame {
    pub fn calibrated_from_raw(raw: &[u8; 36], calibration: &GyroCalibration) -> Self {
        GyroFrame(0.0, 0.0, 0.0)
    }
}

pub enum InputFrame {
    JoyconRight {
        y: bool,
        x: bool,
        b: bool,
        a: bool,

        sr: bool,
        sl: bool,

        r: bool,
        zr: bool,

        plus: bool,
        click: bool,
        home: bool,

        charge_grip: bool,

        stick: StickFrame,

        accel: AccelFrame,
        gyro: GyroFrame,
    },
    JoyconLeft {
        south: bool,
        north: bool,
        east: bool,
        west: bool,

        sr: bool,
        sl: bool,

        l: bool,
        zl: bool,

        minus: bool,
        click: bool,
        capture: bool,

        charge_grip: bool,

        stick: StickFrame,

        accel: AccelFrame,
        gyro: GyroFrame,
    },
    Unknown,
}

impl InputFrame {
    pub fn from_simple_input(report: InputReport, product: Product) -> Self {
        Self::Unknown
    }

    pub fn from_standard_input(
        report: InputReport,
        product: Product,
        calibration: &Calibration,
    ) -> Self {
        let Calibration(stick_calibration, accel_calibration, gyro_calibration) = calibration;

        let (report_id, timer, _, buttons, left_stick, right_stick, _, motion, _) =
            array_refs![report.0, 1, 1, 1, 3, 3, 3, 1, 36, 360 - 49];

        let r_buttons = JoyconRightButtons::from_bits_truncate(buttons[0]);
        let s_buttons = SharedButtons::from_bits_truncate(buttons[1]);
        let l_buttons = JoyconLeftButtons::from_bits_truncate(buttons[2]);

        match product {
            Product::JoyconLeft => Self::JoyconLeft {
                south: l_buttons.contains(JoyconLeftButtons::SOUTH),
                north: l_buttons.contains(JoyconLeftButtons::NORTH),
                east: l_buttons.contains(JoyconLeftButtons::EAST),
                west: l_buttons.contains(JoyconLeftButtons::WEST),

                sr: l_buttons.contains(JoyconLeftButtons::SR),
                sl: l_buttons.contains(JoyconLeftButtons::SL),
                l: l_buttons.contains(JoyconLeftButtons::L),
                zl: l_buttons.contains(JoyconLeftButtons::ZL),

                minus: s_buttons.contains(SharedButtons::MINUS),
                click: s_buttons.contains(SharedButtons::CL),
                capture: s_buttons.contains(SharedButtons::CAPTURE),
                charge_grip: s_buttons.contains(SharedButtons::CHARGE_GRIP),

                stick: StickFrame::calibrated_from_raw(left_stick, stick_calibration),

                accel: AccelFrame::calibrated_from_raw(motion, accel_calibration),
                gyro: GyroFrame::calibrated_from_raw(motion, gyro_calibration),
            },
            Product::JoyconRight => Self::JoyconRight {
                y: r_buttons.contains(JoyconRightButtons::Y),
                x: r_buttons.contains(JoyconRightButtons::X),
                b: r_buttons.contains(JoyconRightButtons::B),
                a: r_buttons.contains(JoyconRightButtons::A),

                sr: r_buttons.contains(JoyconRightButtons::SR),
                sl: r_buttons.contains(JoyconRightButtons::SL),
                r: r_buttons.contains(JoyconRightButtons::R),
                zr: r_buttons.contains(JoyconRightButtons::ZR),

                plus: s_buttons.contains(SharedButtons::PLUS),
                click: s_buttons.contains(SharedButtons::CL),
                home: s_buttons.contains(SharedButtons::HOME),
                charge_grip: s_buttons.contains(SharedButtons::CHARGE_GRIP),

                stick: StickFrame::calibrated_from_raw(right_stick, stick_calibration),

                accel: AccelFrame::calibrated_from_raw(motion, accel_calibration),
                gyro: GyroFrame::calibrated_from_raw(motion, gyro_calibration),
            },
            _ => Self::Unknown,
        }
    }
}

pub struct FlashMirror([u8; 32768]);
