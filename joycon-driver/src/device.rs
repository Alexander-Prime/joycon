pub mod axis;
pub mod button;
pub mod calibration;
pub mod frame;
pub mod id;

use hidapi::{HidApi, HidDevice, HidError, HidResult};

use common::log;

use id::{Product, Vendor};

const DEFAULT_BODY_COLOR: (u8, u8, u8) = (0x40, 0x40, 0x40);
const DEFAULT_BUTTON_COLOR: (u8, u8, u8) = (0x1c, 0x1c, 0x1c);

fn init_api() -> HidApi {
    HidApi::new().unwrap_or_else(|e| {
        log::wtf("Couldn't initialize HidApi");
        panic!(e);
    })
}

pub struct Device {
    hid_device: HidDevice,

    serial_number: String,
    rumble_counter: u8,
    leds: u8,

    firmware_version: u16,
    product: Product,
    mac_address: u64,

    // Mirror of a subset of the Joy-Con's internal flash memory
    spi_mirror: [u8; 0xA000],

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],
}

impl Device {
    fn open(hid_device: HidDevice) -> HidResult<Device> {
        Ok(Device {
            hid_device,

            rumble_counter: 0,
            serial_number: String::default(),
            leds: 0x00,

            firmware_version: 0,
            mac_address: 0,
            product: Product::JoyConR,

            spi_mirror: [0; 0xA000],

            read_buffer: [0; 360],
        })
    }

    pub fn find_product(product: Product) -> HidResult<Device> {
        let api = init_api();
        api.open(Vendor::Nintendo as u16, product as u16)
            .and_then(|dev| Self::open(dev))
    }

    pub fn for_serial_number(serial: &str) -> HidResult<Device> {
        let api = init_api();
        api.devices()
            .iter()
            .find(|&dev| dev.serial_number == Some(String::from(serial)))
            .ok_or(HidError::HidApiError {
                message: format!("Couldn't find a device matching serial \"{}\"", serial),
            })
            .and_then(|dev| api.open_path(&dev.path))
            .and_then(|dev| Self::open(dev))
    }

    pub async fn receive(&self) -> () {}

    pub async fn send() -> () {}
}

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
