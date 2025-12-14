//! Interface for communicating with PiShock devices.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::time::Duration;
//! use unishock::pishock::serial::hub::PiShockHub;
//!
//! let ports = PiShockHub::available_ports()?;
//! let port = ports.first().expect("no PiShock hubs found");
//! let mut hub = PiShockHub::connect(port, Duration::MAX)?;
//! # return Ok(());
//! # }
//! ```

use std::{collections::VecDeque, time::Duration};

use serialport::{DataBits, SerialPort, SerialPortInfo, SerialPortType};

use crate::pishock::serial::{
  commands::{AddNetwork, Connect, Info, Operate, RemoveNetwork, Restart, ShockerOperation},
  error::{Result, SerialError},
  payload::{self, SerialCommand},
  response::TerminalInfo,
};

pub const PISHOCK_VID: u16 = 0x1A86;
pub const PISHOCK_PID: u16 = 0x7523;
pub const PISHOCK_BAUD: u32 = 115_200;

/// Represents a PiShock hub connected via serial port.
///
/// Communicate with the hub using provided inherent methods, send commands with
/// [`PiShockHub::send_command`], or even raw data directly through the serial
/// port using [`PiShockHub::send_raw`].
pub struct PiShockHub {
  serial_port: Box<dyn SerialPort>,
  buffer: Vec<u8>,
  line_queue: VecDeque<String>,
  delimiters: (bool, bool),
}

impl PiShockHub {
  /// Connect to PiShock hub using provided serial port.
  pub fn connect(port: &SerialPortInfo, timeout: Duration) -> Result<Self> {
    if !matches!(port.port_type, SerialPortType::UsbPort(..)) {
      return Err(SerialError::InvalidPortType);
    }

    let serial_port = serialport::new(&port.port_name, PISHOCK_BAUD) //
      .data_bits(DataBits::Eight)
      .timeout(timeout)
      .open()?;

    return Ok(PiShockHub {
      serial_port,
      buffer: Vec::new(),
      line_queue: VecDeque::new(),
      delimiters: (false, false),
    });
  }

  /// List available PiShock hubs connected via serial port.
  pub fn available_ports() -> Result<Vec<SerialPortInfo>, serialport::Error> {
    return Ok(serialport::available_ports()?.into_iter().filter(is_pishock_serial).collect());
  }
}

impl PiShockHub {
  /// Send raw data through the serial port.
  pub fn send_raw(&mut self, data: &[u8]) -> Result<()> {
    self.serial_port.write_all(data)?;

    return Ok(());
  }

  /// Send a known PiShock command through the serial port.
  pub fn send_command<T: SerialCommand>(&mut self, command: T) -> Result<()> {
    payload::serialize(&mut self.serial_port, command)?;

    return Ok(());
  }

  /// Connect to a network using the provided SSID and password without saving it.
  pub fn connect_network(&mut self, ssid: impl AsRef<str>, password: impl AsRef<str>) -> Result<()> {
    let (ssid, password) = (ssid.as_ref(), password.as_ref());

    self.send_command(Connect { ssid, password })?;

    return Ok(());
  }

  /// Add a new network to the saved networks and restart the hub.
  pub fn add_network(&mut self, ssid: impl AsRef<str>, password: impl AsRef<str>) -> Result<()> {
    let (ssid, password) = (ssid.as_ref(), password.as_ref());

    self.send_command(AddNetwork { ssid, password })?;

    return Ok(());
  }

  /// Remove a network from the saved networks.
  pub fn remove_network(&mut self, ssid: impl AsRef<str>) -> Result<()> {
    let ssid = ssid.as_ref();

    self.send_command(RemoveNetwork { ssid })?;

    return Ok(());
  }

  /// Restart the hub.
  pub fn restart(&mut self) -> Result<()> {
    self.send_command(Restart)?;

    return Ok(());
  }

