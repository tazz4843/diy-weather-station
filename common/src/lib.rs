#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod config;
#[cfg(feature = "std")]
pub mod influx_payloads;
pub mod json_payloads;

#[macro_use]
extern crate serde_derive;
