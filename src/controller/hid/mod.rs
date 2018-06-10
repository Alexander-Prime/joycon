pub mod input;
pub mod output;

use InputMode::*;

pub enum InputMode {
    Full,
    NfcIr,
    Simple,
}

impl<'a> From<&'a u8> for InputMode {
    fn from(code: &u8) -> InputMode {
        match code {
            0x30 => Full,
            0x31 => NfcIr,
            0x3f => Simple,
            _ => Full,
        }
    }
}

impl<'a> From<&'a InputMode> for u8 {
    fn from(mode: &'a InputMode) -> u8 {
        match mode {
            Full => 0x30,
            NfcIr => 0x31,
            Simple => 0x3f,
        }
    }
}
