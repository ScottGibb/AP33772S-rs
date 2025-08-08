//! This module outlines the AP33772S device. Specifically the top level methods and structure of the device.
//!
use super::hal::*;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;
use crate::hal;

/// Represents the AP33772S device.
/// It provides methods for interacting with the device over I2C.
/// See The [GitHub Repo](https://github.com/ScottGibb/AP33772S-rs) for examples on how to use the API.
pub struct Ap33772s<I2C: I2c> {
    pub(crate) i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    pub const ADDRESS: SevenBitAddress = 0x52;

    /// Creates a new instance of the AP33772S device. This Instance has no initialisation with the I2C bus.
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Creates a new instance of the AP33772S device and checks if the device is present on the bus.
    /// TODO: Integrate Setting of Thermal Resistance and Thresholds matching RotoPD Board
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c);
        device.is_device_present().await?;
        // TODO: Initialize Thermal Resistances and Thresholds
        Ok(device)
    }

    /// Checks if the device is present on the I2C bus. It checks an command register of the device and matches with the expected value.
    #[maybe_async::maybe_async]
    pub async fn is_device_present(&mut self) -> Result<(), Ap33772sError> {
        let system_control = self.read_one_byte_command::<SystemControl>().await?;
        system_control
            .command_version()
            .map_err(|raw_command_version| {
                Ap33772sError::WrongCommandVersion(raw_command_version)
            })?;
        Ok(())
    }

    /// Performs a hard reset on the device.
    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await
    }
}

/// Represents the different errors that can occur while interacting with the AP33772S device.
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
pub enum Ap33772sError {
    /// Represents an I2C Error this is specifcally a low level bus communication error
    I2c(hal::ErrorKind),
    /// Represents a conversion error, this can happen if the data being converted is in the wrong scale/format
    ConversionFailed,
    /// Represents a data malformed error, this can happen if the data being received is
    /// not in the expected format. Usuaully will occur if a reserved bit is being used and
    /// the enum cannot represent the state correctly. The u8 inside the error represents the value that was not expected
    DataMalformed(u8),
    /// This can occur when sending a Power Request and the arguments to the function are not correct
    InvalidRequest,
    /// This can occur when there is another device on the bus using the same I2C Address. Specifically the u8 returns the value
    /// thats supposed to be the command version of the device.
    WrongCommandVersion(u8), // The value stored at the command version location
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
            Ap33772sError::ConversionFailed => write!(f, "Conversion error"),
            Ap33772sError::DataMalformed(_value) => write!(f, "Malformed Data error"),
            Ap33772sError::WrongCommandVersion(value) => {
                write!(
                    f,
                    "Device not found. Raw value at command version location: {value}"
                )
            }
            Ap33772sError::InvalidRequest => write!(f, "Invalid request"),
        }
    }
}
