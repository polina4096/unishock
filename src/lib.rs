//! Interact with remote-controlled PiShock shockers, or other OpenShock-supported devices.
//!
//! `unishock` implements various protocols and supports multiple interfaces for working with remote-controlled shocker
//! hardware. As of now, the library only works with PiShock devices over serial port, however there are plans to extend
//! support for different PiShock APIs and implement compatibility with OpenShock.
//!
//! # Supported platforms
//!
//!  * Linux
//!  * Windows
//!  * macOS

pub mod pishock;
