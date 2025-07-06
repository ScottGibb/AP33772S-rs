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
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
mod hal {
    pub use embedded_hal_async::i2c::Error;
    pub use embedded_hal_async::i2c::ErrorKind;
    pub use embedded_hal_async::i2c::I2c;
}
#[derive(PartialEq, Clone, Debug)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))] #TODO: Why not this one?
#[non_exhaustive]
pub enum Ap33772sError {
    InvalidCommand,
    I2c(hal::ErrorKind),
    ConversionFailed,
    DataMalformed,
    DeviceNotFound,
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
            Ap33772sError::DeviceNotFound => write!(f, "Device not found"),
            Ap33772sError::WrongCommandVersion => write!(f, "Wrong command version"),
        }
    }
}
