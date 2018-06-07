pub mod axis;
pub mod button;
pub mod hid;
pub mod id;
pub mod state;

use std::cell::Cell;

use hidapi::{HidApi, HidDevice};

use log;

use self::id::{ProductId, VendorId};

use self::hid::{InputMode, output::{Command::*, OutputReport::*, NEUTRAL_RUMBLE}};

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
    body_color: [u8; 3],
    button_color: [u8; 3],
    serial_number: String,
    rumble_counter: Cell<u8>,

    // Maximum Joy-Con packet size, w/ NFC/IR data
    // Most packets won't send more than ~50 bytes
    read_buffer: Cell<[u8; 360]>,
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
    pub fn handle_input(&self) -> Result<usize, &'a str> {
        self.device.read(&mut (self.read_buffer.get())[..])
    }

    fn from_device(device: HidDevice) -> Result<JoyCon, &str> {
        let serial = match device.get_serial_number_string() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let mut jc = JoyCon {
            device: device,
            rumble_counter: Cell::new(0),
            body_color: [0x22; 3],
            button_color: [0x44; 3],
            serial_number: serial,

            read_buffer: Cell::from([0; 360]),
        };

        let mut colors = Vec::from(&[0; 6][..]);
        jc.read_spi(0x6050, &mut colors[..]).expect("");

        jc.body_color = [colors[0], colors[1], colors[2]];
        jc.button_color = [colors[3], colors[4], colors[5]];

        Ok(jc)
    }

    pub fn body_color(&self) -> [u8; 3] {
        self.body_color
    }

    pub fn button_color(&self) -> [u8; 3] {
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

    pub fn set_input_mode(&self, mode: InputMode) -> Result<usize, &str> {
        let sub = SetInputMode(mode);
        let cmd = DoCommand(self.rumble_counter.get(), &NEUTRAL_RUMBLE, sub);
        self.device.write(&cmd.make_buffer())
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

    fn inc_counter(&self) {
        let counter = self.rumble_counter.get();
        self.rumble_counter.set((counter + 1) % 0xf);
    }
}
