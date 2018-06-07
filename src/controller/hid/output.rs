use endian::u32_to_le_array;

use super::InputMode;

pub const NEUTRAL_RUMBLE: [u8; 8] = [
    0x00, // Neutral L rumble
    0x01,
    0x40,
    0x40,
    0x00, // Neutral R rumble
    0x01,
    0x40,
    0x40,
];

pub enum OutputReport<'a> {
    DoCommand(u8, &'a [u8], Command),
    Rumble(u8, &'a [u8]),
}

impl<'a> OutputReport<'a> {
    fn code(&self) -> u8 {
        match self {
            OutputReport::DoCommand(_, _, _) => 0x01,
            OutputReport::Rumble(_, _) => 0x10,
        }
    }

    pub fn make_buffer(&self) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(11);
        buf.push(self.code());

        match self {
            OutputReport::DoCommand(counter, rumble, sub) => {
                buf.push(*counter);
                buf.extend_from_slice(rumble);
                buf.extend_from_slice(&sub.make_buffer())
            }
            OutputReport::Rumble(counter, rumble) => {
                buf.push(*counter);
                buf.extend_from_slice(rumble);
            }
        }
        buf
    }
}

pub enum Command {
    RequestDeviceInfo,
    SetInputMode(InputMode),
    ReadSpi(u32, usize),
    SetLeds(u8),
}

impl<'a> Command {
    fn code(&self) -> u8 {
        match self {
            Command::RequestDeviceInfo => 0x02,
            Command::SetInputMode(_) => 0x03,
            Command::ReadSpi(_, _) => 0x10,
            Command::SetLeds(_) => 0x30,
        }
    }

    pub fn make_buffer(&self) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(1);
        buf.push(self.code());
        match self {
            Command::SetInputMode(mode) => {
                buf.push(mode.code());
            }
            Command::ReadSpi(addr, len) => {
                buf.reserve(5);
                buf.extend_from_slice(&u32_to_le_array(*addr));
                buf.push(*len as u8);
            }
            Command::SetLeds(bitmask) => {
                buf.push(*bitmask);
            }
            _ => {}
        }
        buf
    }
}
