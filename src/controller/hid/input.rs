use self::InputReport::*;

pub enum InputReport {
    CommandResponse,
    FullInput,
    ExtendedInput,
    SimpleInput,
    Unknown,
}

impl From<u8> for InputReport {
    fn from(code: u8) -> InputReport {
        match code {
            0x21 => CommandResponse,
            0x30 | 0x32 | 0x33 => FullInput,
            0x31 => ExtendedInput,
            0x3f => SimpleInput,
            _ => Unknown,
        }
    }
}
