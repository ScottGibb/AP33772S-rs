//! # AP33772S Device Driver Implementation
//!
//! This module contains the main device driver structure and high-level methods for the AP33772S
//! USB-C Power Delivery controller. It provides the primary interface for device initialization,
//! power delivery negotiation, and device management.
//!
//! ## Device Structure
//!
//! The [`Ap33772s`] struct is the main interface to the device, supporting two operational modes:
//!
//! ### Delay-Based Mode (Default)
//! Uses HAL-provided delays for timing-sensitive operations like power delivery negotiation.
//! This mode is suitable for most applications and provides automatic timing management.
//!
//! ### Interrupt-Based Mode  
//! Uses an interrupt pin to detect when the device is ready for communication.
//! This mode provides more efficient operation but requires additional hardware connections.
//!
//! ## Initialization Patterns
//!
//! ### Simple Initialization
//! ```rust,no_run
//! # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
//! use ap33772s_rs::Ap33772s;
//!
//! // Create device instance without initialization
//! let mut device = Ap33772s::new(i2c, delay);
//!
//! // Check if device is present
//! device.is_device_present().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Complete Initialization (Recommended)
//! ```rust,no_run
//! # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
//! use ap33772s_rs::Ap33772s;
//!
//! // Create and fully initialize device with default settings
//! let mut device = Ap33772s::new_default(i2c, delay).await?;
//!
//! // Device is now ready for power delivery operations
//! # Ok(())
//! # }
//! ```
//!
//! ## Power Delivery Operations
//!
//! The main high-level functions for power delivery management:
//!
//! - [`negotiate_power_delivery`] - Request specific power with custom parameters
//! - [`negotiate_maximum_power_delivery`] - Request maximum power from a PDO
//! - [`get_all_source_power_capabilities`] - Query available power options
//! - [`get_statistics`] - Monitor current operational state
//!
//! ## Device Management
//!
//! - [`is_device_present`] - Verify device communication
//! - [`hard_reset`] - Perform complete device reset
//! - Various getters and setters for configuration and monitoring
//!
//! [`negotiate_power_delivery`]: Ap33772s::negotiate_power_delivery
//! [`negotiate_maximum_power_delivery`]: Ap33772s::negotiate_maximum_power_delivery
//! [`get_all_source_power_capabilities`]: crate::Ap33772s::get_all_source_power_capabilities
//! [`get_statistics`]: crate::Ap33772s::get_statistics
//! [`is_device_present`]: Ap33772s::is_device_present
//! [`hard_reset`]: Ap33772s::hard_reset
#[cfg(not(feature = "interrupts"))]
use core::time::Duration;

use super::hal::*;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;

// Public API Types
use crate::types::command_structures::*;
use crate::types::*;
use crate::units::*;

