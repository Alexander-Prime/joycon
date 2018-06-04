use endian::u32_to_le_array;

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

pub enum Command<'a> {
    DoSubcommand(u8, &'a [u8], Subcommand),
    Rumble(u8, &'a [u8]),
}

impl<'a> Command<'a> {
    fn code(&self) -> u8 {
        match self {
            Command::DoSubcommand(_, _, _) => 0x01,
            Command::Rumble(_, _) => 0x10,
        }
    }

    pub fn make_buffer(&self) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(11);
        buf.push(self.code());

        match self {
            Command::DoSubcommand(counter, rumble, sub) => {
                buf.push(*counter);
                buf.extend_from_slice(rumble);
                buf.extend_from_slice(&sub.make_buffer())
            }
            Command::Rumble(counter, rumble) => {
                buf.push(*counter);
                buf.extend_from_slice(rumble);
            }
        }
        buf
    }
}

pub enum Subcommand {
    RequestDeviceInfo,
    SetInputMode(InputMode),
    ReadSpi(u32, usize),
    SetLeds(u8),
}

impl<'a> Subcommand {
    fn code(&self) -> u8 {
        match self {
            Subcommand::RequestDeviceInfo => 0x02,
            Subcommand::SetInputMode(_) => 0x03,
            Subcommand::ReadSpi(_, _) => 0x10,
            Subcommand::SetLeds(_) => 0x30,
        }
    }

    pub fn make_buffer(&self) -> Vec<u8> {
        let mut buf = <Vec<u8>>::with_capacity(1);
        buf.push(self.code());
        match self {
            Subcommand::SetInputMode(mode) => {
                buf.push(mode.code());
            }
            Subcommand::ReadSpi(addr, len) => {
                buf.reserve(5);
                buf.extend_from_slice(&u32_to_le_array(*addr));
                buf.push(*len as u8);
            }
            Subcommand::SetLeds(bitmask) => {
                buf.push(*bitmask);
            }
            _ => {}
        }
        buf
    }
}

pub enum InputMode {
    Full,
    NfcIr,
    Simple,
}

impl InputMode {
    fn code(&self) -> u8 {
        match self {
            InputMode::Full => 0x30,
            InputMode::NfcIr => 0x31,
            InputMode::Simple => 0x3f,
        }
    }
}
