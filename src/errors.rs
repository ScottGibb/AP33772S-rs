//! # Error Types for AP33772S Driver
//!
//! This module contains all error types that can occur when using the AP33772S driver.
//! The errors are designed to provide clear information about what went wrong and how
//! to potentially resolve the issue.
//!
//! ## Error Categories
//!
//! ### Communication Errors
//! - [`Ap33772sError::I2c`] - Low-level I2C bus communication failures
//!
//! ### Data Errors  
//! - [`Ap33772sError::ConversionFailed`] - Value conversion/scaling problems
//! - [`Ap33772sError::DataMalformed`] - Invalid data received from device
//!
//! ### Request Validation Errors
//! - [`Ap33772sError::InvalidRequest`] - Power delivery request parameter validation
//!
//! ### Device Errors
//! - [`Ap33772sError::WrongCommandVersion`] - Device not found or wrong device type
//! - [`Ap33772sError::InitialisationFailure`] - Device failed to initialize properly
//! - [`Ap33772sError::PowerDataObjectNotDetected`] - Requested PDO not available
//!
//! ## Error Handling Strategies
//!
//! ```rust,no_run
//! use ap33772s_rs::{Ap33772sError, errors::RequestError};
//!
//! async fn handle_errors(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) {
//!     match device.get_status() {
//!         Ok(status) => println!("Status: {:?}", status),
//!         Err(Ap33772sError::I2c(_)) => {
//!             // I2C communication problem - check connections
//!             eprintln!("I2C communication failed - check wiring");
//!         },
//!         Err(Ap33772sError::WrongCommandVersion(version)) => {
//!             // Wrong device or device not responding
//!             eprintln!("Device not found or wrong type. Got version: 0x{:02X}", version);
//!         },
//!         Err(Ap33772sError::DataMalformed(value)) => {
//!             // Device returned unexpected data
//!             eprintln!("Device returned invalid data: 0x{:02X}", value);
//!         },
//!         Err(other) => eprintln!("Other error: {}", other),
//!     }
//! }
//! ```
use crate::{hal, types::command_structures::PowerDataObject};

/// Comprehensive error type for all AP33772S driver operations.
///
/// This enum covers all possible error conditions that can occur when communicating
/// with and controlling the AP33772S device. Each variant provides specific information
/// about the type of failure to help with debugging and error handling.
///
/// # Error Categories
///
/// ## Communication Errors
/// These errors indicate problems with the I2C communication link to the device.
///
/// ## Data Processing Errors  
/// These errors occur when data cannot be properly converted or is in an unexpected format.
///
/// ## Request Validation Errors
/// These errors are caught before sending commands to the device, indicating invalid parameters.
///
/// ## Device State Errors
/// These errors indicate problems with device detection, initialization, or capabilities.
///
/// # Error Recovery
///
/// Most errors are recoverable with appropriate action:
/// - **I2C errors**: Check wiring, power, and bus configuration
/// - **Conversion errors**: Verify parameter ranges and units  
/// - **Invalid requests**: Adjust parameters within valid ranges
/// - **Device errors**: Power cycle device or check connections
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
pub enum Ap33772sError {
    /// I2C bus communication error.
    ///
    /// This error indicates a low-level problem with the I2C communication link
    /// to the AP33772S device. It wraps the embedded-hal I2C error type.
    ///
    /// # Common Causes
    /// - **No device response**: Device not powered or not connected
    /// - **Bus arbitration loss**: Multiple masters on the I2C bus
    /// - **NACK response**: Device busy or address not recognized  
    /// - **Bus errors**: Short circuits, incorrect pull-ups, electrical noise
    ///
    /// # Recovery Actions
    /// - Verify device power supply (3.3V or 5V)
    /// - Check I2C wiring (SDA, SCL, GND connections)
    /// - Verify pull-up resistors (typically 4.7kΩ)
    /// - Check for bus conflicts with other devices
    /// - Reduce I2C clock speed if experiencing noise issues
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::Ap33772sError;
    /// match error {
    ///     Ap33772sError::I2c(i2c_error) => {
    ///         eprintln!("I2C communication failed: {:?}", i2c_error);
    ///         // Check hardware connections
    ///     },
    ///     _ => {},
    /// }
    /// ```
    I2c(hal::ErrorKind),

