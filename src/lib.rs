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
pub mod getters;
pub mod setters;

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
    pub use embedded_hal::i2c::Error;
    pub use embedded_hal::i2c::ErrorKind;
    pub use embedded_hal::i2c::I2c;
    pub use embedded_hal::i2c::SevenBitAddress;
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
mod hal {
    pub use embedded_hal_async::i2c::Error;
    pub use embedded_hal_async::i2c::ErrorKind;
    pub use embedded_hal_async::i2c::I2c;
    pub use embedded_hal_async::i2c::SevenBitAddress;
}
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Ap33772sError {
    InvalidCommand,
    I2c(hal::ErrorKind),
    ConversionFailed,
    DataMalformed,
    DeviceNotFound(u8), // The value stored at the command version location
    WrongCommandVersion,
}

impl<E: hal::Error> From<E> for Ap33772sError {
    fn from(e: E) -> Self {
        Ap33772sError::I2c(e.kind())
    }
}

// Allows Error Bubbling when working with both std and no-std rust
impl core::error::Error for Ap33772sError {}

impl core::fmt::Display for Ap33772sError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Ap33772sError::I2c(err) => write!(f, "I2C error: {err:?}"),
            Ap33772sError::InvalidCommand => write!(f, "Invalid command"),
            Ap33772sError::ConversionFailed => write!(f, "Conversion error"),
            Ap33772sError::DataMalformed => write!(f, "Malformed Data error"),
            Ap33772sError::DeviceNotFound(value) => {
                write!(
                    f,
                    "Device not found. Raw value at command version location: {value}"
                )
            }
            Ap33772sError::WrongCommandVersion => write!(f, "Wrong command version"),
        }
    }
}
