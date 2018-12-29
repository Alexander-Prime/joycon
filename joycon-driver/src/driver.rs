use std::cell::Cell;
use std::fmt;

use arraydeque::{ArrayDeque, Wrapping};
use hidapi::{HidApi, HidDevice, HidError};
use termion::{color, style};

use common::has::Has;
use common::log;

use super::button::Button;
use super::device::{HciState, InputMode};
use super::frame::{AxisFrame, ButtonFrame, InputFrame, MotionFrame};
use super::id::{Product, Vendor};
use super::input::{InputReport, ResponseData, SpiChunk};
use super::output::{Command::*, OutputReport::*, NEUTRAL_RUMBLE};

fn init_api() -> HidApi {
    match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            log::wtf("Couldn't initialize HidApi");
            panic!(e);
        }
    }
}

// Offset in SPI memory that `spi_mirror` begins at
const SPI_ORIGIN: u16 = 0x6000;

const DEFAULT_BODY_COLOR: (u8, u8, u8) = (0x40, 0x40, 0x40);
const DEFAULT_BUTTON_COLOR: (u8, u8, u8) = (0x1c, 0x1c, 0x1c);

pub struct Driver {
    device: HidDevice,
    serial_number: String,
    rumble_counter: Cell<u8>,
    leds: Cell<u8>,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],

    // Mirror of a subset of the Joy-Con's internal flash memory
    spi_mirror: [u8; 0xA000],

    frames: ArrayDeque<[InputFrame; 32], Wrapping>,
}

impl Driver {
    /// Constructs a new Driver for the first device matching the given product ID
    pub fn find(product: Product) -> Result<Driver, HidError> {
        let api = init_api();
        match api.open(Vendor::Nintendo as u16, product as u16) {
            Ok(device) => Driver::for_device(device),
            Err(e) => Err(e),
        }
    }

    /// Constructs a new Driver for the device matching the given serial number
    pub fn for_serial(serial: &str) -> Result<Driver, HidError> {
        let api = init_api();
        let device_info = api.devices().iter().find(|dev| match &dev.serial_number {
            Some(s) if s == serial => true,
            Some(_) | None => false,
        });
        let device_info = match device_info {
            Some(d) => d,
            None => {
                return Err(HidError::HidApiError {
                    message: format!("Couldn't find a device matching serial \"{}\"", serial),
                })
            }
        };

        let device = match api.open_path(&device_info.path) {
            Ok(dev) => dev,
            Err(e) => return Err(e),
        };
        return Driver::for_device(device);
    }

    fn for_device(device: HidDevice) -> Result<Driver, HidError> {
        let serial = match device.get_serial_number_string() {
            Ok(Some(s)) => s,
            Ok(None) => String::new(),
            Err(e) => return Err(e),
        };

        if let Err(e) = device.set_blocking_mode(false) {
            return Err(e);
        }

        let mut jc = Driver {
            device,
            rumble_counter: Cell::new(0),
            serial_number: serial,
            leds: Cell::new(0x00),

            spi_mirror: [0; 0xA000],

            read_buffer: [0; 360],

            frames: ArrayDeque::new(),
        };

        // TODO Find a way to guarantee this isn't racing other input packets
        jc.flush()
            .and_then(|_| jc.set_input_mode(InputMode::Simple))
            .and_then(|_| jc.await_input())
            .and_then(|_| jc.read_spi(0x6050, 6))
            .and_then(|_| jc.await_input())
            .and_then(|_| Ok(jc))
    }

    /// Read and handle all buffered inputs. Blocks until the queue is emptied.
    /// On success, returns `Ok(len)`, where `len` is the number of inputs that were flushed.
    pub fn flush(&mut self) -> Result<usize, HidError> {
        let mut count = 0;
        loop {
            match self.handle_input() {
                Ok(None) => return Ok(count),
                Err(e) => return Err(e),
                _ => count += 1,
            }
        }
    }

    /// Block until the next input packet and handle it with `handle_input()`
    fn await_input(&mut self) -> Result<usize, HidError> {
        // TODO Investigate whether `set_blocking_mode()` introduces any overhead
        if let Err(e) = self.device.set_blocking_mode(true) {
            return Err(e);
        };
        let result = match self.handle_input() {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Ok(0),
            Err(e) => Err(e),
        };
        if let Err(e) = self.device.set_blocking_mode(false) {
            return Err(e);
        };
        result
    }

