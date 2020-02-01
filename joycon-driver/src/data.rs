pub mod raw;

pub struct StickFrame(f32, f32);

pub struct AccFrame(f32, f32, f32);

pub struct GyroFrame(f32, f32, f32);

pub enum InputFrame {
    JoyconLeft {
        south: bool,
        north: bool,
        east: bool,
        west: bool,

        sl: bool,
        sr: bool,

        l: bool,
        zl: bool,

        minus: bool,
        click: bool,
        capture: bool,

        charge_grip: bool,

        stick: StickFrame,

        acc: AccFrame,
        gyro: GyroFrame,
    },
    JoyconRight {
        y: bool,
        x: bool,
        b: bool,
        a: bool,

        sl: bool,
        sr: bool,

        r: bool,
        zr: bool,

        plus: bool,
        click: bool,
        home: bool,

        charge_grip: bool,

        stick: StickFrame,

        acc: AccFrame,
        gyro: GyroFrame,
    },
}

pub struct FlashMirror([u8; 32768]);