/// Main driver interface for the AP33772S USB-C Power Delivery controller.
///
/// This structure provides the primary interface for controlling and monitoring the AP33772S device.
/// It encapsulates the I2C communication, timing mechanisms, and optional interrupt handling required
/// for proper device operation.
///
/// # Generic Parameters
///
/// - `I2C` - I2C peripheral implementing [`embedded_hal::i2c::I2c`] trait
/// - `D` - Delay provider implementing [`embedded_hal::delay::DelayNs`] trait
/// - `P` - Input pin for interrupt mode (when `interrupts` feature enabled)
///
/// # Operation Modes
///
/// ## Delay-Based Mode (Default)
/// Uses the provided delay implementation for timing-sensitive operations:
/// - Power delivery negotiation timing (100ms delays)
/// - Device initialization sequences
/// - Reset and startup timing
///
/// ## Interrupt-Based Mode (Feature: `interrupts`)
/// Uses an interrupt pin to detect when the device is ready:
/// - More efficient than polling delays
/// - Requires additional hardware connection
/// - Currently under development
///
/// # I2C Address
///
/// The AP33772S uses I2C address **0x52** (7-bit addressing).
///
/// # Initialization
///
/// Two initialization patterns are available:
///
/// ## Basic Initialization
/// Create a device instance without automatic configuration:
/// ```rust,no_run
/// # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
/// let mut device = Ap33772s::new(i2c, delay);
///
/// // Manual verification and configuration required
/// device.is_device_present().await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Complete Initialization (Recommended)
/// Create and fully configure a device with safe defaults:
/// ```rust,no_run
/// # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
/// let mut device = Ap33772s::new_default(i2c, delay).await?;
///
/// // Device is immediately ready for use
/// let stats = device.get_statistics().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Power Delivery Workflow
///
/// Typical power delivery negotiation workflow:
/// 1. Get available power capabilities
/// 2. Select appropriate Power Data Object (PDO)
/// 3. Negotiate power delivery
/// 4. Monitor results and statistics
///
/// ```rust,no_run
/// # use ap33772s_rs::{Ap33772s, types::{command_structures::*, units::*}};
/// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// // Step 1: Query available power capabilities
/// let capabilities = device.get_all_source_power_capabilities().await?;
///
/// // Step 2: Negotiate for 12V at maximum current
/// let response = device.negotiate_power_delivery(
///     PowerDataObject::Two, // Typically 9V or 12V
///     None, // Fixed PDO
///     OperatingCurrentSelection::Maximum,
///     &capabilities
/// ).await?;
///
/// // Step 3: Verify negotiation success
/// if response == PowerDeliveryResponse::Success {
///     println!("Power delivery negotiation successful!");
///     
///     // Step 4: Monitor the results
///     let stats = device.get_statistics().await?;
///     println!("New voltage: {:.1}V", stats.voltage.get::<volt>());
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Error Handling
///
/// All operations return [`Ap33772sError`] which provides detailed information about:
/// - I2C communication issues
/// - Device detection problems  
/// - Invalid power delivery requests
/// - Thermal or electrical protection events
///
/// # Thread Safety
///
/// This driver is **not** thread-safe. If multiple tasks need access to the device,
/// external synchronization (mutex, etc.) is required.
///
/// # Hardware Requirements
///
/// - **I2C bus**: 3.3V or 5V with appropriate pull-up resistors (typically 4.7kΩ)
/// - **Power supply**: 3.3V or 5V for the AP33772S
/// - **USB-C connector**: Properly wired with CC pins connected to AP33772S
/// - **Thermal management**: Adequate for expected power levels
///
/// # See Also
///
/// - [GitHub Examples](https://github.com/ScottGibb/AP33772S-rs/tree/main/examples) for complete usage examples
/// - [Device Datasheet](https://www.diodes.com/part/view/AP33772S/) for hardware specifications
///
/// [`Ap33772sError`]: crate::errors::Ap33772sError
pub struct Ap33772s<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> {
    pub(crate) i2c: I2C,
    /// The underlying delay mechanism required for the USB C Power Delivery negotiation
    #[cfg(not(feature = "interrupts"))]
    pub(crate) delay: D,
    /// The InputPin assigned for the Interrupt signal. This pin will go high when the AP33772S is ready for communication
    #[cfg(feature = "interrupts")]
    pub(crate) interrupt_pin: P,
}

/// This impl block represents the the initialisation methods for when no interrupts are used. This approach uses a
/// delay approach which is dependent on the users HAL
#[cfg(not(feature = "interrupts"))]
impl<I2C: I2c, D: DelayNs> Ap33772s<I2C, D> {
    /// Timing delay for power delivery negotiation operations.
    ///
    /// This delay allows the AP33772S and connected source sufficient time to process
    /// power delivery request messages and complete the negotiation protocol.
    const NEGOTIATE_TIMING_DELAY: Duration = Duration::from_millis(100);

    /// Initial boot-up delay for device initialization.
    ///
    /// This delay ensures the AP33772S has completed its internal boot sequence
    /// before attempting communication or configuration.
    const BOOT_UP_DELAY: Duration = Duration::from_millis(100);

    /// The I2C address of the AP33772S device (7-bit addressing).
    ///
    /// This address is fixed in the device hardware and is defined in the AP33772S datasheet.
    /// The device will respond to this address for all I2C communication.
    pub const ADDRESS: SevenBitAddress = 0x52;

