use arrayref::array_refs;
use byteorder::{ByteOrder, LittleEndian};

pub struct StickCalibration {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
}

impl StickCalibration {
    pub fn from_raw_right(raw: &[u8; 9]) -> Self {
        let (center, min, max) = array_refs![raw, 3, 3, 3];
        Self::from_raw_parts(center, min, max)
    }

    pub fn from_raw_left(raw: &[u8; 9]) -> Self {
        let (max, center, min) = array_refs![raw, 3, 3, 3];
        Self::from_raw_parts(center, min, max)
    }

    fn from_raw_parts(center: &[u8; 3], min: &[u8; 3], max: &[u8; 3]) -> Self {
        let (max_x_diff, max_y_diff) = u12_pair_from_bytes(max);
        let (center_x, center_y) = u12_pair_from_bytes(center);
        let (min_x_diff, min_y_diff) = u12_pair_from_bytes(min);
        StickCalibration {
            min_x: center_x - min_x_diff,
            max_x: center_x + max_x_diff,
            min_y: center_x - min_y_diff,
            max_y: center_y + max_y_diff,
        }
    }
}

pub struct AccelCalibration(pub [u8; 12]);

pub struct GyroCalibration(pub [u8; 12]);

pub struct Calibration(
    pub StickCalibration,
    pub AccelCalibration,
    pub GyroCalibration,
);

fn u12_pair_from_bytes(buf: &[u8; 3]) -> (u16, u16) {
    let pair = LittleEndian::read_u24(buf);
    ((pair & 0xfff) as u16, (pair >> 12) as u16)
}
