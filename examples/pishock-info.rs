use std::{error::Error, time::Duration};

use unishock::pishock::serial::hub::PiShockHub;

fn main() -> Result<(), Box<dyn Error>> {
  let ports = PiShockHub::available_ports()?;
  let port = ports.first().expect("no PiShock hubs found");
  let mut hub = PiShockHub::connect(port, Duration::MAX)?;

  println!("{:#?}", hub.info()?);

  return Ok(());
}
