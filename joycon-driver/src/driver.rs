mod command;
mod event;

use hidapi::HidError;

use crate::data::calibration::Calibration;
use crate::data::raw::InputReport;
use crate::data::InputFrame;
use crate::device::Device;
use crate::handler::{Handler, HandlerError};

pub use self::command::DriverCommand;
pub use self::event::DriverEvent;

pub struct Driver {
    device: Device,
    calibration: Option<Calibration>,
    handlers: Vec<Box<dyn Handler>>,
}

impl Driver {
    pub fn for_serial_number(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder::new(serial_number)
    }

    pub fn start(mut self) -> DriverResult<()> {
        loop {
            self.handle_input()?;
            self.handle_output()?;
        }
    }

    fn handle_input<'a>(&mut self) -> DriverResult<()> {
        let product = self.device.product;
        while let Some(result) = self.device.read() {
            let report = result.map_err(|e| DriverError::ReadFailed(e))?;
            match report.report_type() {
                InputReport::TYPE_SIMPLE_INPUT => (), // TODO Convert these into fake InputFrames
                InputReport::TYPE_STANDARD_INPUT => {
                    if let Some(cal) = &self.calibration {
                        let event = DriverEvent::Frame(InputFrame::from_standard_input(
                            report, product, cal,
                        ));
                        self.send_event(event)?;
                    }
                }
                InputReport::TYPE_SUBCOMMAND_REPLY => (),
                _ => (), // TODO Generate some kind of debug event
            }
        }
        Ok(())
    }

    fn handle_output<'a>(&mut self) -> DriverResult<()> {
        for h in self.handlers.iter_mut() {
            while let Some(command) = h.read() {}
        }
        Ok(())
    }

    fn send_event(&mut self, event: DriverEvent) -> DriverResult<()> {
        for h in self.handlers.iter_mut() {
            h.write(&event).map_err(|e| DriverError::EventFailed(e))?;
        }
        Ok(())
    }
}

pub struct DriverBuilder {
    serial_number: &'static str,
    handlers: Vec<Box<dyn Handler>>,
}

impl DriverBuilder {
    fn new(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder {
            serial_number,
            handlers: Vec::new(),
        }
    }

    pub fn with<T: Handler + 'static>(mut self, service: T) -> Self {
        self.handlers.push(Box::new(service));
        self
    }

    pub fn build(self) -> DriverResult<Driver> {
        let DriverBuilder {
            serial_number,
            handlers,
        } = self;

        let device = Device::open(serial_number).map_err(|e| DriverError::BuildFailed(e))?;

        Ok(Driver {
            device,
            calibration: None,
            handlers,
        })
    }
}

pub type DriverResult<T> = Result<T, DriverError>;

pub enum DriverError {
    BuildFailed(HidError),
    CommandFailed(DriverCommand),
    ReadFailed(HidError),
    EventFailed(HandlerError),
}
