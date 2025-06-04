//! This is a placeholder crate to reserve the `ap33772` namespace.
//! Actual functionality will be published in a future release.
#![no_std]
#![deny(unsafe_code)]

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

pub mod ap33772s;
pub mod getters;
pub mod setters;

#[cfg(feature = "advanced")]
pub mod commands;
#[cfg(feature = "advanced")]
pub mod communications;

#[cfg(not(feature = "advanced"))]
mod commands;
#[cfg(not(feature = "advanced"))]
mod communications;

#[cfg(feature = "sync")]
mod hal {
    pub use embedded_hal::i2c::ErrorKind;
    pub use embedded_hal::i2c::I2c;
    pub use embedded_hal::i2c::Error;
}

#[cfg(feature = "async")]
mod hal {
    pub use embedded_hal_async::i2c::ErrorKind;
    pub use embedded_hal_async::i2c::I2c;
    pub use embedded_hal_async::i2c::Error;
}
#[derive(PartialEq, Clone, Debug)]
pub enum Ap33772sError {
    InvalidCommand,
    I2c(hal::ErrorKind),
    ConversionError,
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
            Ap33772sError::I2c(err) => write!(f, "I2C error: {:?}", err),
            Ap33772sError::InvalidCommand => write!(f, "Invalid command"),
            Ap33772sError::ConversionError => write!(f, "Conversion error"),
        }
    }
}
