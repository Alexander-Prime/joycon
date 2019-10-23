#[derive(Copy, Clone, Debug)]
pub enum Product {
    JoyConL = 0x2006,
    JoyConR = 0x2007,
    ProController = 0x2009,
    ChargeGrip = 0x200E,
}

impl Product {
    /// Device's self-reported type, from a response to subcommand 0x02
    pub fn from_device_type(device_type: u8) -> Option<Product> {
        match device_type {
            0x01 => Some(Product::JoyConL),
            0x02 => Some(Product::JoyConR),
            0x03 => Some(Product::ProController),
            _ => None,
        }
    }
}

pub enum Vendor {
    Nintendo = 0x057E,
}
