use byteorder::{ByteOrder, LittleEndian};

use controller::axis::StickFrame;
use controller::button::ButtonFrame;
use controller::motion::MotionFrame;

use self::InputReport::*;

type BatteryState = u8;

pub enum InputReport<'a> {
    CommandResponse {
        battery: BatteryState,
        buttons: ButtonFrame,
        left_stick: StickFrame,
        right_stick: StickFrame,
        data: ResponseData<'a>,
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

impl<'a> From<&'a [u8]> for InputReport<'a> {
    fn from(buf: &[u8]) -> InputReport {
        match buf[0] {
            0x21 => CommandResponse {
                // Timer byte at buf[1]
                battery: buf[2] >> 1,
                buttons: ButtonFrame::from(&buf[3..6]),
                left_stick: StickFrame::from(&buf[6..9]),
                right_stick: StickFrame::from(&buf[9..12]),
                data: ResponseData::from(&buf[13..49]),
            },
            0x30 | 0x31 | 0x32 | 0x33 => ExtendedInput {
                battery: buf[1] >> 1,
                buttons: ButtonFrame::from(&buf[2..5]),
                left_stick: StickFrame::from(&buf[5..8]),
                right_stick: StickFrame::from(&buf[8..11]),
                // TODO We actually get 3 motion frames here, should probably average them
                motion: MotionFrame::from(&buf[11..23]),
            },
            0x3f => SimpleInput(LittleEndian::read_u16(&buf[1..4]), buf[3]),
            _ => Unknown,
        }
    }
}

pub enum ResponseData<'a> {
    ReadSpi(SpiChunk),
    Unknown(&'a [u8]),
}

impl<'a> From<&'a [u8]> for ResponseData<'a> {
    fn from(buf: &[u8]) -> ResponseData {
        match buf[1] {
            0x10 => ResponseData::ReadSpi(SpiChunk::from(&buf[2..])),
            _ => ResponseData::Unknown(&buf[..]),
        }
    }
}

pub enum SpiChunk {
    BodyColor(u8, u8, u8),
    ButtonColor(u8, u8, u8),
    Unknown(u16, u8),
}

impl<'a> From<&'a [u8]> for SpiChunk {
    fn from(buf: &'a [u8]) -> SpiChunk {
        // Byte 2 is the size; not used for now
        let addr = LittleEndian::read_u16(&buf[..2]);
        match addr {
            0x6050 => SpiChunk::BodyColor(buf[3], buf[4], buf[5]),
            0x6053 => SpiChunk::ButtonColor(buf[3], buf[4], buf[5]),
            _ => SpiChunk::Unknown(addr, buf[2]),
        }
    }
}