    /// Creates a new AP33772S device instance without initialization.
    ///
    /// This constructor creates a device instance with the provided I2C peripheral and
    /// delay implementation, but does not perform any communication with the device.
    /// Use [`is_device_present`] to verify communication before proceeding.
    ///
    /// # Parameters
    ///
    /// - `i2c` - I2C peripheral for device communication
    /// - `delay` - Delay implementation for timing-sensitive operations
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::Ap33772s;
    ///
    /// let mut device = Ap33772s::new(i2c, delay);
    ///
    /// // Verify device is present before use
    /// device.is_device_present().await?;
    ///
    /// // Now safe to use device APIs
    /// let status = device.get_status().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new_default`] for complete initialization with safe defaults
    /// - [`is_device_present`] to verify device communication
    ///
    /// [`new_default`]: Self::new_default
    /// [`is_device_present`]: Self::is_device_present
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self { i2c, delay }
    }

    /// Creates and initializes an AP33772S device with safe default configuration.
    ///
    /// This function performs complete device initialization including:
    /// 1. Device presence verification
    /// 2. Status checking and boot sequence validation
    /// 3. Default thermal resistance configuration
    /// 4. Default protection threshold setup
    /// 5. Error recovery with hard reset if needed
    ///
    /// # Parameters
    ///
    /// - `i2c` - I2C peripheral for device communication  
    /// - `delay` - Delay implementation for timing operations
    ///
    /// # Returns
    ///
    /// Returns a fully initialized device ready for power delivery operations.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if I2C communication fails
    /// - [`Ap33772sError::WrongCommandVersion`] if device not found or wrong type
    /// - [`Ap33772sError::InitialisationFailure`] if device failed to initialize properly
    ///
    /// # Initialization Sequence
    ///
    /// The function follows this initialization sequence:
    /// 1. **Device Detection**: Verify AP33772S is present and responding
    /// 2. **Status Check**: Ensure device is in proper boot state
    /// 3. **Default Configuration**: Apply safe thermal and protection settings
    /// 4. **Error Recovery**: Hard reset and retry if initialization fails
    ///
    /// # Default Settings Applied
    ///
    /// - **Thermal Resistances**: Device default values optimized for typical PCB designs
    /// - **Protection Thresholds**: Conservative values suitable for most applications
    /// - **Boot Timing**: Proper delays for device startup sequence
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::Ap33772s;
    ///
    /// // Create and initialize device with defaults
    /// let mut device = Ap33772s::new_default(i2c, delay).await?;
    ///
    /// // Device is immediately ready for use
    /// let capabilities = device.get_all_source_power_capabilities().await?;
    /// println!("Found {} power data objects", capabilities.power_data_objects.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Recovery Behavior
    ///
    /// If the device is found to be in an unexpected state (already initialized or
    /// partially configured), the function will:
    /// 1. Attempt a hard reset to restore known state
    /// 2. Re-run the initialization sequence
    /// 3. Return [`Ap33772sError::InitialisationFailure`] to indicate the recovery action
    ///
    /// The device may still be usable after this error, but a full power cycle is
    /// recommended for guaranteed clean state.
    ///
    /// # Hardware Considerations
    ///
    /// **Important**: This function must be called after the device is powered on but
    /// before any Power Delivery negotiation occurs. If called on an already-running
    /// device, it may disrupt ongoing power delivery operations.
    ///
    /// For RotoPD board users: If initialization fails, disconnect both the Stemma
    /// connector and USB-C device, then reconnect to ensure clean power-on reset.
    ///
    /// # See Also
    ///
    /// - [`new`] for basic device creation without initialization
    /// - [`set_thermal_resistances`] to customize thermal settings
    /// - [`set_thresholds`] to customize protection settings
    /// - [`hard_reset`] for manual device reset
    ///
    /// [`new`]: Self::new
    /// [`set_thermal_resistances`]: crate::Ap33772s::set_thermal_resistances
    /// [`set_thresholds`]: crate::Ap33772s::set_thresholds
    /// [`hard_reset`]: Self::hard_reset
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C, delay: D) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c, delay);
        device.is_device_present().await?;

        let device_status = device.get_status().await?;
        if device_status.i2c_ready()
            && device_status.started()
            && device_status.new_power_data_object()
        {
            Self::initialise(&mut device).await?;
        } else {
            // Device May already be initialised, to do a fresh install, the user must fully power cycle the device
            device.hard_reset().await?; // This does not fully power cycle the RotoPD board due to the device being powered by the STEMMA connector
            Self::initialise(&mut device).await?;
            return Err(Ap33772sError::InitialisationFailure);
        }
        Ok(device)
    }

    /// Internal initialization helper function.
    ///
    /// Performs the actual device configuration steps including boot delay,
    /// thermal resistance setup, and protection threshold configuration.
    #[maybe_async::maybe_async]
    async fn initialise(device: &mut Self) -> Result<(), Ap33772sError> {
        device
            .delay
            .delay_ms(
                u32::try_from(Self::BOOT_UP_DELAY.as_millis())
                    .expect("This should not fail, HAL Duration Type Conversions"),
            )
            .await; // Initial delay to allow the device to power up
        device
            .set_thermal_resistances(ThermalResistances::default())
            .await?;
        device.set_thresholds(Thresholds::default()).await
    }

    /// Negotiates Power Delivery with the connected source device.
    ///
    /// This is the primary high-level function for requesting specific power delivery
    /// parameters from a USB-C Power Delivery source. It handles the complete negotiation
    /// sequence including request validation, message transmission, timing delays, and
    /// response verification.
    ///
    /// # Parameters
    ///
    /// - `power_data_object_index` - Which PDO to request (PowerDataObject::One through PowerDataObject::Thirteen)
    /// - `voltage_selection` - Desired voltage for variable PDOs (`None` for fixed PDOs)
    /// - `current_selection` - Operating current selection mode
    /// - `data_objects` - Available source capabilities from [`get_all_source_power_capabilities`]
    ///
    /// # Power Data Object Selection
    ///
    /// PDOs are typically arranged by the source in order of preference:
    /// - **PDO 1**: Usually 5V (USB default)
    /// - **PDO 2-4**: Common fixed voltages (9V, 12V, 15V, 20V)
    /// - **PDO 5-7**: Extended Power Range or PPS/AVS if supported
    ///
    /// Use [`get_all_source_power_capabilities`] to determine what's actually available.
    ///
    /// # Voltage Selection Guidelines
    ///
    /// ## Fixed PDOs
    /// Set `voltage_selection` to `None` - voltage is predetermined by the PDO.
    ///
    /// ## Variable/PPS/AVS PDOs  
    /// Specify desired voltage within the PDO's range:
    /// ```rust,no_run
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// # async fn example() {
    /// // For a PPS PDO supporting 3.3V-21V, request 12.5V
    /// let voltage = Some(ElectricPotential::new::<volt>(12.5));
    /// # }
    /// ```
    ///
    /// # Current Selection Options
    ///
    /// - [`OperatingCurrentSelection::Maximum`] - Request maximum available current from PDO
    /// - [`OperatingCurrentSelection::_1A`] - Request minimum operating current
    /// - Custom values (check PDO capabilities first)
    ///
    /// # Returns
    ///
    /// Returns a [`PowerDeliveryResponse`] indicating the source's response:
    /// - [`PowerDeliveryResponse::Success`] - Request accepted, power delivery active
    /// - [`PowerDeliveryResponse::NotSupported`] - Request rejected by source
    /// - [`PowerDeliveryResponse::Busy`] - Source busy, retry later
    /// - Other responses per USB PD specification
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::PowerDataObjectNotDetected`] - Requested PDO not available from source
    /// - [`Ap33772sError::InvalidRequest`] - Invalid voltage/current parameters
    /// - [`Ap33772sError::I2c`] - Communication failure during negotiation
    ///
    /// # Timing Behavior
    ///
    /// This function includes automatic timing management:
    /// 1. Validates request parameters
    /// 2. Sends power delivery request message
    /// 3. Waits 100ms for source processing
    /// 4. Reads and returns the response
    ///
    /// The delay is handled by the provided HAL delay implementation and may be
    /// blocking or non-blocking depending on the HAL.
    ///
    /// # Usage Examples
    ///
    /// ## Fixed Voltage Request (Most Common)
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::{command_structures::*, units::*}};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get available capabilities
    /// let capabilities = device.get_all_source_power_capabilities().await?;
    ///
    /// // Request 12V (typically PDO 3) at maximum current
    /// let response = device.negotiate_power_delivery(
    ///     PowerDataObject::Three,
    ///     None, // Fixed PDO - no voltage selection
    ///     OperatingCurrentSelection::Maximum,
    ///     &capabilities
    /// ).await?;
    ///
    /// match response {
    ///     PowerDeliveryResponse::Success => {
    ///         println!("Power delivery negotiation successful!");
    ///         let stats = device.get_statistics().await?;
    ///         println!("New voltage: {:.1}V", stats.voltage.get::<volt>());
    ///     },
    ///     PowerDeliveryResponse::NotSupported => {
    ///         println!("Power request rejected by source");
    ///     },
    ///     _ => println!("Unexpected response: {:?}", response),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Variable Voltage Request (PPS/AVS)
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::{command_structures::*, units::*}};
    /// # async fn pps_example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// # let capabilities = device.get_all_source_power_capabilities().await?;
    /// // Request 12.5V from a PPS PDO (assuming PDO 7 is PPS)
    /// let response = device.negotiate_power_delivery(
    ///     PowerDataObject::Seven,
    ///     Some(ElectricPotential::new::<volt>(12.5)), // Custom voltage
    ///     OperatingCurrentSelection::Maximum,
    ///     &capabilities
    /// ).await?;
    ///
    /// if response == PowerDeliveryResponse::Success {
    ///     println!("PPS voltage set to 12.5V");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Best Practices
    ///
    /// ## Power Capability Verification
    /// Always query capabilities first to ensure the desired PDO exists:
    /// ```rust,no_run
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// # async fn verify_example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let capabilities = device.get_all_source_power_capabilities().await?;
    /// let pdo = capabilities.get_power_data_object(PowerDataObject::Three);
    ///
    /// if pdo.is_detected() {
    ///     let max_voltage = pdo.get_max_voltage()?;
    ///     println!("PDO 3 supports up to {:.1}V", max_voltage.get::<volt>());
    /// } else {
    ///     println!("PDO 3 not available from this source");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Error Handling
    /// Handle different error types appropriately:
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772sError, errors::RequestError};
    /// # async fn error_example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut device: ap33772s_rs::Ap33772s<Box<dyn embedded_hal::i2c::I2c>, Box<dyn embedded_hal::delay::DelayNs>> = todo!();
    /// # let capabilities = device.get_all_source_power_capabilities().await?;
    /// match device.negotiate_power_delivery(/* parameters */).await {
    ///     Ok(response) => println!("Response: {:?}", response),
    ///     Err(Ap33772sError::PowerDataObjectNotDetected(pdo)) => {
    ///         println!("PDO {:?} not available, try a different one", pdo);
    ///     },
    ///     Err(Ap33772sError::InvalidRequest(RequestError::VoltageOutOfRange)) => {
    ///         println!("Voltage out of range, check PDO limits");
    ///     },
    ///     Err(other) => eprintln!("Negotiation failed: {}", other),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Advanced Usage
    ///
    /// For applications requiring lower-level control or custom timing, consider using
    /// the `advanced` feature to access [`send_power_delivery_request`] and
    /// [`get_power_delivery_request_result`] separately.
    ///
    /// # See Also
    ///
    /// - [`negotiate_maximum_power_delivery`] for simplified maximum power requests
    /// - [`get_all_source_power_capabilities`] to query available PDOs
    /// - [`get_statistics`] to monitor negotiation results
    /// - [`send_power_delivery_request`] for advanced low-level control
    ///
    /// [`PowerDeliveryResponse`]: crate::types::command_structures::PowerDeliveryResponse
    /// [`PowerDeliveryResponse::Success`]: crate::types::command_structures::PowerDeliveryResponse::Success
    /// [`PowerDeliveryResponse::NotSupported`]: crate::types::command_structures::PowerDeliveryResponse::NotSupported
    /// [`PowerDeliveryResponse::Busy`]: crate::types::command_structures::PowerDeliveryResponse::Busy
    /// [`OperatingCurrentSelection::Maximum`]: crate::types::command_structures::OperatingCurrentSelection::Maximum
    /// [`OperatingCurrentSelection::_1A`]: crate::types::command_structures::OperatingCurrentSelection::_1A
    /// [`get_all_source_power_capabilities`]: crate::Ap33772s::get_all_source_power_capabilities
    /// [`negotiate_maximum_power_delivery`]: Self::negotiate_maximum_power_delivery
    /// [`get_statistics`]: crate::Ap33772s::get_statistics
    /// [`send_power_delivery_request`]: crate::Ap33772s::send_power_delivery_request
    /// [`get_power_delivery_request_result`]: crate::Ap33772s::get_power_delivery_request_result
    #[maybe_async::maybe_async]
    pub async fn negotiate_power_delivery(
        &mut self,
        power_data_object_index: PowerDataObject,
        voltage_selection: Option<ElectricPotential>,
        current_selection: OperatingCurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        // Check to see if PDO requested is available on the Source, return early if not
        if !data_objects
            .get_power_data_object(power_data_object_index)
            .is_detected()
        {
            return Err(Ap33772sError::PowerDataObjectNotDetected(
                power_data_object_index,
            ));
        }

        self.send_power_delivery_request(
            power_data_object_index,
            voltage_selection,
            current_selection,
            data_objects,
        )
        .await?;
        self.delay
            .delay_ms(
                u32::try_from(Self::NEGOTIATE_TIMING_DELAY.as_millis())
                    .expect("This should not fail, HAL Duration Type Conversions"),
            )
            .await;
        self.get_power_delivery_request_result().await
    }

    /// Negotiates maximum power delivery from a specific Power Data Object.
    ///
    /// This is a simplified version of [`negotiate_power_delivery`] that automatically
    /// requests the maximum available power from the specified PDO without requiring
    /// detailed parameter specification.
    ///
    /// # Parameters
    ///
    /// - `power_data_object_index` - Which PDO to request maximum power from
    ///
    /// # Behavior
    ///
    /// This function:
    /// 1. Sends a maximum power request using the AP33772S special message format
    /// 2. Automatically sets current selection to maximum
    /// 3. Uses appropriate voltage selection for the PDO type
    /// 4. Waits for source response with automatic timing
    ///
    /// # Returns
    ///
    /// Returns the same [`PowerDeliveryResponse`] values as [`negotiate_power_delivery`].
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - Other errors related to device state or communication
    ///
    /// # Use Cases
    ///
    /// - **Fast charging applications**: Get maximum power without analysis
    /// - **Simple power management**: Avoid complex PDO capability checking
    /// - **Emergency power**: Quickly access highest available power
    /// - **Testing and characterization**: Determine maximum source capability
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::command_structures::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Request maximum power from PDO 4 (often 20V)
    /// let response = device.negotiate_maximum_power_delivery(
    ///     PowerDataObject::Four
    /// ).await?;
    ///
    /// match response {
    ///     PowerDeliveryResponse::Success => {
    ///         let stats = device.get_statistics().await?;
    ///         println!("Maximum power negotiated: {:.1}W",
    ///                  stats.requested_power.get::<watt>());
    ///     },
    ///     PowerDeliveryResponse::NotSupported => {
    ///         println!("Maximum power request rejected");
    ///     },
    ///     _ => println!("Unexpected response: {:?}", response),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Comparison with negotiate_power_delivery
    ///
    /// | Feature | negotiate_power_delivery | negotiate_maximum_power_delivery |
    /// |---------|-------------------------|----------------------------------|
    /// | Voltage control | Custom (for variable PDOs) | Automatic maximum |
    /// | Current control | Selectable | Always maximum |
    /// | Parameter validation | Full validation | Minimal |
    /// | Use case | Precise control | Maximum power |
    ///
    /// # Timing Behavior
    ///
    /// Uses the same timing as [`negotiate_power_delivery`] with the standard
    /// 100ms negotiation delay for source processing.
    ///
    /// # See Also
    ///
    /// - [`negotiate_power_delivery`] for full parameter control
    /// - [`send_maximum_power_delivery_request`] for low-level access (advanced mode)
    /// - [`get_all_source_power_capabilities`] to verify PDO availability first
    ///
    /// [`negotiate_power_delivery`]: Self::negotiate_power_delivery
    /// [`PowerDeliveryResponse`]: crate::types::command_structures::PowerDeliveryResponse
    /// [`send_maximum_power_delivery_request`]: crate::Ap33772s::send_maximum_power_delivery_request
    /// [`get_all_source_power_capabilities`]: crate::Ap33772s::get_all_source_power_capabilities
    #[maybe_async::maybe_async]
    pub async fn negotiate_maximum_power_delivery(
        &mut self,
        power_data_object_index: PowerDataObject,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        self.send_maximum_power_delivery_request(power_data_object_index)
            .await?;
        self.delay
            .delay_ms(
                u32::try_from(Self::NEGOTIATE_TIMING_DELAY.as_millis())
                    .expect("This should not fail, HAL Duration Type Conversions"),
            )
            .await;
        self.get_power_delivery_request_result().await
    }
}

