#[derive(Copy, Clone, Debug)]
pub enum Product {
    JoyconLeft,
    JoyconRight,
    ProController,
    ChargeGrip,
    Unknown(u16),
}

impl Product {
    pub fn from_product_id(product_id: u16) -> Self {
        use Product::*;
        match product_id {
            0x2006 => JoyconLeft,
            0x2007 => JoyconRight,
            0x2009 => ProController,
            0x200e => ChargeGrip,
            ______ => Unknown(______),
        }
    }

    /// Device's self-reported type, from a response to subcommand 0x02
    pub fn from_device_type(device_type: u8) -> Self {
        use Product::*;
        match device_type {
            0x01 => JoyconLeft,
            0x02 => JoyconRight,
            0x03 => ProController,
            ____ => Unknown(0),
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

pub enum Vendor {
    Nintendo,
    Unknown(u16),
}

impl Vendor {
    pub fn from_vendor_id(vendor_id: u16) -> Self {
        use Vendor::*;
        match vendor_id {
            0x057e => Nintendo,
            ______ => Unknown(______),
        }
    }
}

impl Default for Vendor {
    fn default() -> Self {
        Self::Unknown(0)
    }
}
