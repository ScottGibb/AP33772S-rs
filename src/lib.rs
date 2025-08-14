//! This crate provides an interface for the AP33772S I2C device. It supports both synchronous and asynchronous operations.
//! It is designed to be used in embedded systems, and it can be compiled for both `no_std` and `std` environments.
//! There are two modes the driver can run in normal and advanced mode.
//! Advanced mode can be enabled by setting the `advanced` feature flag. This allows full access to the underlying registers and exposes
//! the full functionality of the AP33772S device. The normal mode is a simplified version
//! that provides a higher-level interface for common operations.
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
#[cfg_attr(docsrs, doc(cfg(feature = "advanced")))]
#[cfg(feature = "advanced")]
pub mod commands;
#[cfg_attr(docsrs, doc(cfg(feature = "advanced")))]
#[cfg(feature = "advanced")]
pub mod communications;

#[cfg(not(feature = "advanced"))]
mod commands;
#[cfg(not(feature = "advanced"))]
mod communications;

#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
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

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
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