    /// Data conversion or scaling error.
    ///
    /// This error occurs when a value cannot be properly converted between different
    /// units or scales, typically when the input value is outside the valid range
    /// for the target format.
    ///
    /// # Common Causes
    /// - **Voltage out of range**: Requesting voltage outside device capabilities
    /// - **Current out of range**: Requesting current beyond limits  
    /// - **Temperature out of range**: Invalid temperature values
    /// - **Scaling overflow**: Value too large for device register format
    ///
    /// # Recovery Actions
    /// - Check parameter values against device specifications
    /// - Verify units are correct (volts vs millivolts, etc.)
    /// - Consult datasheet for valid ranges
    /// - Use appropriate scaling factors
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772sError, units::*};
    /// // This might cause ConversionFailed if voltage is too high
    /// let result = device.set_minimum_selection_voltage(
    ///     ElectricPotential::new::<volt>(50.0) // Too high!
    /// );
    ///
    /// if let Err(Ap33772sError::ConversionFailed) = result {
    ///     eprintln!("Voltage value out of range");
    /// }
    /// ```
    ConversionFailed,

    /// Device returned malformed or unexpected data.
    ///
    /// This error indicates that the AP33772S returned data that doesn't match
    /// the expected format or contains reserved bit patterns that cannot be
    /// properly interpreted by the driver.
    ///
    /// # Common Causes
    /// - **Reserved bits set**: Device using newer firmware with different bit meanings
    /// - **Communication corruption**: Noise causing bit errors
    /// - **Device malfunction**: Internal device error or undefined state
    /// - **Timing issues**: Reading registers during state transitions
    ///
    /// # Debugging
    /// The contained `u8` value shows the raw data that caused the error, which
    /// can help identify bit patterns or values that weren't expected.
    ///
    /// # Recovery Actions
    /// - Retry the operation after a brief delay
    /// - Check device datasheet for register format changes
    /// - Verify I2C signal integrity
    /// - Power cycle the device to reset state
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::Ap33772sError;
    /// match error {
    ///     Ap33772sError::DataMalformed(raw_value) => {
    ///         eprintln!("Invalid data from device: 0x{:02X}", raw_value);
    ///         // Log for debugging, potentially retry
    ///     },
    ///     _ => {},
    /// }
    /// ```
    DataMalformed(u8),

    /// Power delivery request validation error.
    ///
    /// This error is generated during validation of power delivery request parameters
    /// before the request is sent to the device. It prevents invalid requests that
    /// would be rejected by the source or could cause unsafe operation.
    ///
    /// # Request Error Types
    /// See [`RequestError`] for specific validation failure reasons:
    /// - Missing required parameters (voltage for variable PDOs)
    /// - Voltage outside PDO range
    /// - Current exceeding PDO capability
    ///
    /// # Prevention
    /// - Use [`get_all_source_power_capabilities`] to check available PDOs
    /// - Verify voltage ranges for variable PDOs
    /// - Check current limits before requesting
    /// - Use appropriate PDO selection for your requirements
    ///
    /// [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
    InvalidRequest(RequestError),

    /// Device not found or wrong device type detected.
    ///
    /// This error occurs during device detection when the command version register
    /// doesn't contain the expected value for an AP33772S device. This usually
    /// indicates a hardware or addressing problem.
    ///
    /// # Common Causes
    /// - **Wrong I2C address**: Using incorrect device address
    /// - **Different device**: Another device at the same address
    /// - **Device not responding**: Power, reset, or communication issues
    /// - **Bus conflicts**: Multiple devices with same address
    ///
    /// # Device Address
    /// The AP33772S uses I2C address 0x51 (7-bit) or 0xA2/0xA3 (8-bit read/write).
    ///
    /// # Debugging
    /// The contained `u8` value shows what was read from the command version
    /// register, which can help identify what device is actually responding.
    ///
    /// # Recovery Actions
    /// - Verify correct I2C address configuration
    /// - Check device power and reset signals
    /// - Scan I2C bus to identify active devices
    /// - Verify device part number and compatibility
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::Ap33772sError;
    /// match error {
    ///     Ap33772sError::WrongCommandVersion(version) => {
    ///         eprintln!("Expected AP33772S but found device with version: 0x{:02X}", version);
    ///         // Check I2C address and device type
    ///     },
    ///     _ => {},
    /// }
    /// ```
    WrongCommandVersion(u8),

    /// Device initialization failure.
    ///
    /// This error indicates that the AP33772S failed to initialize properly
    /// during the startup sequence. This can happen if the device is already
    /// initialized, powered incorrectly, or has internal faults.
    ///
    /// # Common Causes
    /// - **Already initialized**: Device was previously configured and is running
    /// - **Incorrect power sequencing**: Device powered before proper initialization
    /// - **Hardware faults**: Internal device problems
    /// - **Timing violations**: Not waiting for device boot sequence
    ///
    /// # Recovery Actions
    /// - Perform hard reset of the device
    /// - Power cycle both device and any connected USB-C devices
    /// - Verify power supply stability during startup
    /// - Check for proper boot timing delays
    /// - If using RotoPD board, disconnect both Stemma connector and USB-C device
    ///
    /// # Initialization Sequence
    /// Proper initialization requires:
    /// 1. Device power-on
    /// 2. Wait for internal boot sequence (typically 100ms)
    /// 3. Check device presence and version
    /// 4. Configure thermal resistances and thresholds
    /// 5. Enable desired power delivery modes
    InitialisationFailure,

    /// Requested Power Data Object not available from source.
    ///
    /// This error occurs when attempting to negotiate with a Power Data Object (PDO)
    /// that is not advertised by the connected USB-C Power Delivery source.
    ///
    /// # Common Causes
    /// - **PDO not supported**: Source doesn't offer the requested power level
    /// - **Cable limitations**: Cable cannot support requested power (especially EPR)
    /// - **Source limitations**: Source doesn't support advanced PD features
    /// - **Stale capabilities**: Source capabilities changed since last query
    ///
    /// # Prevention
    /// Always check available capabilities before making requests:
    /// 1. Call [`get_all_source_power_capabilities`]
    /// 2. Verify the desired PDO is present and valid
    /// 3. Check PDO type and capabilities match requirements
    /// 4. Use appropriate fallback PDOs if primary choice unavailable
    ///
    /// # Recovery Actions
    /// - Query current source capabilities
    /// - Select from available PDOs
    /// - Check cable and source specifications
    /// - Consider using lower power PDOs as fallback
    ///
    /// [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
    PowerDataObjectNotDetected(PowerDataObject),
}

