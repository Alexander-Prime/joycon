pub enum InputMode {
    Full,
    NfcIr,
    Simple,
}

impl<'a> From<&'a u8> for InputMode {
    fn from(code: &u8) -> InputMode {
        match code {
            0x30 => InputMode::Full,
            0x31 => InputMode::NfcIr,
            0x3f => InputMode::Simple,
            _ => InputMode::Full,
        }
    }
}

impl<'a> From<&'a InputMode> for u8 {
    fn from(mode: &'a InputMode) -> u8 {
        match mode {
            InputMode::Full => 0x30,
            InputMode::NfcIr => 0x31,
            InputMode::Simple => 0x3f,
        }
    }
}

pub enum HciState {
    Disconnect,
    Reconnect,
    Pair,
    Home,
}

impl<'a> From<&'a u8> for HciState {
    fn from(code: &u8) -> HciState {
        match code {
            0x00 => HciState::Disconnect,
            0x01 => HciState::Reconnect,
            0x02 => HciState::Pair,
            0x04 => HciState::Home,
            _ => HciState::Disconnect,
        }
    }
}

impl<'a> From<&'a HciState> for u8 {
    fn from(state: &'a HciState) -> u8 {
        match state {
            HciState::Disconnect => 0x00,
            HciState::Reconnect => 0x01,
            HciState::Pair => 0x02,
            HciState::Home => 0x04,
        }
    }
}