  /// Get information about the hub.
  ///
  /// Blocks the current thread indefinitely until the hub responds.
  pub fn info(&mut self) -> Result<TerminalInfo> {
    self.send_command(Info)?;

    loop {
      let line = self.read_line()?;

      if let Some(terminal_info) = parse_terminal_info(&line) {
        return terminal_info;
      }
    }
  }
}

impl PiShockHub {
  /// Instruct the hub to shock the specified shocker.
  ///
  /// Intensity should be between 0 and 100. Duration is provided in milliseconds.
  pub fn shock(&mut self, shocker_id: u32, intensity: u8, duration: u32) -> Result<()> {
    self.send_command(Operate {
      shocker_id,
      operation: ShockerOperation::Shock,
      intensity,
      duration,
    })?;

    return Ok(());
  }

  /// Instruct the hub to vibrate the specified shocker.
  ///
  /// Intensity should be between 0 and 100. Duration is provided in milliseconds.
  pub fn vibrate(&mut self, shocker_id: u32, intensity: u8, duration: u32) -> Result<()> {
    self.send_command(Operate {
      shocker_id,
      operation: ShockerOperation::Vibrate,
      intensity,
      duration,
    })?;

    return Ok(());
  }

  /// Instruct the hub to beep the specified shocker.
  ///
  /// Duration is provided in milliseconds.
  pub fn beep(&mut self, shocker_id: u32, duration: u32) -> Result<()> {
    self.send_command(Operate {
      shocker_id,
      operation: ShockerOperation::Beep,
      intensity: 0,
      duration,
    })?;

    return Ok(());
  }

  /// Instruct the hub to stop the specified shocker.
  ///
  /// If the shocker is performing any action, it will be stopped.
  pub fn stop(&mut self, shocker_id: u32) -> Result<()> {
    self.send_command(Operate {
      shocker_id,
      operation: ShockerOperation::End,
      intensity: 0,
      duration: 0,
    })?;

    return Ok(());
  }
}

impl PiShockHub {
  /// Read a line from the hub.
  ///
  /// Blocks the current thread indefinitely until the hub responds.
  pub fn read_line(&mut self) -> Result<String> {
    loop {
      let mut read_buffer = [0; 32];

      let read_length = self.serial_port.read(&mut read_buffer)?;

      for &c in &read_buffer[.. read_length] {
        if c == b'\r' {
          self.delimiters.0 = true;
          continue;
        }

        if c == b'\n' {
          self.delimiters.1 = true;
          continue;
        }

        if self.delimiters.0 && self.delimiters.1 {
          self.delimiters = (false, false);

          let line = String::from_utf8(self.buffer.drain(..).collect())?;

          // Put the line in the queue.
          //
          // Can't just return because need to read to the end, otherwise risk
          // skipping data. Storing only a single line will result in skipped
          // lines if the `read_buffer` contains multiple lines.
          self.line_queue.push_back(line);
        }

        self.delimiters = (false, false);
        self.buffer.push(c);
      }

      // Return if a line has been read.
      if let Some(line) = self.line_queue.pop_front() {
        return Ok(line);
      }
    }
  }
}

impl std::io::Write for PiShockHub {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    return self.serial_port.write(buf);
  }

  fn flush(&mut self) -> std::io::Result<()> {
    return self.serial_port.flush();
  }
}

/// Attempt to parse the terminal info from raw line output string.
pub fn parse_terminal_info(output: &str) -> Option<Result<TerminalInfo>> {
  if let Some(json) = output.strip_prefix("TERMINALINFO: ") {
    return Some(serde_json::from_str::<TerminalInfo>(json).map_err(|_| SerialError::InvalidResponse));
  }

  return None;
}

/// Check if the given serial port is a PiShock hub.
fn is_pishock_serial(port_type: &SerialPortInfo) -> bool {
  if let SerialPortType::UsbPort(info) = &port_type.port_type {
    // On macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*) ports are enumerated. We only want Dial-in.
    let macos_check = cfg!(not(target_os = "macos")) || port_type.port_name.starts_with("/dev/tty.");

    return info.vid == PISHOCK_VID && info.pid == PISHOCK_PID && macos_check;
  }

  return false;
}