/// Specific validation errors for Power Delivery requests.
///
/// This error type provides detailed information about why a power delivery request
/// was rejected during validation before being sent to the device. These errors help
/// identify parameter issues and guide correction of the request.
///
/// # Validation Process
///
/// Power delivery requests are validated against:
/// - Available Power Data Object (PDO) capabilities
/// - Voltage ranges for variable PDOs
/// - Current limits for all PDO types  
/// - Required vs optional parameters
///
/// # Error Recovery
///
/// Each error type suggests specific remediation:
/// - Check PDO capabilities with [`get_all_source_power_capabilities`]
/// - Adjust voltage/current within valid ranges
/// - Provide required parameters for variable PDOs
///
/// [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RequestError {
    /// Required parameter is missing from the request.
    ///
    /// This error occurs when a power delivery request is missing a parameter
    /// that is required for the specified Power Data Object (PDO) type.
    ///
    /// # Common Scenarios
    /// - **Variable PDOs**: Voltage selection parameter is required but was `None`
    /// - **PPS/AVS PDOs**: Voltage must be specified within the adjustable range
    /// - **Battery PDOs**: Power specification may be required
    ///
    /// # Resolution
    /// - For Fixed PDOs: Set voltage selection to `None`
    /// - For Variable/PPS/AVS PDOs: Provide `Some(voltage)` within the PDO's range
    /// - Check PDO type with [`source_power_type()`] to determine requirements
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// // This would cause MissingArgument for a PPS PDO
    /// let result = device.send_power_delivery_request(
    ///     PowerDataObject::Seven, // Assume this is PPS
    ///     None, // ERROR: PPS requires voltage selection!
    ///     OperatingCurrentSelection::Maximum,
    ///     &capabilities
    /// );
    /// ```
    MissingArgument,

    /// Requested voltage is outside the valid range for the PDO.
    ///
    /// This error occurs when the requested voltage exceeds either the minimum
    /// or maximum voltage supported by the specified Power Data Object.
    ///
    /// # Voltage Validation
    /// The driver checks that:
    /// ```text
    /// PDO.min_voltage ≤ requested_voltage ≤ PDO.max_voltage
    /// ```
    ///
    /// # Common Causes
    /// - **Too low**: Requesting voltage below PDO minimum (rare)
    /// - **Too high**: Requesting voltage above PDO maximum (common)
    /// - **Wrong PDO**: Selected wrong PDO for desired voltage
    /// - **Unit confusion**: Volts vs millivolts mix-up
    ///
    /// # Resolution
    /// 1. Get PDO voltage range: `pdo.get_min_voltage()` and `pdo.get_max_voltage()`
    /// 2. Adjust requested voltage to be within range
    /// 3. Or select a different PDO that supports the desired voltage
    ///
    /// # Example Ranges
    /// - **Fixed 5V PDO**: Only supports exactly 5V
    /// - **PPS PDO**: Might support 3.3V - 21V adjustable
    /// - **AVS PDO**: Might support 15V - 28V adjustable
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// // Check PDO range first
    /// let pdo = capabilities.get_power_data_object(PowerDataObject::Seven);
    /// let max_voltage = pdo.get_max_voltage()?;
    ///
    /// if requested_voltage <= max_voltage {
    ///     // Safe to request
    /// } else {
    ///     // Choose different voltage or PDO
    /// }
    /// ```
    VoltageOutOfRange,

    /// Requested current exceeds the PDO's maximum capability.
    ///
    /// This error occurs when the requested operating current is higher than
    /// the maximum current that the Power Data Object can provide.
    ///
    /// # Current Validation
    /// The driver checks that:
    /// ```text
    /// requested_current ≤ PDO.max_current
    /// ```
    ///
    /// # Common Causes
    /// - **Overestimating capacity**: Requesting more current than PDO supports
    /// - **Wrong PDO selection**: Selected low-power PDO for high-power application
    /// - **Cable limitations**: Cable cannot support requested current (especially >3A)
    /// - **Source limitations**: Source hardware cannot provide requested current
    ///
    /// # Resolution
    /// 1. Check PDO current capability: `pdo.get_max_current()`
    /// 2. Reduce requested current to within limits
    /// 3. Select higher-power PDO if available
    /// 4. Verify cable rating for high-current operation (>3A requires 5A cable)
    ///
    /// # Current Considerations
    /// - **Standard USB-C cables**: Limited to 3A maximum
    /// - **5A rated cables**: Required for >3A operation (electronically marked)
    /// - **Power calculation**: P = V × I, higher voltage allows lower current for same power
    ///
    /// # Example
    /// ```rust,no_run
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// // Check current capability
    /// let max_current = pdo.get_max_current();
    /// let requested_current = ElectricCurrent::new::<ampere>(5.0);
    ///
    /// if requested_current <= max_current {
    ///     // Request is valid
    /// } else {
    ///     // Reduce current or find higher-power PDO
    ///     let safe_current = max_current;
    /// }
    /// ```
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
        use crate::hal::Error;
        use crate::hal::ErrorKind;
        defmt::write!(
            f,
            "AP33772S Error: {}",
            match self {
                Ap33772sError::I2c(err) => {
                    // Convert the ErrorKind into a string for defmt
                    let kind_str = match err.kind() {
                        ErrorKind::Bus => "Bus",
                        ErrorKind::ArbitrationLoss => "ArbitrationLoss",
                        ErrorKind::NoAcknowledge(_) => "NoAcknowledge",
                        ErrorKind::Overrun => "Overrun",
                        ErrorKind::Other => "Other",
                        _ => "Unknown",
                    };
                    defmt::write!(f, "AP33772S Error: I2C error ({})", kind_str);
                }

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
                Ap33772sError::InvalidRequest(err) =>
                    defmt::write!(f, "Invalid request: {:?}", err),
                Ap33772sError::InitialisationFailure =>
                    defmt::write!(f, "Failed to initialise correctly!"),
                Ap33772sError::PowerDataObjectNotDetected(power_data_object) => {
                    defmt::write!(
                        f,
                        "Power Data Object not detected on source: {:?}",
                        power_data_object
                    )
                }
            }
        );
    }
}
