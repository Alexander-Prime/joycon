use byteorder::{ByteOrder, LittleEndian};

pub struct MotionFrame {
    accelerometer: (u16, u16, u16),
    gyroscope: (u16, u16, u16),
}

impl<'a> From<&'a [u8]> for MotionFrame {
    fn from(buf: &[u8]) -> MotionFrame {
        MotionFrame {
            accelerometer: (
                LittleEndian::read_u16(&buf[0..1]),
                LittleEndian::read_u16(&buf[2..3]),
                LittleEndian::read_u16(&buf[4..5]),
            ),
            gyroscope: (
                LittleEndian::read_u16(&buf[6..7]),
                LittleEndian::read_u16(&buf[8..9]),
                LittleEndian::read_u16(&buf[10..11]),
            ),
        }
    }
}
