use crate::data::InputFrame;

pub enum DriverEvent {
  Frame(InputFrame),
  BatteryUpdate(u8),
  Disconnect,
}
