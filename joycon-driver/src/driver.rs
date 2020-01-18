mod command;
mod event;

use async_std::sync::{channel, Receiver, Sender};
use async_std::task;
use hidapi::HidResult;

use crate::device::Device;
use crate::handler::Handler;
use crate::input::InputReport;
use crate::output::OutputReport;

pub use self::command::DriverCommand;
pub use self::event::DriverEvent;

pub struct Driver {
    device: Device,
    event_sender: Sender<DriverEvent>,
    command_receiver: Receiver<DriverCommand>,
}

impl Driver {
    pub fn for_serial_number(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder::new(serial_number)
    }

    pub async fn start(mut self) -> HidResult<()> {
        let report_sender = self.device.get_sender();
        let report_receiver = self.device.get_receiver();

        let command_receiver = self.command_receiver;
        let event_sender = self.event_sender;

        task::spawn(async move {
            while let Some(command) = command_receiver.recv().await {
                Self::handle_output(command, &report_sender).await;
            }
        });
        task::spawn(async move {
            while let Some(report) = report_receiver.recv().await {
                Self::handle_input(report, &event_sender).await;
            }
        });

        self.device.start().await
    }

    async fn handle_input(report: InputReport, event_sender: &Sender<DriverEvent>) {
        match report {
            InputReport::SimpleInput(buttons, stick) => {
                let e = ();
                event_sender.send(()).await
            }
            InputReport::ExtendedInput { battery, frame } => {
                let e = ();
                event_sender.send(()).await
            }
            InputReport::CommandResponse {
                battery,
                frame,
                data,
            } => {
                let e = ();
                event_sender.send(()).await
            }
            InputReport::Unknown => {}
        }
    }

    async fn handle_output(command: DriverCommand, report_sender: &Sender<OutputReport>) {}
}

pub struct DriverBuilder {
    serial_number: &'static str,
    services: Vec<Box<dyn Handler + Send + Sync>>,
}

impl DriverBuilder {
    fn new(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder {
            serial_number,
            services: Vec::new(),
        }
    }

    pub fn with<T: Handler + Send + Sync + 'static>(mut self, service: T) -> Self {
        self.services.push(Box::new(service));
        self
    }

    pub fn build(self) -> Driver {
        let DriverBuilder {
            serial_number,
            services,
        } = self;

        let device = Device::new(serial_number);

        let (command_sender, command_receiver) = channel(32);
        let (event_sender, event_receiver) = channel(32);
        let channel = DriverChannel(command_sender, event_receiver);

        for svc in services {
            svc.start(&channel);
        }

        Driver {
            device,
            event_sender,
            command_receiver,
        }
    }
}

pub struct DriverChannel(Sender<DriverCommand>, Receiver<DriverEvent>);
