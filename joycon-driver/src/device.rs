pub mod id;

use hidapi::{HidApi, HidDevice, HidError, HidResult};

use crate::data::raw::{InputReport, OutputReport};

pub struct Device {
    serial_number: String,
    hid_device: HidDevice,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],
}

impl Device {
    pub fn open(serial: &'static str) -> HidResult<Device> {
        let api = HidApi::new()?;
        let dev_info = api
            .devices()
            .iter()
            .find(|dev| match &dev.serial_number {
                // TODO find a way to do this as a single condition expression
                Some(s) if s == serial => true,
                _ => false,
            })
            .ok_or(HidError::HidApiError {
                message: format!("Couldn't find a device matching serial \"{}\"", serial),
            })?;

        let hid_device = dev_info.open_device(&api)?;
        hid_device.set_blocking_mode(false)?;

        Ok(Self {
            serial_number: serial.to_owned(),
            hid_device,
            read_buffer: [0u8; 360],
        })
    }

    pub fn read<'a>(&'a mut self) -> Option<HidResult<InputReport<'a>>> {
        match self.hid_device.read(&mut self.read_buffer) {
            Ok(0) => None,
            Err(e) => Some(Err(e)),
            Ok(_) => Some(Ok(InputReport(&self.read_buffer))),
        }
    }

    pub fn write(&self, report: OutputReport) -> HidResult<()> {
        self.hid_device.write(report.0)?;
        Ok(())
    }
}

pub enum InputMode {
    Full,
    NfcIr,
    Simple,
}

impl From<&u8> for InputMode {
    fn from(code: &u8) -> InputMode {
        match code {
            0x30 => InputMode::Full,
            0x31 => InputMode::NfcIr,
            0x3f => InputMode::Simple,
            _ => InputMode::Full,
        }
    }
}

impl From<&InputMode> for u8 {
    fn from(mode: &InputMode) -> u8 {
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

impl From<&u8> for HciState {
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

impl From<&HciState> for u8 {
    fn from(state: &HciState) -> u8 {
        match state {
            HciState::Disconnect => 0x00,
            HciState::Reconnect => 0x01,
            HciState::Pair => 0x02,
            HciState::Home => 0x04,
        }
    }
}
