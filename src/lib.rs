//! # AP33772S Rust Driver
//!
//! This crate provides a comprehensive interface for the **AP33772S** USB-C Power Delivery (PD) controller
//! by Diodes Incorporated. It supports both synchronous and asynchronous operations and is designed
//! for use in embedded systems with `no_std` environments.
//!
//! ## Key Features
//!
//! - **USB-C Power Delivery 3.1 compliant** - Full support for PD negotiation and protocols
//! - **Standard Power Range (SPR)** - 5V, 9V, 12V, 15V up to 3A/5A
//! - **Extended Power Range (EPR)** - Up to 28V at 5A for higher power applications  
//! - **Programmable Power Supply (PPS)** - Adjustable voltage supply in 3.3V-21V range
//! - **Adjustable Voltage Supply (AVS)** - Extended range adjustable supply 15V-28V
//! - **Protection features** - Over voltage, current, temperature protection with configurable thresholds
//! - **Thermal management** - Configurable thermal resistances and derating capabilities
//! - **Cross-platform** - Works with any embedded-hal compatible I2C implementation
//!
//! ## Operating Modes
//!
//! The driver supports two operational modes:
//!
//! ### Normal Mode (Default)
//! Provides a high-level API with convenient getter and setter methods for common operations.
//! This mode is suitable for most applications and abstracts away low-level register details.
//!
//! ### Advanced Mode  
//! Enable with the `advanced` feature flag to gain full access to underlying registers and
//! all device functionality. This mode exposes the complete command interface for specialized
//! applications requiring fine-grained control.
//!
//! ## Feature Flags
//!
//! - `sync` - Enables synchronous I2C operations using embedded-hal
//! - `async` - Enables asynchronous I2C operations using embedded-hal-async  
//! - `advanced` - Exposes low-level commands and communication modules
//! - `interrupts` - Enables interrupt-driven operation instead of delay-based timing
//! - `defmt` - Adds defmt logging support for debugging
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ap33772s_rs::{Ap33772s, types::units::*};
//!
//! // Create device instance (sync mode)
//! let mut device = Ap33772s::new(i2c, delay);
//!
//! // Get current device statistics
//! let stats = device.get_statistics().await?;
//! println!("Voltage: {:.2}V, Current: {:.2}A",
//!          stats.voltage.get::<volt>(),
//!          stats.current.get::<ampere>());
//!
//! // Negotiate power delivery
//! device.negotiate_power_delivery(
//!     PowerDataObject::One,
//!     Some(ElectricPotential::new::<volt>(12.0)),
//!     OperatingCurrentSelection::Maximum
//! ).await?;
//! ```
//!
//! ## Related Documentation
//!
//! - [AP33772S Datasheet](https://www.diodes.com/part/view/AP33772S/)
//! - [USB Power Delivery Specification](https://www.usb.org/documents)
//! - [GitHub Repository](https://github.com/ScottGibb/AP33772S-rs) - Examples and development setup
#![no_std]
#![deny(unsafe_code)]

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

pub mod ap33772s;
mod errors;
mod getters;
mod setters;
pub mod types;

// Expose all underlying registers and communication methods for full access
#[cfg(feature = "advanced")]
pub mod commands;
#[cfg(feature = "advanced")]
pub mod communications;

#[cfg(not(feature = "advanced"))]
mod commands;
#[cfg(not(feature = "advanced"))]
mod communications;

/// Sync Based HAL Imports
#[cfg(feature = "sync")]
mod hal {
    pub use embedded_hal::delay::DelayNs;
    #[cfg(feature = "interrupts")]
    pub use embedded_hal::digital::InputPin;
    pub use embedded_hal::i2c::Error;
    pub use embedded_hal::i2c::ErrorKind;
    pub use embedded_hal::i2c::I2c;
    pub use embedded_hal::i2c::SevenBitAddress;
}

/// Aysnc Based HAL Imports
#[cfg(feature = "async")]
mod hal {
    #[cfg(feature = "interrupts")]
    pub use embedded_hal::digital::InputPin;
    pub use embedded_hal_async::delay::DelayNs;
    pub use embedded_hal_async::i2c::Error;
    pub use embedded_hal_async::i2c::ErrorKind;
    pub use embedded_hal_async::i2c::I2c;
    pub use embedded_hal_async::i2c::SevenBitAddress;
}
