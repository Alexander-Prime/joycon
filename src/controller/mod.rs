pub mod product;
pub mod vendor;

use std::cell::Cell;

use hidapi::{HidApi, HidDevice};

use endian::u32_to_le_array;
use log;

use self::product::Product;
use self::vendor::Vendor;

lazy_static! {
    static ref API: HidApi = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            log::wtf("Couldn't initialize HidApi");
            panic!(e);
        }
    };
}

const SUBCOMMAND_HEADER: [u8; 10] = [
    0x01, // Main command: rumble + subcommand
    //----
    0xF0, // Counter position, replace this before writing
    0x00, // Neutral L rumble
    0x01,
    0x40,
    0x40,
    0x00, // Neutral R rumble
    0x01,
    0x40,
    0x40,
];

pub struct JoyCon<'a> {
    device: HidDevice<'a>,
    body_color: [u8; 3],
    button_color: [u8; 3],
    serial_number: String,
    rumble_counter: Cell<u8>,
}

impl<'a> JoyCon<'a> {
    pub fn find(product: Product) -> Result<JoyCon<'a>, &'a str> {
        match API.open(Vendor::Nintendo as u16, product as u16) {
            Ok(device) => JoyCon::from_device(device),
            Err(e) => {
                log::e(e);
                Err(e)
            }
        }
    }

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
        let result = self.do_subcommand(0x30, &[bitmask]);
        self.device.read(&mut [0; 16]).unwrap(); // Ignore response
        result
    }

    fn do_subcommand(&self, subcommand_code: u8, arguments: &[u8]) -> Result<usize, &'static str> {
        let mut command = Vec::from(&mut SUBCOMMAND_HEADER[..]);
        command[1] = self.rumble_counter.get();
        command.push(subcommand_code);
        command.extend_from_slice(&arguments);

        self.inc_counter();

        self.device.write(command.as_slice())
    }

    fn read_spi(&self, addr: u32, buffer: &mut [u8]) -> Result<usize, &str> {
        let mut args = Vec::from(&mut u32_to_le_array(addr)[..]);

        args.push(buffer.len() as u8);
        if let Err(e) = self.do_subcommand(0x10, &args[..]) {
            return Err(e);
        }

        let mut response = Vec::new();
        response.resize(20 + buffer.len(), 0);
        if let Err(e) = self.device.read(response.as_mut_slice()) {
            return Err(e);
        }

        let start = response.len() - buffer.len();
        buffer.copy_from_slice(&response[start..]);

        log::i(&format!("read_spi @ 0x{:04x}: {}", addr, log::buf(&buffer)));

        Ok(1)
    }

    fn inc_counter(&self) {
        let counter = self.rumble_counter.get();
        self.rumble_counter.set((counter + 1) % 0xf);
    }
}
