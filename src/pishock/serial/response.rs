//! Data received over the serial port.

use serde::{Deserialize, Serialize};

/// Information about the PiShock Hub.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalInfo {
  /// Firmware version.
  #[serde(rename = "version")]
  pub firmware_version: String,

  /// Hardware type.
  #[serde(rename = "type")]
  pub hardware_type: HardwareType,

  /// Whether the terminal is connected to the server.
  pub connected: bool,

  /// PiShock Hub id on the PiShock server.
  pub client_id: u32,

  /// WiFi SSID the PiShock Hub is connected to currently.
  #[serde(rename = "wifi")]
  pub current_ssid: String,

  /// Server address the PiShock Hub is connected to.
  pub server: String,

  /// MAC address of the PiShock Hub.
  pub mac_address: String,

  /// List of shockers connected to the PiShock Hub.
  pub shockers: Vec<ShockerInfo>,

  /// List of networks configured on the PiShock Hub.
  pub networks: Vec<NetworkInfo>,

  /// One-time key which was used for pairing.
  pub otk: String,

  /// Whether the PiShock Hub is claimed via OTK.
  pub claimed: bool,

  /// Whether the PiShock Hub is connected to a development server.
  pub is_dev: bool,

  /// Whether the PiShock Hub is connected as a publisher.
  pub publisher: bool,

  /// Whether the PiShock Hub has polled the server.
  pub polled: bool,

  /// Whether the PiShock Hub is connected as a subscriber.
  pub subscriber: bool,

  /// Public IP address of the PiShock Hub.
  pub public_ip: String,

  /// Whether the PiShock Hub has internet access.
  #[serde(rename = "internet")]
  pub internet_access: bool,

  /// The ID of the user that claimed the PiShock.
  pub owner_id: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardwareType(pub u8);

impl HardwareType {
  pub const NEXT: HardwareType = HardwareType(3);
  pub const LITE: HardwareType = HardwareType(4);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShockerInfo {
  #[serde(rename = "id")]
  pub shocker_id: u32,
  #[serde(rename = "type")]
  pub model: ShockerModel,
  pub paused: bool,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShockerModel(pub u8);

impl ShockerModel {
  pub const PETRAINER: ShockerModel = ShockerModel(0);
  pub const SMALL_ONE: ShockerModel = ShockerModel(1);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
  pub ssid: String,
  pub password: String,
}
