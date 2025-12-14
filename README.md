# unishock

Rust crate to interface with various shocker devices including PiShock and OpenShock.

[![crates.io][Crate Logo]][Crate]
[![Documentation][Doc Logo]][Doc]

Currently this crate only supports serial communication for PiShock hubs and nothing else. Other ways to interact with the devices and OpenShock support is planned.

All functionality supported by the PiShock serial port API is implemented.

## Examples

Beep a connected PiShock shocker:

```rust
let ports = PiShockHub::available_ports()?;
let port = ports.first().expect("no PiShock hubs found");
let mut hub = PiShockHub::connect(port, Duration::MAX)?;

let shocker_id = 1337; // From the hub info/website.
let duration = 100;    // In milliseconds.
hub.beep(shocker_id, duration)?;
```

Retrieve current PiShock hub state:

```rust
let ports = PiShockHub::available_ports()?;
let port = ports.first().expect("no PiShock hubs found");
let mut hub = PiShockHub::connect(port, Duration::MAX)?;

println!("{:#?}", hub.info()?);
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in unishock by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

[Crate]: https://crates.io/crates/unishock
[Crate Logo]: https://img.shields.io/crates/v/unishock.svg
[Doc]: https://docs.rs/unishock
[Doc Logo]: https://docs.rs/unishock/badge.svg
