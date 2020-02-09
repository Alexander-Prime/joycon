use crate::data::{AccelFrame, GyroFrame, InputFrame, StickFrame};
use crate::driver::DriverEvent;
use crate::handler::{Handler, HandlerResult};

pub struct ConsoleHandler;

impl ConsoleHandler {
    pub fn new() -> Self {
        ConsoleHandler
    }

    fn print_input_frame(frame: &InputFrame) {
        match frame {
            InputFrame::JoyconLeft {
                south,
                north,
                east,
                west,
                sr,
                sl,
                l,
                zl,
                minus,
                click,
                capture,
                charge_grip,
                stick,
                accel,
                gyro,
            } => {
                println!(
                    "south: {} | north: {} | east: {} | west: {}",
                    south, north, east, west
                );
                println!("sr: {} | sl: {} | l: {} | zl: {}", sr, sl, l, zl);
                println!(
                    "minus: {} | click: {} | capture: {} | charge_grip: {}",
                    minus, click, capture, charge_grip
                );

                let StickFrame(x, y) = stick;
                println!("stick: [{} {}]", x, y);

                let AccelFrame(x, y, z) = accel;
                println!("accel: [{} {} {}]", x, y, z);

                let GyroFrame(x, y, z) = gyro;
                println!("gyro: [{} {} {}]", x, y, z);
            }
            InputFrame::JoyconRight {
                y,
                x,
                b,
                a,
                sr,
                sl,
                r,
                zr,
                plus,
                click,
                home,
                charge_grip,
                stick,
                accel,
                gyro,
            } => {
                println!("y: {} | x: {} | b: {} | a: {}", y, x, b, a);
                println!("sr: {} | sl: {} | r: {} | zr: {}", sr, sl, r, zr);
                println!(
                    "plus: {} | click: {} | home: {} | charge_grip: {}",
                    plus, click, home, charge_grip
                );

                let StickFrame(x, y) = stick;
                println!("stick: [{} {}]", x, y);

                let AccelFrame(x, y, z) = accel;
                println!("accel: [{} {} {}]", x, y, z);

                let GyroFrame(x, y, z) = gyro;
                println!("gyro: [{} {} {}]", x, y, z);
            }
            InputFrame::Unknown => (),
        }
    }
}

impl Handler for ConsoleHandler {
    fn write(&mut self, event: &DriverEvent) -> HandlerResult<()> {
        match event {
            DriverEvent::Frame(frame) => Self::print_input_frame(frame),
            _________________________ => (),
        };
        Ok(())
    }
}
