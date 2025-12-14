//! Commands supported by the PiShock serial API.
//!
//! All communication with the PiShock hub using the serial port is done through a set of commands. They are represented
//! as ASCII JSON strings terminated with a newline character (`\n`).
//!
//! Most commands follow the following format:
//!
//! ```json
//! {
//!   "cmd": "...",
//!   "value": {...}
//! }
//! ```
//!
//! Some have the value as a string instead of an object:
//!
//! ```json
//! {
//!   "cmd": "...",
//!   "value": "..."
//! }
//! ```
//!
//! Commands which have no meaningful data to be transferred omit the `value` field entirely:
//!
//! ```json
//! {
//!   "cmd": "..."
//! }
//! ```

use sealed::sealed;
use serde::Serialize;

use crate::pishock::serial::payload::{__seal_serial_command, SerialCommand};

macro_rules! test_command {
  ($name:ident, $cmd:expr, $expected:expr) => {
    #[cfg(test)]
    #[test]
    fn $name() -> Result<(), std::io::Error> {
      let mut payload = Vec::new();

      crate::pishock::serial::payload::serialize(&mut payload, $cmd)?;
      assert_eq!(std::str::from_utf8(&payload).expect("bad utf8 string"), concat!($expected, "\n"));

      return Ok(());
    }
  };
}

/// Connect to a network.
///
/// Attempts to connect to the given network temporarily, without saving it.
#[derive(Debug, Serialize)]
pub struct Connect<'a> {
  pub ssid: &'a str,
  pub password: &'a str,
}

#[sealed]
impl SerialCommand for Connect<'_> {
  const ID: &'static str = "connect";
}

test_command! {
  serial_cmd_connect_serialize,
  Connect {
    ssid: "test_ssid",
    password: "test_password",
  },
  r#"{"cmd":"connect","value":{"ssid":"test_ssid","password":"test_password"}}"#
}

/// Add a network.
///
/// Adds a new network to the saved networks and restarts the hub.
///
/// The PiShock will respond with [`TerminalInfo`](crate::pishock::serial::response::TerminalInfo).
#[derive(Debug, Serialize)]
pub struct AddNetwork<'a> {
  pub ssid: &'a str,
  pub password: &'a str,
}

#[sealed]
impl SerialCommand for AddNetwork<'_> {
  const ID: &'static str = "addnetwork";
}

test_command! {
  serial_cmd_addnetwork_serialize,
  AddNetwork {
    ssid: "test_ssid",
    password: "test_password",
  },
  r#"{"cmd":"addnetwork","value":{"ssid":"test_ssid","password":"test_password"}}"#
}

/// Remove a network.
///
/// Removes a network from the networks saved on the PiShock.
///
/// The PiShock will respond with [`TerminalInfo`](crate::pishock::serial::response::TerminalInfo).
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct RemoveNetwork<'a> {
  pub ssid: &'a str,
}

#[sealed]
impl SerialCommand for RemoveNetwork<'_> {
  const ID: &'static str = "removenetwork";
}

test_command! {
  serial_cmd_removenetwork_serialize,
  RemoveNetwork {
    ssid: "test_ssid",
  },
  r#"{"cmd":"removenetwork","value":"test_ssid"}"#
}

/// Reboots the PiShock.
#[derive(Debug, Serialize)]
pub struct Restart;

#[sealed]
impl SerialCommand for Restart {
  const ID: &'static str = "restart";
}

test_command! {
  serial_cmd_restart_serialize,
  Restart,
  r#"{"cmd":"restart"}"#
}

/// Requests the current state.
///
/// The PiShock will respond with [`TerminalInfo`](crate::pishock::serial::response::TerminalInfo).
#[derive(Debug, Serialize)]
pub struct Info;

#[sealed]
impl SerialCommand for Info {
  const ID: &'static str = "info";
}

test_command! {
  serial_cmd_info_serialize,
  Info,
  r#"{"cmd":"info"}"#
}

/// Requests the current state.
///
/// The PiShock will respond with [`TerminalInfo`](crate::pishock::serial::response::TerminalInfo).
#[derive(Debug, Serialize)]
pub struct Operate {
  /// Id of the shocker to operate, can be found in [`TerminalInfo`](crate::pishock::serial::response::TerminalInfo).
  #[serde(rename = "id")]
  pub shocker_id: u32,
  /// Operation to perform on the shocker.
  #[serde(rename = "op")]
  pub operation: ShockerOperation,
  /// Intensity of the operation, 0-100. Should be `0` for [`ShockerOperation::Beep`], or [`ShockerOperation::End`].
  pub intensity: u8,
  /// Duration of the operation in milliseconds. Should be `0` for [`ShockerOperation::End`].
  pub duration: u32,
}

/// All available shocker operations.
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShockerOperation {
  /// Shock the shocker.
  Shock,
  /// Vibrate the shocker.
  Vibrate,
  /// Beep the shocker.
  Beep,
  /// Stop any currently running operation.
  End,
}

#[sealed]
impl SerialCommand for Operate {
  const ID: &'static str = "operate";
}

test_command! {
  serial_cmd_operate_serialize,
  Operate {
    shocker_id: 1,
    operation: ShockerOperation::Shock,
    duration: 100,
    intensity: 10,
  },
  r#"{"cmd":"operate","value":{"id":1,"op":"shock","intensity":10,"duration":100}}"#
}
