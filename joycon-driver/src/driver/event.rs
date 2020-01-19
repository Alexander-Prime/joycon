use crate::device::frame::InputFrame;

pub enum DriverEvent {
  Frame(InputFrame),
  BatteryUpdate(u8),
  Disconnect,
}
