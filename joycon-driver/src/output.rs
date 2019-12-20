use byteorder::{ByteOrder, LittleEndian};

use super::device::{HciState, InputMode};

use self::Command::*;
use self::OutputReport::*;

pub const NEUTRAL_RUMBLE: [u8; 8] = [
    0x00, // Neutral L rumble
    0x01, 0x40, 0x40, 0x00, // Neutral R rumble
    0x01, 0x40, 0x40,
];

pub enum OutputReport {
    DoCommand(u8, Vec<u8>, Command),
    Rumble(u8, Vec<u8>),
}

impl From<&OutputReport> for u8 {
    fn from(rpt: &OutputReport) -> u8 {
        match rpt {
            DoCommand(_, _, _) => 0x01,
            Rumble(_, _) => 0x10,
        }
    }
}

impl From<OutputReport> for Vec<u8> {
    fn from(rpt: OutputReport) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(11);
        buf.push(u8::from(&rpt));

        match rpt {
            DoCommand(counter, rumble, cmd) => {
                buf.push(counter);
                buf.extend_from_slice(&rumble);
                buf.extend_from_slice(&<Vec<u8>>::from(cmd));
            }
            Rumble(counter, rumble) => {
                buf.push(counter);
                buf.extend_from_slice(&rumble);
            }
        }

        buf
    }
}

pub enum Command {
    RequestDeviceInfo,
    SetInputMode(InputMode),
    SetHciState(HciState),
    ReadSpi(u32, usize),
    SetLeds(u8),
    Unknown,
}

impl From<&[u8]> for Command {
    fn from(buf: &[u8]) -> Command {
        match buf[0] {
            0x02 => RequestDeviceInfo,
            0x03 => SetInputMode(InputMode::from(&buf[1])),
            0x06 => SetHciState(HciState::from(&buf[1])),
            0x10 => ReadSpi(LittleEndian::read_u32(&buf[1..5]), buf[5] as usize),
            0x30 => SetLeds(buf[1]),
            _ => Unknown,
        }
    }
}

impl From<&Command> for u8 {
    fn from(cmd: &Command) -> u8 {
        match cmd {
            RequestDeviceInfo => 0x02,
            SetInputMode(_) => 0x03,
            SetHciState(_) => 0x06,
            ReadSpi(_, _) => 0x10,
            SetLeds(_) => 0x30,
            Unknown => 0x00,
        }
    }
}

impl From<Command> for Vec<u8> {
    fn from(cmd: Command) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(1);
        buf.push(u8::from(&cmd));

        match cmd {
            SetInputMode(mode) => {
                buf.push(u8::from(&mode));
            }
            ReadSpi(addr, len) => {
                buf.resize(5, 0);
                LittleEndian::write_u32(&mut buf[1..5], addr);
                buf.push(len as u8);
            }
            SetLeds(bitmask) => {
                buf.push(bitmask);
            }
            _ => {}
        }
        buf
    }
}
