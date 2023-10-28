//! This is an embedded-hal device driver for the MPL3115A2 Altitude sensor.
//!
//! It is accurate to around 0.3m.
//!
//! There are also temperature and pressure readings available.
//!
//! The pressure mode is likely wildly inaccurate and needs to be tested against a known good pressure sensor.

#![no_std]

mod reg;

use core::fmt::Debug;

#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2c(E),
    /// Failed to parse sensor data (Not yet used)
    InvalidData,
    /// Chip ID doesn't match the expected value
    UnsupportedChip,
}

pub mod device_impl;

/// Device Mode
///
/// Useful for "non" blocking measurements
#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Inactive,
    Active,
    TakingReading,
}

/// Pressure or Altitude Mode
///
/// Toggle as required
#[derive(Copy, Clone, PartialEq)]
pub enum PressureAlt {
    Pressure,
    Altitude,
}

pub use device_impl::MPL3115A2;

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("Cannot enable both blocking and async features");
