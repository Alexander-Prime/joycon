pub mod axis;
pub mod button;
pub mod calibration;
pub mod frame;
pub mod id;

use async_std::{sync, task};
use hidapi::{HidApi, HidDevice, HidError, HidResult};

use crate::input::InputReport;
use crate::output::OutputReport;
use id::Product;

pub struct Device {
    serial_number: String,

    firmware_version: u16,
    product: Product,
    mac_address: u64,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],

    input_channel: (sync::Sender<InputReport>, sync::Receiver<InputReport>),
    output_channel: (sync::Sender<OutputReport>, sync::Receiver<OutputReport>),
}

impl Device {
    pub fn new(serial: &'static str) -> Device {
        Device {
            serial_number: String::from(serial),

            firmware_version: 0,
            mac_address: 0,
            product: Product::JoyConR,

            read_buffer: [0u8; 360],

            input_channel: sync::channel::<InputReport>(32),
            output_channel: sync::channel::<OutputReport>(32),
        }
    }

    pub async fn start(&mut self) -> HidResult<()> {
        let Device { serial_number, .. } = self;
        let api = HidApi::new()?;
        let dev_info = api
            .devices()
            .iter()
            .find(|dev| match &dev.serial_number {
                // TODO find a way to do this as a single condition expression
                Some(s) if s == serial_number => true,
                _ => false,
            })
            .ok_or(HidError::HidApiError {
                message: format!(
                    "Couldn't find a device matching serial \"{}\"",
                    serial_number
                ),
            })?;

        let hid_device = dev_info.open_device(&api)?;
        hid_device.set_blocking_mode(false)?;

        loop {
            self.handle_read(&hid_device).await?;
            self.handle_write(&hid_device).await?;
        }
    }

    async fn handle_read(&mut self, hid_device: &HidDevice) -> HidResult<()> {
        while hid_device.read(&mut self.read_buffer[..])? > 0 {
            let report = InputReport::from(&self.read_buffer[..]);
            self.handle_input_report_internal(report);
            self.input_channel
                .0
                .send(InputReport::from(&self.read_buffer[..]))
                .await;
            task::yield_now().await;
        }
        Ok(())
    }

    async fn handle_write(&self, hid_device: &HidDevice) -> HidResult<()> {
        let Device { output_channel, .. } = self;
        let (.., receiver) = output_channel;
        while !receiver.is_empty() {
            if let Some(item) = receiver.recv().await {
                hid_device.write(&<Vec<u8>>::from(item))?;
            }
        }
        Ok(())
    }

    pub fn get_receiver(&self) -> sync::Receiver<InputReport> {
        let (.., receiver) = &self.input_channel;
        receiver.clone()
    }

    pub fn get_sender(&self) -> sync::Sender<OutputReport> {
        let (sender, ..) = &self.output_channel;
        sender.clone()
    }

    fn handle_input_report_internal(&mut self, report: InputReport) {}
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
