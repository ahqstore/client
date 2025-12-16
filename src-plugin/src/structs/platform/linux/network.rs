use crate::structs::platform::common::ConnectionType;

pub fn get_network() -> Option<ConnectionType> {
  return Some(ConnectionType::Unmetered)
}