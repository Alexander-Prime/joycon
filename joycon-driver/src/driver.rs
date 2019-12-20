use arraydeque::{ArrayDeque, Wrapping};
use async_std::sync::{channel, Receiver, Sender};
use async_std::task;

use crate::device::frame::InputFrame;
use crate::device::Device;
use crate::service::Service;

// Offset in SPI memory that `spi_mirror` begins at
const SPI_ORIGIN: u16 = 0x6000;

const PENDING_LEDS: u8 = 0b1111_0000;

pub struct Driver {
    device: Device,
    frames: ArrayDeque<[InputFrame; 32], Wrapping>,
    // TODO see if there's a nice way to store generic Futures here
    service_tasks: Vec<task::JoinHandle<()>>,
}

impl Driver {
    pub fn for_serial_number(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder::new(serial_number)
    }

    pub async fn start(&self) {}
}

pub struct DriverBuilder {
    serial_number: &'static str,
    services: Vec<Box<dyn Service + Send + Sync>>,
}

impl DriverBuilder {
    fn new(serial_number: &'static str) -> DriverBuilder {
        DriverBuilder {
            serial_number,
            services: Vec::new(),
        }
    }

    pub fn with<T: Service + Send + Sync + 'static>(mut self, service: T) -> Self {
        self.services.push(Box::new(service));
        self
    }

    pub fn build(self) -> Driver {
        let DriverBuilder {
            serial_number,
            services,
        } = self;

        let device = Device::new(serial_number);

        let (sender, receiver) = channel(32);
        let channel = DriverChannel(sender, receiver);

        let service_tasks = services
            .iter()
            .map(|svc| svc.start_service(&channel))
            .collect();

        Driver {
            device,
            frames: ArrayDeque::new(),
            service_tasks,
        }
    }
}

pub struct DriverChannel(Sender<()>, Receiver<()>);