    /// Receive an input packet, read its input report code, and handle the rest
    /// of its data appropriately. Callers cannot access this data directly;
    /// instead, the data is saved to the controller's state and can be read
    /// after `handle_input()` returns.
    fn handle_input(&mut self) -> Result<Option<usize>, HidError> {
        let mut buf = self.read_buffer;

        let len = match self.device.read(&mut buf[..]) {
            Ok(0) => return Ok(None),
            Err(e) => return Err(e),
            Ok(len) => len,
        };

        let report = InputReport::from(&buf[..]);
        match report {
            InputReport::CommandResponse {
                battery: _,
                frame,
                data,
            } => {
                self.frames.push_back(frame);
                self.handle_response(data);
            }
            InputReport::ExtendedInput { battery: _, frame } => {
                self.frames.push_back(frame);
                ()
            }
            _ => (),
        }
        Ok(Some(len))
    }

    fn handle_response(&mut self, data: ResponseData) {
        match data {
            ResponseData::ReadSpi(chunk) => self.save_spi_chunk(chunk),
            ResponseData::Unknown(buf) => {
                log::e(&format!(
                    "Received unknown response ACK {}",
                    log::buf(&buf[..2])
                ));
                log::e(&log::buf(buf))
            }
            _ => (),
        }
    }

    fn save_spi_chunk(&mut self, chunk: SpiChunk) {
        let SpiChunk(addr, buf) = chunk;
        let start = (addr - SPI_ORIGIN) as usize;
        let end = start + buf.len();
        self.spi_mirror[start..end].copy_from_slice(buf);
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    pub fn set_leds(&self, bitmask: u8) -> Result<usize, HidError> {
        if bitmask == self.leds.replace(bitmask) {
            return Ok(0);
        }
        let sub = SetLeds(bitmask);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    pub fn set_input_mode(&self, mode: InputMode) -> Result<usize, HidError> {
        let sub = SetInputMode(mode);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    pub fn reset(&self) -> Result<usize, HidError> {
        if let Err(e) = self.set_input_mode(InputMode::Simple) {
            return Err(e);
        };
        let sub = SetHciState(HciState::Reconnect);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    fn read_spi(&self, addr: u32, length: usize) -> Result<usize, HidError> {
        let sub = ReadSpi(addr, length);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        let buf = &<Vec<u8>>::from(cmd);
        self.device.write(buf)
    }

    pub fn body_color(&self) -> (u8, u8, u8) {
        (
            self.spi_mirror[0x50],
            self.spi_mirror[0x51],
            self.spi_mirror[0x52],
        )
    }

    pub fn button_color(&self) -> (u8, u8, u8) {
        (
            self.spi_mirror[0x53],
            self.spi_mirror[0x54],
            self.spi_mirror[0x55],
        )
    }

    fn button_text(&self, btn: Button, text: &'static str) -> String {
        if self.has(btn) {
            text.to_string()
        } else {
            " ".repeat(text.chars().count())
        }
    }
}

impl Has<Button> for Driver {
    fn has(&self, btn: Button) -> bool {
        match self.frames.back() {
            None => false,
            Some(frame) => frame.buttons.has(btn),
        }
    }
}

impl fmt::Display for Driver {
    /// Creates a string identifying this device, including its name and serial
    /// number, formatted with the device's physical colors
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((bdy_r, bdy_g, bdy_b), (btn_r, btn_g, btn_b)) =
            (self.body_color(), self.button_color());
        let prod_str = match self.device.get_product_string() {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => String::new(),
        };
        write!(
            f,
            "{}{}{}",
            color::Fg(color::Rgb(btn_r, btn_g, btn_b)),
            color::Bg(color::Rgb(bdy_r, bdy_g, bdy_b)),
            style::Bold,
        )
        .and_then(|_| write!(f, " {} [{}] ", prod_str, self.serial_number()))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Left, "←")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Up, "↑")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Down, "↓")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Right, "→")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Y, "Y")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::X, "X")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::B, "B")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::A, "A")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::L, "L")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Zl, "ZL")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::R, "R")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Zr, "ZR")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Cl, "CL")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Cr, "CR")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Minus, "-")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Plus, "+")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Home, "Home")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Capture, "Cap")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Sl, "SL")))
        .and_then(|_| write!(f, " {} ", self.button_text(Button::Sr, "SR")))
        .and_then(|_| write!(f, "{}", style::Reset))
    }
}