#[cfg(feature = "interrupts")]
impl<I2C: I2c, D: DelayNs, P: InputPin> Ap33772s<I2C, D, P> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    /// Creates a new instance of the AP33772S device. This Instance has no initialisation with the I2C bus.
    pub fn new(i2c: I2C, delay: D, interrupt_pin: P) -> Self {
        todo!("Not implemented Yet");
        Self {
            i2c,
            delay,
            interrupt_pin,
        }
    }
    /// Creates a new instance of the AP33772S device and checks if the device is present on the bus.
    /// TODO: Integrate Setting of Thermal Resistance and Thresholds matching RotoPD Board. This also handles the timings required for initialisation by using the provided hals delay method
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C, delay: D, interrupt_pin: P) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c, delay, interrupt_pin);
        device.is_device_present().await?;
        // TODO: Initialize Thermal Resistances and Thresholds
        todo!("Not implemented Yet");
        Ok(device)
    }
}
#[cfg(not(feature = "interrupts"))]
impl<I2C: I2c, D: DelayNs> Ap33772s<I2C, D> {
    /// Verifies that an AP33772S device is present and responding on the I2C bus.
    ///
    /// This function performs device detection by reading a known register (system control)
    /// and verifying that the command version field contains the expected value for an
    /// AP33772S device. This is essential for confirming device presence and type.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the device is detected and responding correctly.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if I2C communication fails (device not responding, bus errors, etc.)
    /// - [`Ap33772sError::WrongCommandVersion`] if device responds but is not an AP33772S
    ///
    /// # Device Detection Process
    ///
    /// 1. **I2C Communication**: Attempts to read the system control register
    /// 2. **Command Version Check**: Verifies the device reports the expected AP33772S version
    /// 3. **Type Validation**: Confirms this is actually an AP33772S device
    ///
    /// # I2C Address
    ///
    /// The function communicates with the device at I2C address [`0x52`] (7-bit addressing).
    /// Ensure your I2C bus configuration matches this address.
    ///
    /// # Usage Examples
    ///
    /// ## Basic Device Detection
    /// ```rust,no_run
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Verify device is present before proceeding
    /// device.is_device_present().await?;
    /// println!("AP33772S device detected and responding");
    ///
    /// // Now safe to use other device functions
    /// let status = device.get_status().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Error Handling with Device Detection
    /// ```rust,no_run
    /// # use ap33772s_rs::types::Ap33772sError;
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) {
    /// match device.is_device_present().await {
    ///     Ok(()) => println!("AP33772S found and ready"),
    ///     Err(Ap33772sError::I2c(_)) => {
    ///         eprintln!("I2C communication failed - check wiring and power");
    ///     },
    ///     Err(Ap33772sError::WrongCommandVersion(version)) => {
    ///         eprintln!("Wrong device type. Expected AP33772S, got version: 0x{:02X}", version);
    ///     },
    ///     Err(other) => eprintln!("Unexpected error: {}", other),
    /// }
    /// # }
    /// ```
    ///
    /// # Troubleshooting Device Detection
    ///
    /// ## I2C Communication Errors
    /// If you get [`Ap33772sError::I2c`] errors:
    /// - **Check power supply**: Ensure AP33772S has stable 3.3V or 5V power
    /// - **Verify wiring**: Confirm SDA, SCL, and GND connections
    /// - **Check pull-ups**: I2C bus needs pull-up resistors (typically 4.7kΩ)
    /// - **Bus speed**: Try reducing I2C clock speed if using high speeds
    /// - **Address conflicts**: Ensure no other devices use address 0x52
    ///
    /// ## Wrong Device Type Errors
    /// If you get [`Ap33772sError::WrongCommandVersion`]:
    /// - **Different device**: Another device may be at the same address
    /// - **Wrong part number**: Verify you have an AP33772S (not AP33771S, etc.)
    /// - **Device failure**: The AP33772S may be damaged or in undefined state
    /// - **Firmware differences**: Different firmware versions (rare)
    ///
    /// # When to Call This Function
    ///
    /// - **After device power-on**: Before any other device operations
    /// - **During initialization**: As part of device setup sequence
    /// - **For diagnostics**: When troubleshooting communication issues
    /// - **Before configuration**: To ensure device is ready for setup
    ///
    /// **Note**: [`new_default`] automatically calls this function, so separate
    /// calling is only needed when using [`new`] for manual initialization.
    ///
    /// # Performance Considerations
    ///
    /// This function performs a single I2C transaction and completes quickly.
    /// It's safe to call multiple times but unnecessary once device presence
    /// is confirmed.
    ///
    /// # See Also
    ///
    /// - [`new_default`] which includes automatic device detection
    /// - [`get_status`] for detailed device status after detection
    /// - [`hard_reset`] if device detection fails and recovery is needed
    ///
    /// [`0x52`]: Self::ADDRESS
    /// [`new_default`]: Self::new_default
    /// [`new`]: Self::new
    /// [`get_status`]: crate::Ap33772s::get_status
    /// [`hard_reset`]: Self::hard_reset
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

    /// Performs a complete hardware reset of the AP33772S device.
    ///
    /// This function initiates a hard reset of the device, which completely resets
    /// all internal state, registers, and configuration back to power-on defaults.
    /// This is the most comprehensive reset available and should be used when the
    /// device is in an unknown or problematic state.
    ///
    /// # Reset Behavior
    ///
    /// A hard reset will:
    /// - **Clear all registers**: Return all configuration to default values
    /// - **Reset state machines**: Return internal logic to power-on state
    /// - **Clear fault conditions**: Remove any active protection or error states
    /// - **Restart boot sequence**: Device will re-run its internal initialization
    /// - **Break USB-C connections**: Disconnect from any active PD sessions
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the reset command was successfully sent to the device.
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if the I2C communication fails while sending
    /// the reset command.
    ///
    /// # Important Timing Considerations
    ///
    /// After calling this function:
    /// 1. **Wait for reset**: Allow 100-200ms for the device to complete reset
    /// 2. **Re-initialize**: Call [`new_default`] or manually reconfigure device
    /// 3. **Reconnect USB-C**: May need to disconnect/reconnect USB-C device
    ///
    /// # Hardware Limitations
    ///
    /// **Important for RotoPD Board Users**: This function sends a software reset
    /// command but cannot fully power-cycle the device if it's powered through
    /// the Stemma connector. For complete reset on RotoPD boards:
    /// 1. Call `hard_reset()`
    /// 2. Disconnect Stemma connector
    /// 3. Disconnect USB-C device
    /// 4. Reconnect Stemma connector
    /// 5. Reconnect USB-C device
    ///
    /// # Usage Examples
    ///
    /// ## Basic Reset and Recovery
    /// ```rust,no_run
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Reset device to recover from error state
    /// device.hard_reset().await?;
    ///
    /// // Wait for reset to complete
    /// device.delay.delay_ms(200).await;
    ///
    /// // Re-verify device presence
    /// device.is_device_present().await?;
    ///
    /// println!("Device successfully reset and verified");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Reset with Full Re-initialization
    /// ```rust,no_run
    /// # async fn recovery_example(i2c: impl embedded_hal::i2c::I2c, delay: impl embedded_hal::delay::DelayNs) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::Ap33772s;
    ///
    /// // Create device instance
    /// let mut device = Ap33772s::new(i2c, delay);
    ///
    /// // Attempt reset and full re-initialization
    /// device.hard_reset().await?;
    ///
    /// // Wait for reset completion
    /// device.delay.delay_ms(200).await;
    ///
    /// // Perform complete re-initialization
    /// device = Ap33772s::new_default(device.i2c, device.delay).await?;
    ///
    /// println!("Device fully reset and re-initialized");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # When to Use Hard Reset
    ///
    /// Use hard reset when:
    /// - **Device unresponsive**: Not responding to normal commands
    /// - **Initialization failure**: [`new_default`] returns [`Ap33772sError::InitialisationFailure`]
    /// - **Fault recovery**: Device stuck in protection or error state
    /// - **Configuration cleanup**: Need to return to known clean state
    /// - **Testing/development**: Reset between test iterations
    ///
    /// # Recovery Sequence
    ///
    /// Recommended recovery sequence after hard reset:
    /// 1. **Wait**: Allow reset to complete (100-200ms)
    /// 2. **Verify**: Call [`is_device_present`] to confirm communication
    /// 3. **Configure**: Set thermal resistances and thresholds
    /// 4. **Enable**: Configure power delivery modes if needed
    /// 5. **Test**: Verify basic functionality before use
    ///
    /// # Alternative Reset Methods
    ///
    /// - **Power cycle**: Physical power removal (most complete)
    /// - **Soft reset**: Via other configuration registers (limited scope)
    /// - **USB-C disconnect**: Breaking PD sessions without device reset
    ///
    /// # See Also
    ///
    /// - [`new_default`] for complete initialization after reset
    /// - [`is_device_present`] to verify device after reset
    /// - [`get_status`] to check device state after reset
    ///
    /// [`new_default`]: Self::new_default
    /// [`is_device_present`]: Self::is_device_present
    /// [`get_status`]: crate::Ap33772s::get_status
    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await
    }
}
