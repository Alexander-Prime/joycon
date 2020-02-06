use bitflags::bitflags;

bitflags! {
    pub struct JoyconRightButtons: u8 {
        const Y = 0x1;
        const X = 0x2;
        const B = 0x4;
        const A = 0x8;

        const SR = 0x10;
        const SL = 0x20;
        const R = 0x40;
        const ZR = 0x80;
    }
}

bitflags! {
    pub struct SharedButtons: u8 {
        const MINUS = 0x1;
        const PLUS = 0x2;
        const CR = 0x4;
        const CL = 0x8;

        const HOME = 0x10;
        const CAPTURE = 0x20;
        const CHARGE_GRIP = 0x80;
    }
}

bitflags! {
    pub struct JoyconLeftButtons: u8 {
        const SOUTH = 0x1;
        const NORTH = 0x2;
        const EAST = 0x4;
        const WEST = 0x8;

        const SR = 0x10;
        const SL = 0x20;
        const L = 0x40;
        const ZL = 0x80;
    }
}

bitflags! {
    pub struct BatteryStatus: u8 {
        const CHARGING = 0b0001_0000;
        const LEVEL = 0b1110_0000;
    }
}

impl BatteryStatus {
    fn level(&self) -> u8 {
        (self.bits & Self::LEVEL.bits) >> 5
    }
}

pub struct InputReport<'a>(pub &'a [u8; 360]);

impl InputReport<'_> {
    pub const TYPE_SUBCOMMAND_REPLY: u8 = 0x21;
    pub const TYPE_STANDARD_INPUT: u8 = 0x30;
    pub const TYPE_SIMPLE_INPUT: u8 = 0x3f;

    pub fn report_type(&self) -> u8 {
        self.0[0]
    }
}

pub struct OutputReport<'a>(pub &'a [u8; 48]);

impl OutputReport<'_> {
    pub const TYPE_SUBCOMMAND: u8 = 0x10;
}
