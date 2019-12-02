use arraydeque::{ArrayDeque, Wrapping};
use hidapi::HidResult;

use crate::device::frame::InputFrame;
use crate::device::Device;
use crate::io::{Reader, Writer};

// Offset in SPI memory that `spi_mirror` begins at
const SPI_ORIGIN: u16 = 0x6000;

const PENDING_LEDS: u8 = 0b1111_0000;

pub struct Driver {
    device: Device,
    frames: ArrayDeque<[InputFrame; 32], Wrapping>,
}

impl Driver {
    pub fn for_serial_number(serial_number: &str) -> DriverBuilder {
        DriverBuilder {
            serial_number: String::from(serial_number),
            readers: Default::default(),
            writers: Default::default(),
        }
    }

    pub async fn listen(&self) {}
}

pub struct DriverBuilder {
    serial_number: String,
    readers: Vec<Box<dyn Reader>>,
    writers: Vec<Box<dyn Writer>>,
}

impl DriverBuilder {
    pub fn with_reader(self, reader: Box<dyn Reader>) -> Self {
        self
    }

    pub fn with_writer(self, writer: Box<dyn Writer>) -> Self {
        self
    }

    pub fn build(self) -> HidResult<Driver> {
        Device::for_serial_number(&self.serial_number).map(|device| Driver {
            device,
            frames: ArrayDeque::default(),
        })
        // TODO Find a way to guarantee this isn't racing other input packets
        // device.flush()
        //     .and_then(|_| device.set_input_mode(InputMode::Simple))
        //     .and_then(|_| device.await_input())
        //     .and_then(|_| device.get_device_info())
        //     .and_then(|_| device.await_input())
        //     .and_then(|_| device.read_spi(0x603d, 24)) // Calibration and colors
        //     .and_then(|_| device.await_input())
        //     .and_then(|_| Ok(device))
    }
}
