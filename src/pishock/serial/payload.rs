//! Data sent over the serial port.

use std::io::Write;

use sealed::sealed;
use serde::Serialize;

/// PiShock serial protocol command metadata.
///
/// Specifies the command ID.
///
/// # Implementing
///
/// Currently, this trait may not be implemented by third parties. The trait is
/// sealed in order to make changes in the future as the design may not be final.
/// There is generally no need to implement this trait outside of the original crate.
///
/// If a command is missing, please open an issue on GitHub.
#[sealed(pub(in crate::pishock::serial))]
pub trait SerialCommand: Serialize {
  const ID: &'static str;
}

#[derive(Serialize)]
struct SerialPayload<T> {
  #[serde(rename = "cmd")]
  id: &'static str,

  #[serde(rename = "value")]
  data: T,
}

/// Serialize the payload to a writer.
pub fn serialize<T: SerialCommand>(mut writer: impl Write, command: T) -> Result<(), std::io::Error> {
  if std::mem::size_of::<T>() != 0 {
    serde_json::to_writer(&mut writer, &SerialPayload { id: T::ID, data: command })?;
    writer.write_all(b"\n")?;
  }
  else {
    writer.write_fmt(format_args!("{{\"cmd\":\"{}\"}}\n", T::ID))?;
  }

  return Ok(());
}
