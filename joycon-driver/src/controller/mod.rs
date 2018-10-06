pub mod axis;
pub mod button;
pub mod hid;
pub mod id;
pub mod motion;

use std::cell::Cell;
use std::fmt;

use hidapi::{HidApi, HidDevice};
use termion::{color, style};

use log;

use self::axis::StickFrame;
use self::button::ButtonFrame;
use self::hid::input::{InputReport, ResponseData, SpiChunk};
use self::hid::output::{Command::*, OutputReport::*, NEUTRAL_RUMBLE};
use self::hid::{HciState, InputMode};
use self::id::{ProductId, VendorId};
use self::motion::MotionFrame;

lazy_static! {
    static ref API: HidApi = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            log::wtf("Couldn't initialize HidApi");
            panic!(e);
        }
    };
}

pub struct Controller<'a> {
    device: HidDevice<'a>,
    body_color: (u8, u8, u8),
    button_color: (u8, u8, u8),
    serial_number: String,
    rumble_counter: Cell<u8>,
    leds: Cell<u8>,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],

    latest_frame: InputFrame,
}

impl<'a> Controller<'a> {
    /// Constructs a new Controller for the first device matching the given product ID
    pub fn find(product: ProductId) -> Result<Controller<'a>, &'a str> {
        match API.open(VendorId::Nintendo as u16, product as u16) {
            Ok(device) => Controller::from_device(device),
            Err(e) => {
                log::e(e);
                Err(e)
            }
        }
    }

    /// Constructs a new Controller for the device matching the given serial number
    pub fn from_serial(serial: &str) -> Result<Controller<'a>, &'a str> {
        for dev in API.devices().iter() {
            match &dev.serial_number {
                Some(s) if s.eq(serial) => {
                    let device = match API.open_serial(dev.vendor_id, dev.product_id, serial) {
                        Ok(dev) => dev,
                        Err(e) => return Err(e),
                    };
                    return Controller::from_device(device);
                }
                _ => continue,
            }
        }

        log::e(&format!("Couldn't find device with serial [{}]", serial));

        Err("Couldn't find device")
    }

    fn from_device(device: HidDevice) -> Result<Controller, &str> {
        let serial = match device.get_serial_number_string() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        if let Err(e) = device.set_blocking_mode(false) {
            return Err(e);
        }

        let jc = Controller {
            device: device,
            rumble_counter: Cell::new(0),
            body_color: (0x22, 0x22, 0x22),
            button_color: (0x44, 0x44, 0x44),
            serial_number: serial,
            leds: Cell::new(0x00),

            read_buffer: [0; 360],

            latest_frame: InputFrame::new(),
        };

        jc.set_input_mode(InputMode::Simple);
        jc.read_spi(0x6050, 3);
        jc.read_spi(0x6053, 3);

        Ok(jc)
    }

    /// Read and handle all buffered inputs. Blocks until the queue is emptied.
    /// On success, returns `Ok(len)`, where `len` is the number of inputs that were flushed.
    pub fn flush(&mut self) -> Result<usize, &'a str> {
        let mut count = 0;
        loop {
            match self.handle_input() {
                Ok(None) => return Ok(count),
                Err(e) => return Err(e),
                _ => count += 1,
            }
        }
    }

    /// Receive an input packet, read its input report code, and handle the rest
    /// of its data appropriately. Callers cannot access this data directly;
    /// instead, the data is saved to the controller's state and can be read
    /// after `handle_input()` returns.
    fn handle_input(&mut self) -> Result<Option<usize>, &'a str> {
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
                buttons,
                left_stick,
                right_stick,
                data,
            } => {
                self.latest_frame.buttons = buttons;
                self.latest_frame.left_stick = left_stick;
                self.latest_frame.right_stick = right_stick;
                self.handle_response(data);
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
        match chunk {
            SpiChunk::BodyColor(r, g, b) => {
                self.body_color = (r, g, b);
            }
            SpiChunk::ButtonColor(r, g, b) => {
                self.button_color = (r, g, b);
            }
            SpiChunk::Unknown(addr, len) => log::e(&format!(
                "Read unknown SPI data, {} bytes at 0x{:04x}",
                len, addr
            )),
        }
    }

    pub fn body_color(&self) -> (u8, u8, u8) {
        self.body_color
    }

    pub fn button_color(&self) -> (u8, u8, u8) {
        self.button_color
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    pub fn set_leds(&self, bitmask: u8) -> Result<usize, &'a str> {
        if bitmask == self.leds.replace(bitmask) {
            return Ok(0);
        }
        let sub = SetLeds(bitmask);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    pub fn set_input_mode(&self, mode: InputMode) -> Result<usize, &'a str> {
        let sub = SetInputMode(mode);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    pub fn reset(&self) -> Result<usize, &'a str> {
        self.set_input_mode(InputMode::Simple);
        let sub = SetHciState(HciState::Reconnect);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&<Vec<u8>>::from(cmd))
    }

    fn read_spi(&self, addr: u32, length: usize) -> Result<usize, &str> {
        let sub = ReadSpi(addr, length);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        let buf = &<Vec<u8>>::from(cmd);
        self.device.write(buf)
    }
}

impl<'a> fmt::Display for Controller<'a> {
    /// Creates a string identifying this device, including its name and serial
    /// number, formatted with the device's physical colors
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (bdy_r, bdy_g, bdy_b) = self.body_color();
        let (btn_r, btn_g, btn_b) = self.button_color();
        let prod_str = match self.device.get_product_string() {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        write!(
            f,
            "{}{}{} {} [{}] {}",
            color::Fg(color::Rgb(btn_r, btn_g, btn_b)),
            color::Bg(color::Rgb(bdy_r, bdy_g, bdy_b)),
            style::Bold,
            prod_str,
            self.serial_number(),
            style::Reset
        )
    }
}

pub struct InputFrame {
    buttons: ButtonFrame,
    left_stick: StickFrame,
    right_stick: StickFrame,
    motion: MotionFrame,
}

impl InputFrame {
    fn new() -> InputFrame {
        InputFrame {
            buttons: ButtonFrame::new(),
            left_stick: StickFrame::new(),
            right_stick: StickFrame::new(),
            motion: MotionFrame::new(),
        }
    }
}
