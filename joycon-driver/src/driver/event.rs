use crate::device::frame::InputFrame;
use crate::input::InputReport;

pub enum DriverEvent {
  BasicFrame(InputReport),
  Frame(InputFrame),
  BatteryUpdate(u8),
  Disconnect,
}
