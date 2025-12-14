//! PiShock serial API error types.

use thiserror::Error;

pub type Result<T, E = SerialError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum SerialError {
  #[error("IO failure: {0}")]
  IoFailure(#[from] std::io::Error),

  #[error("IO failure: {0}")]
  FromUtf8Failure(#[from] std::string::FromUtf8Error),

  #[error("Serial failure: {0}")]
  SerialPortFailure(#[from] serialport::Error),

  #[error("Invalid PiShock hub response received")]
  InvalidResponse,

  #[error("Invalid port type")]
  InvalidPortType,
}
