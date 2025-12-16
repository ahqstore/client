use windows::Networking::Connectivity::{NetworkConnectivityLevel, NetworkCostType, NetworkInformation};

use crate::structs::platform::common::ConnectionType;

pub fn get_network() -> Option<ConnectionType> {
  let info = NetworkInformation::GetInternetConnectionProfile().ok()?;

  let cost = info.GetConnectionCost().ok()?.NetworkCostType().ok()?;
  let netlevel = info.GetNetworkConnectivityLevel().ok()?.0;

  let conn = NetworkConnectivityLevel::InternetAccess.0 == netlevel;

  if !conn {
    return Some(ConnectionType::Disconnected);
  }

  let unmetered = cost.0 == NetworkCostType::Unrestricted.0;

  if unmetered {
    Some(ConnectionType::Unmetered)
  } else {
    Some(ConnectionType::Metered)
  }
}