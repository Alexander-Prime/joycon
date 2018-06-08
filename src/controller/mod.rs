pub mod axis;
pub mod button;
pub mod hid;
pub mod id;
pub mod state;

use std::cell::Cell;

use hidapi::{HidApi, HidDevice};
use termion::{color::*, style::{*, Reset as Clear}};

use log;

use self::id::{ProductId, VendorId};

use self::axis::ControllerAxis as Axis;
use self::button::ControllerButton as Button;
use self::hid::InputMode;
use self::hid::output::{Command::*, OutputReport::*, NEUTRAL_RUMBLE};
use self::state::ControllerState;

lazy_static! {
    static ref API: HidApi = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            log::wtf("Couldn't initialize HidApi");
            panic!(e);
        }
    };
}

pub struct JoyCon<'a> {
    device: HidDevice<'a>,
    body_color: (u8, u8, u8),
    button_color: (u8, u8, u8),
    serial_number: String,
    rumble_counter: Cell<u8>,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: [u8; 360],

    state: ControllerState,
}

impl<'a> JoyCon<'a> {
    /// Constructs a new JoyCon for the first device matching the given product
    /// ID
    pub fn find(product: ProductId) -> Result<JoyCon<'a>, &'a str> {
        match API.open(VendorId::Nintendo as u16, product as u16) {
            Ok(device) => JoyCon::from_device(device),
            Err(e) => {
                log::e(e);
                Err(e)
            }
        }
    }

    /// Constructs a new JoyCon for the device matching the given serial number
    pub fn from_serial(serial: &str) -> Result<JoyCon<'a>, &'a str> {
        for dev in API.devices().iter() {
            match &dev.serial_number {
                Some(s) if s.eq(serial) => {
                    let device = match API.open_serial(dev.vendor_id, dev.product_id, serial) {
                        Ok(dev) => dev,
                        Err(e) => return Err(e),
                    };
                    return JoyCon::from_device(device);
                }
                _ => continue,
            }
        }

        log::e(&format!("Couldn't find device with serial [{}]", serial));

        Err("Couldn't find device")
    }

    /// Receive an input packet, read its input report code, and handle the rest
    /// of its data appropriately. Callers cannot access this data directly;
    /// instead, the data is saved to the controller's state and can be read
    /// after `handle_input()` returns.
    pub fn handle_input(&mut self) -> Result<usize, &'a str> {
        self.device.read(&mut self.read_buffer[..])
    }

    /// Creates a string identifying this device, including its name and serial
    /// number, formatted with the device's physical colors
    pub fn identify(&self) -> String {
        let (bdy_r, bdy_g, bdy_b) = self.body_color();
        let (btn_r, btn_g, btn_b) = self.button_color();
        String::from(format!(
            "{}{}{} {} [{}] {}",
            Fg(Rgb(btn_r, btn_g, btn_b)),
            Bg(Rgb(bdy_r, bdy_g, bdy_b)),
            Bold,
            self.device.get_product_string().unwrap(),
            self.serial_number(),
            Clear
        ))
    }

    /// Creates a string representing the current input status, formatted with
    /// the device's physical colors
    pub fn input_str(&self) -> &str {
        let (bdy_r, bdy_g, bdy_b) = self.body_color();
        let (btn_r, btn_g, btn_b) = self.button_color();
        // format!(" {}< {}v {}^ {}> "); // Left, down, up right (for now)
        ""
    }

    fn button_state_color(&self, btn: Button) -> (u8, u8, u8) {
        if self.state.button(btn) {
            (0, 0xff, 0)
        } else {
            self.button_color()
        }
    }

    fn axis_state_color(&self, axis: Axis) -> (u8, u8, u8) {
        (0, 0, 0)
    }

    fn from_device(device: HidDevice) -> Result<JoyCon, &str> {
        let serial = match device.get_serial_number_string() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let mut jc = JoyCon {
            device: device,
            rumble_counter: Cell::new(0),
            body_color: (0x22, 0x22, 0x22),
            button_color: (0x44, 0x44, 0x44),
            serial_number: serial,

            read_buffer: [0; 360],

            state: ControllerState::new(),
        };

        jc.set_input_mode(InputMode::Simple);

        let mut colors = Vec::from(&[0; 6][..]);
        jc.read_spi(0x6050, &mut colors[..]).unwrap();

        jc.body_color = (colors[0], colors[1], colors[2]);
        jc.button_color = (colors[3], colors[4], colors[5]);

        Ok(jc)
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

    pub fn set_leds(&self, bitmask: u8) -> Result<usize, &str> {
        let sub = SetLeds(bitmask);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&cmd.make_buffer())
    }

    pub fn set_input_mode(&mut self, mode: InputMode) -> Result<usize, &str> {
        let sub = SetInputMode(mode);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device
            .write(&cmd.make_buffer())
            .and_then(|c| self.handle_input())
    }

    fn read_spi(&self, addr: u32, buffer: &mut [u8]) -> Result<usize, &str> {
        let sub = ReadSpi(addr, buffer.len());
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);

        let cmd_buf = cmd.make_buffer();

        let result = self.device.write(&cmd_buf);
        if let Err(_) = result {
            return result;
        }

        let mut response = Vec::new();
        // 4 extra response bytes
        response.resize(4 + cmd_buf.len() + buffer.len(), 0);
        let result = self.device.read(response.as_mut_slice());
        if let Err(_) = result {
            return result;
        }

        let start = response.len() - buffer.len();
        buffer.copy_from_slice(&response[start..]);

        log::i(&format!("read_spi @ 0x{:04x}: {}", addr, log::buf(&buffer)));

        result
    }
}
