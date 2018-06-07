pub mod input;
pub mod output;

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
