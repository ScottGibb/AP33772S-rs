//! # AP33772S USB-C Power Delivery Driver
//!
//! This crate provides a Rust driver for the AP33772S USB-C Power Delivery controller IC.
//! It supports both synchronous and asynchronous I2C operations and is designed for embedded systems
//! with `no_std` support.
//! 
//! ## Features
//! 
//! The driver supports two main operational modes controlled by feature flags:
//! 
//! ### Synchronous vs Asynchronous Operation
//! 
//! - **`sync`** (default): Uses [`embedded_hal`] traits for synchronous I2C operations
//! - **`async`**: Uses `embedded_hal_async` traits for asynchronous I2C operations
//! 
//! **Note**: These features are mutually exclusive - you must choose one.
//! 
//! ### Access Levels
//! 
//! - **Normal mode** (default): Provides a high-level API through getter/setter methods on [`Ap33772s`]
//! - **`advanced`**: Exposes low-level register access when enabled (see examples)
//! 
//! ### Additional Features
//! 
//! - **`interrupts`**: Enables interrupt pin support for asynchronous device communication
//! - **`defmt`**: Adds defmt formatting support for embedded debugging
//! 
//! ## Quick Start
//! 
//! ```toml
//! [dependencies]
//! ap33772s-rs = { version = "0.1", features = ["sync"] }
//! ```
//! 
//! ```rust,no_run
//! use ap33772s_rs::{Ap33772s, types::Statistics};
//! 
//! # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
//! // Create and initialize the driver
//! let mut ap33772s = Ap33772s::new_default(i2c, delay).await?;
//! 
//! // Read device statistics
//! let stats: Statistics = ap33772s.get_statistics().await?;
//! println!("Current: {}A, Voltage: {}V", stats.current, stats.voltage);
//! # Ok(())
//! # }
//! ```
//! 
//! ## Advanced Usage
//! 
//! Enable the `advanced` feature for direct register access:
//! 
//! ```toml
//! [dependencies]
//! ap33772s-rs = { version = "0.1", features = ["sync", "advanced"] }
//! ```
//! 
//! This exposes low-level register operations for advanced device control.
//! 
//! For more examples, see the repository's `examples/` directory and [`README.md`](https://github.com/ScottGibb/AP33772S-rs).
#![no_std]
#![deny(unsafe_code)]

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

mod ap33772s;
pub use ap33772s::Ap33772s;
mod errors;
mod getters;
mod setters;
pub mod types;
pub mod units;

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
