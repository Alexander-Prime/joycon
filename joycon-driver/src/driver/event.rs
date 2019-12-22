use crate::device::frame::InputFrame;

pub enum DriverEvent {
  Frame(InputFrame),
  Disconnect,
}
