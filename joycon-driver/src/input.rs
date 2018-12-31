use byteorder::{BigEndian, ByteOrder, LittleEndian};

use super::frame::{AxisFrame, ButtonFrame, InputFrame, MotionFrame};

use self::InputReport::*;

type BatteryState = u8;

pub enum InputReport<'a> {
    CommandResponse {
        battery: BatteryState,
        frame: InputFrame,
        data: ResponseData<'a>,
    },
    ExtendedInput {
        battery: BatteryState,
        frame: InputFrame,
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
                frame: InputFrame::from(&buf[3..12]),
                data: ResponseData::from(&buf[13..49]),
            },
            0x30 | 0x31 | 0x32 | 0x33 => ExtendedInput {
                // Timer byte at buf[1]
                battery: buf[2] >> 1,
                frame: InputFrame::from(&buf[3..49]),
            },
            0x3f => SimpleInput(LittleEndian::read_u16(&buf[1..4]), buf[3]),
            _ => Unknown,
        }
    }
}

pub enum ResponseData<'a> {
    RequestDeviceInfo {
        firmware_version: u16,
        device_type: u8,
        mac_address: u64,
    },
    SetInputMode,
    ReadSpi(SpiChunk<'a>),
    SetLeds,
    GetLeds,
    Unknown(&'a [u8]),
}

impl<'a> From<&'a [u8]> for ResponseData<'a> {
    fn from(buf: &[u8]) -> ResponseData {
        match buf[1] {
            0x02 => ResponseData::RequestDeviceInfo {
                firmware_version: LittleEndian::read_u16(&buf[2..4]),
                device_type: buf[4],
                mac_address: BigEndian::read_u48(&buf[6..12]),
            },
            0x03 => ResponseData::SetInputMode,
            0x10 => ResponseData::ReadSpi(SpiChunk::from(&buf[2..])),
            0x30 => ResponseData::SetLeds,
            0x31 => ResponseData::GetLeds,
            _ => ResponseData::Unknown(&buf[..]),
        }
    }
}

pub struct SpiChunk<'a>(pub u16, pub &'a [u8]);

impl<'a> From<&'a [u8]> for SpiChunk<'a> {
    fn from(buf: &'a [u8]) -> SpiChunk {
        let addr = LittleEndian::read_u16(&buf[..4]);
        let size = buf[4] as usize;
        let buf = &buf[5..5 + size];
        SpiChunk(addr, buf)
    }
}
