//! This Module contains all the public facing Errors that can occur when using this driver
use crate::{hal, types::api_commands::PowerDataObject};

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
    /// This can occur when sending a Power Request and the arguments to the function are not correct, these are checked before transmitting
    /// a PD Request Message
    InvalidRequest(RequestError),
    /// This can occur when there is another device on the bus using the same I2C Address. Specifically the u8 returns the value
    /// thats supposed to be the command version of the device.
    WrongCommandVersion(u8), // The value stored at the command version location
    /// This can occur when the device has not booted correctly Or the device is already initialised. If this is the case it
    /// could be solved by performing a `hard reset` followed by unplugging both the Stemma Connector if using the RotoPD and the USB C PD Device
    InitialisationFailure,
    /// This is a preemptive error that can occur when the user tries to negotiate with the device to use a Power Data Object that is not detected
    /// Inside this error contains the Power Data Object that was not detected
    PowerDataObjectNotDetected(PowerDataObject),
}

/// This Error is specifically an internal error that is used before communication with the device is taken.
/// The eror enum catches incompatible configurations and notifies the user accordingly.
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RequestError {
    MissingArgument,
    VoltageOutOfRange,
    CurrentOutOfRange,
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
            Ap33772sError::InitialisationFailure => write!(f, "Failed to initialise correctly!"),
            Ap33772sError::InvalidRequest(err) => write!(f, "Invalid request: {err:?}"),
            Ap33772sError::PowerDataObjectNotDetected(power_data_object) => {
                write!(
                    f,
                    "Power Data Object not detected on source: {power_data_object:?}"
                )
            }
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Ap33772sError {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AP33772S Error: {}",
            match self {
                Ap33772sError::I2c(err) => defmt::write!(f, "I2C error: {:?}", err),
                Ap33772sError::ConversionFailed => defmt::write!(f, "Conversion error"),
                Ap33772sError::DataMalformed(value) =>
                    defmt::write!(f, "Malformed Data error: {:?}", value),
                Ap33772sError::WrongCommandVersion(value) => {
                    defmt::write!(
                        f,
                        "Device not found. Raw value at command version location: {:?}",
                        value
                    )
                }
                Ap33772sError::InvalidRequest => defmt::write!(f, "Invalid request"),
            }
        );
    }
}
