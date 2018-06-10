use byteorder::{ByteOrder, LittleEndian};

use controller::axis::StickFrame;
use controller::button::ButtonFrame;
use controller::motion::MotionFrame;

use self::InputReport::*;

type BatteryState = u8;

pub enum InputReport {
    CommandResponse {
        battery: BatteryState,
        buttons: ButtonFrame,
        left_stick: StickFrame,
        right_stick: StickFrame,
        data: ResponseData,
    },
    ExtendedInput {
        battery: BatteryState,
        buttons: ButtonFrame,
        left_stick: StickFrame,
        right_stick: StickFrame,
        motion: MotionFrame,
    },
    SimpleInput(u16, u8),
    Unknown,
}

impl<'a> From<&'a [u8]> for InputReport {
    fn from(buf: &[u8]) -> InputReport {
        match buf[0] {
            0x21 => CommandResponse {
                battery: buf[1] >> 1,
                buttons: ButtonFrame::from(&buf[2..4]),
                left_stick: StickFrame::from(&buf[5..7]),
                right_stick: StickFrame::from(&buf[8..10]),
                data: ResponseData::from(&buf[12..48]),
            },
            0x30 | 0x31 | 0x32 | 0x33 => ExtendedInput {
                battery: buf[1] >> 1,
                buttons: ButtonFrame::from(&buf[2..4]),
                left_stick: StickFrame::from(&buf[5..7]),
                right_stick: StickFrame::from(&buf[8..10]),
                // TODO We actually get 3 motion frames here, should probably average them
                motion: MotionFrame::from(&buf[11..22]),
            },
            0x3f => SimpleInput(LittleEndian::read_u16(&buf[1..2]), buf[3]),
            _ => Unknown,
        }
    }
}

enum ResponseData {
    ReadSpi(SpiChunk),
    Unknown,
}

impl<'a> From<&'a [u8]> for ResponseData {
    fn from(buf: &[u8]) -> ResponseData {
        match buf[0] {
            0x10 => ResponseData::ReadSpi(SpiChunk::from(&buf[1..])),
            _ => ResponseData::Unknown,
        }
    }
}

enum SpiChunk {
    BodyColor(u8, u8, u8),
    ButtonColor(u8, u8, u8),
    Unknown,
}

impl<'a> From<&'a [u8]> for SpiChunk {
    fn from(buf: &'a [u8]) -> SpiChunk {
        // Byte 2 is the size; not used for now
        let addr = LittleEndian::read_u16(&buf[..1]);
        match addr {
            0x6050 => SpiChunk::BodyColor(buf[3], buf[4], buf[5]),
            0x6053 => SpiChunk::ButtonColor(buf[3], buf[4], buf[5]),
            _ => SpiChunk::Unknown,
        }
    }
}
