//! # Public Types for AP33772S Driver
//!
//! This module exposes all the types required for using the AP33772S driver API.
//! It provides both high-level convenience types and direct access to command
//! structures for advanced usage.
//!
//! ## Type Categories
//!
//! ### High-Level Data Types
//! - [`PowerDeliveryMode`] - Power delivery capability configuration
//! - [`Statistics`] - Complete device operational status  
//! - [`ThermalResistances`] - Thermal modeling parameters
//! - [`Thresholds`] - Protection threshold configuration
//!
//! ### Unit System  
//! The [`units`] module provides strongly-typed physical quantities using the
//! [uom](https://docs.rs/uom) crate for type-safe unit handling.
//!
//! ### Command Structures
//! The [`command_structures`] module re-exports internal command types used
//! for direct device communication (primarily for advanced mode).
//!
//! ## Usage Examples
//!
//! ```rust,no_run
//! use ap33772s_rs::types::{*, units::*};
//!
//! // Create protection thresholds
//! let thresholds = Thresholds {
//!     over_voltage: ElectricPotential::new::<volt>(21.0),
//!     over_current: ElectricCurrent::new::<ampere>(3.5),
//!     over_temperature: ThermodynamicTemperature::new::<degree_celsius>(125.0),
//!     under_voltage: ElectricPotential::new::<volt>(4.5),
//!     derating: ThermodynamicTemperature::new::<degree_celsius>(85.0),
//! };
//!
//! // Configure power delivery modes
//! let pd_mode = PowerDeliveryMode {
//!     extended_power_range_mode_enabled: true,
//!     programmable_power_supply_adjustable_voltage_supply_enabled: true,
//! };
//! ```
//!
//! ## Error Handling
//! All operations return [`Ap33772sError`] which provides detailed information
//! about failure modes and suggested recovery actions.

/// Re-exports the internal command API types used for getters and setters.
///
/// This module provides access to the low-level command structures that directly
/// correspond to AP33772S registers and messages. These types are primarily used
/// internally by the driver but are exposed for advanced use cases.
///
/// ## When to Use Command Structures
///
/// - **Advanced mode**: When the `advanced` feature is enabled for low-level control
/// - **Custom operations**: Implementing functionality not covered by high-level APIs
/// - **Debugging**: Direct register access for troubleshooting
/// - **Performance**: Bypassing high-level abstraction when needed
///
/// ## Available Types
///
/// ### Configuration Types
/// - [`OperationMode`] - Device operational state and CC configuration
/// - [`Status`] - Device status register with protection flags
/// - [`VoltageOutputControl`] - Output voltage switch control
///
/// ### Power Delivery Types  
/// - [`PowerDataObject`] - PDO selection enumeration
/// - [`OperatingCurrentSelection`] - Current selection modes
/// - [`PowerDeliveryResponse`] - PD negotiation response codes
///
/// ### Data Object Types
/// - [`AllSourceDataPowerDataObject`] - Complete source capabilities
/// - [`StandardPowerRangeDataObject`] - SPR PDO structure
/// - [`ExtendedPowerRangeDataObject`] - EPR PDO structure
/// - [`PowerType`] - PDO type enumeration (Fixed, Variable, Battery, PPS)
///
/// [`OperationMode`]: crate::commands::configuration::operation_mode::OperationMode
/// [`Status`]: crate::commands::configuration::status::Status
/// [`VoltageOutputControl`]: crate::commands::configuration::system_control::VoltageOutputControl
/// [`PowerDataObject`]: crate::commands::power_delivery::power_delivery_request_message::PowerDataObject
/// [`OperatingCurrentSelection`]: crate::commands::power_delivery::power_delivery_request_message::OperatingCurrentSelection
/// [`PowerDeliveryResponse`]: crate::commands::power_delivery::power_delivery_message_result::PowerDeliveryResponse
/// [`AllSourceDataPowerDataObject`]: crate::commands::data_objects::all_source_power_data_object::AllSourceDataPowerDataObject
/// [`StandardPowerRangeDataObject`]: crate::commands::data_objects::standard_power_range_data_object::StandardPowerRangeDataObject
/// [`ExtendedPowerRangeDataObject`]: crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject
/// [`PowerType`]: crate::commands::data_objects::source_power_range_data_object::PowerType
pub mod command_structures {
    pub use crate::commands::configuration::operation_mode::{
        ConfigurationChannel, DeRatingMode, OperationMode,
    };
    pub use crate::commands::configuration::status::Status;
    pub use crate::commands::configuration::system_control::VoltageOutputControl;
    pub use crate::commands::data_objects::all_source_power_data_object::AllSourceDataPowerDataObject;
    pub use crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject;
    pub use crate::commands::data_objects::source_power_range_data_object::PeakCurrent;
    pub use crate::commands::data_objects::source_power_range_data_object::PowerType;
    pub use crate::commands::data_objects::source_power_range_data_object::SourceMaximumCurrent;
    pub use crate::commands::data_objects::source_power_range_data_object::SourcePowerRangeDataObject;
    pub use crate::commands::data_objects::standard_power_range_data_object::MinimumVoltage;
    pub use crate::commands::data_objects::standard_power_range_data_object::StandardPowerRangeDataObject;
    pub use crate::commands::power_delivery::power_delivery_message_result::PowerDeliveryResponse;
    pub use crate::commands::power_delivery::power_delivery_request_message::{
        OperatingCurrentSelection, PowerDataObject,
    };
}

use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thermal_resistances::thermal_resistance_50::ThermalResistance50;
use crate::commands::thermal_resistances::thermal_resistance_75::ThermalResistance75;
use crate::commands::thermal_resistances::thermal_resistance_100::ThermalResistance100;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;

pub use crate::errors::Ap33772sError;
pub use crate::units::*;

/// Power Delivery mode configuration for the AP33772S.
///
/// This structure configures which advanced USB-C Power Delivery features are enabled
/// and advertised to connected sources. It controls the device's capabilities in terms
/// of power ranges and adjustable voltage support.
///
/// # Fields
///
/// ## `programmable_power_supply_adjustable_voltage_supply_enabled`
/// Enables support for variable voltage Power Data Objects:
/// - **PPS (Programmable Power Supply)**: Adjustable voltage in Standard Power Range (3.3V-21V)
/// - **AVS (Adjustable Voltage Supply)**: Adjustable voltage in Extended Power Range (15V-28V)
/// - When `true`: Device can request custom voltages within PDO ranges
/// - When `false`: Device limited to fixed voltage PDOs only
///
/// ## `extended_power_range_mode_enabled`  
/// Enables Extended Power Range (EPR) support:
/// - **EPR capability**: Supports voltages beyond 20V (up to 28V on AP33772S)
/// - **High power**: Enables power delivery beyond 100W
/// - **Cable requirements**: Requires 5A-rated electronically marked cables
/// - When `true`: Device can negotiate EPR PDOs
/// - When `false`: Device limited to Standard Power Range (SPR) only
///
/// # Power Delivery Modes
///
/// Different combinations enable different capabilities:
///
/// ```text
/// | EPR | PPS/AVS | Capability                           |
/// |-----|---------|--------------------------------------|
/// |  F  |    F    | Fixed SPR only (5V, 9V, 12V, 15V, 20V) |
/// |  F  |    T    | SPR + PPS (3.3V-21V adjustable)     |
/// |  T  |    F    | SPR + EPR fixed (up to 28V)         |
/// |  T  |    T    | Full capability (SPR + EPR + PPS/AVS) |
/// ```
///
/// # Usage
///
/// Configure the mode based on your application requirements:
///
/// ```rust
/// # use ap33772s_rs::types::PowerDeliveryMode;
/// // Conservative mode - fixed voltages only
/// let basic_mode = PowerDeliveryMode {
///     extended_power_range_mode_enabled: false,
///     programmable_power_supply_adjustable_voltage_supply_enabled: false,
/// };
///
/// // High-power mode - full capabilities  
/// let advanced_mode = PowerDeliveryMode {
///     extended_power_range_mode_enabled: true,
///     programmable_power_supply_adjustable_voltage_supply_enabled: true,
/// };
///
/// // Precision mode - adjustable voltage in SPR
/// let pps_mode = PowerDeliveryMode {
///     extended_power_range_mode_enabled: false,
///     programmable_power_supply_adjustable_voltage_supply_enabled: true,
/// };
/// ```
///
/// # Hardware Considerations
///
/// - **EPR mode**: Requires appropriate cable (5A rating with electronic marking)
/// - **High power**: Ensure adequate thermal management
/// - **Safety**: Configure appropriate protection thresholds
///
/// # See Also
///
/// - [`get_power_delivery_configuration`] to read current settings
/// - [`set_power_delivery_mode`] to configure these settings
/// - [USB PD Specification](https://github.com/ScottGibb/AP33772S-rs/blob/main/docs/understanding-the-usb-c-pd-specification.md)
///
/// [`get_power_delivery_configuration`]: crate::getters::Ap33772s::get_power_delivery_configuration
/// [`set_power_delivery_mode`]: crate::setters::Ap33772s::set_power_delivery_mode
#[derive(Debug, Clone, PartialEq)]
pub struct PowerDeliveryMode {
    /// Enable Programmable Power Supply (PPS) and Adjustable Voltage Supply (AVS) support.
    ///
    /// When `true`, the device can negotiate variable voltage PDOs that allow custom
    /// voltage selection within the PDO's specified range.
    pub programmable_power_supply_adjustable_voltage_supply_enabled: bool,

    /// Enable Extended Power Range (EPR) mode for high-power applications.
    ///
    /// When `true`, the device can negotiate PDOs with voltages beyond 20V and
    /// power levels beyond 100W (up to 28V @ 5A = 140W on AP33772S).
    pub extended_power_range_mode_enabled: bool,
}
impl core::fmt::Display for PowerDeliveryMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "PowerDeliveryMode {{")?;
        writeln!(
            f,
            "  programmable_power_supply_adjustable_voltage_supply_enabled: {}",
            self.programmable_power_supply_adjustable_voltage_supply_enabled
        )?;
        writeln!(
            f,
            "  extended_power_range_mode_enabled: {}",
            self.extended_power_range_mode_enabled
        )?;
        write!(f, "}}")
    }
}

/// Comprehensive operational statistics from the AP33772S device.
///
/// This structure contains a complete snapshot of the device's electrical and thermal
/// state, including both actual measured values and the values that were requested
/// from the Power Delivery source during negotiation.
///
/// # Measurement Categories
///
/// ## Output Measurements (Actual)
/// These values represent what the device is actually delivering to the load:
/// - [`current`] - Instantaneous output current to the load
/// - [`voltage`] - Instantaneous output voltage to the load  
/// - [`power`] - Calculated output power (voltage × current)
/// - [`temperature`] - Device junction temperature for thermal monitoring
///
/// ## Power Delivery Requests (Negotiated)
/// These values represent what was negotiated with the PD source:
/// - [`requested_voltage`] - Voltage level requested from the source
/// - [`requested_current`] - Current limit negotiated with the source
/// - [`requested_power`] - Calculated power budget (requested_voltage × requested_current)
///
/// # Usage Examples
///
/// ```rust,no_run
/// # use ap33772s_rs::{Ap33772s, types::units::*};
/// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// let stats = device.get_statistics().await?;
///
/// // Check actual output vs requested
/// println!("Power Delivery Status:");
/// println!("  Requested: {:.1}W @ {:.1}V, {:.1}A",
///          stats.requested_power.get::<watt>(),
///          stats.requested_voltage.get::<volt>(),
///          stats.requested_current.get::<ampere>());
///          
/// println!("  Actual:    {:.1}W @ {:.1}V, {:.1}A",
///          stats.power.get::<watt>(),
///          stats.voltage.get::<volt>(),
///          stats.current.get::<ampere>());
///
/// // Monitor thermal status
/// let temp_c = stats.temperature.get::<degree_celsius>();
/// if temp_c > 85.0 {
///     println!("Warning: High temperature {:.1}°C", temp_c);
/// }
///
/// // Check efficiency
/// let efficiency = if stats.requested_power.get::<watt>() > 0.0 {
///     (stats.power.get::<watt>() / stats.requested_power.get::<watt>()) * 100.0
/// } else { 0.0 };
/// println!("System efficiency: {:.1}%", efficiency);
/// # Ok(())
/// # }
/// ```
///
/// # Interpretation Guidelines
///
/// ## Normal Operation
/// - **Voltage**: Should closely match requested voltage (within regulation tolerance)
/// - **Current**: Should be ≤ requested current (depends on load)
/// - **Power**: Will vary with load, up to the negotiated limit
/// - **Temperature**: Should remain well below protection thresholds
///
/// ## Troubleshooting
/// - **Low voltage**: Check cable resistance, source regulation, load current
/// - **High current**: Verify load requirements, check for shorts
/// - **High temperature**: Improve thermal management, reduce power
/// - **Power mismatch**: Investigate load characteristics, cable losses
///
/// # Performance Monitoring
///
/// Use statistics for:
/// - **Load profiling**: Understanding power consumption patterns
/// - **Thermal monitoring**: Preventing overheating conditions  
/// - **Efficiency analysis**: Optimizing power delivery settings
/// - **System validation**: Verifying design requirements are met
///
/// # See Also
///
/// - [`get_statistics`] to retrieve current statistics
/// - Individual getters for specific measurements when complete data isn't needed
/// - [`get_thresholds`] for protection limit values
///
/// [`current`]: Self::current
/// [`voltage`]: Self::voltage
/// [`power`]: Self::power
/// [`temperature`]: Self::temperature
/// [`requested_voltage`]: Self::requested_voltage
/// [`requested_current`]: Self::requested_current
/// [`requested_power`]: Self::requested_power
/// [`get_statistics`]: crate::getters::Ap33772s::get_statistics
/// [`get_thresholds`]: crate::getters::Ap33772s::get_thresholds
#[derive(Debug, Clone, PartialEq)]
pub struct Statistics {
    /// Current output current being delivered to the load.
    ///
    /// This is the instantaneous current measurement from the device's internal
    /// current sensing circuitry. It represents the actual current flowing to
    /// the connected load.
    pub current: ElectricCurrent,

    /// Current output voltage being delivered to the load.
    ///
    /// This is the instantaneous voltage measurement at the device output.
    /// It may differ slightly from the requested voltage due to load regulation,
    /// cable drops, and other factors.
    pub voltage: ElectricPotential,

    /// Calculated output power (voltage × current).
    ///
    /// This represents the instantaneous power being delivered to the load.
    /// It is calculated from the measured voltage and current values.
    pub power: Power,

    /// Device junction temperature for thermal monitoring.
    ///
    /// This is the internal temperature of the AP33772S die, used for thermal
    /// protection and derating decisions. Critical for safe operation.
    pub temperature: ThermodynamicTemperature,

    /// Voltage level that was requested from the Power Delivery source.
    ///
    /// This represents the voltage that was negotiated during the last Power
    /// Delivery exchange. It shows what the source agreed to provide.
    pub requested_voltage: ElectricPotential,

    /// Current limit that was negotiated with the Power Delivery source.  
    ///
    /// This represents the maximum current that the device is allowed to draw
    /// from the source based on the negotiated Power Data Object.
    pub requested_current: ElectricCurrent,

    /// Calculated power budget (requested_voltage × requested_current).
    ///
    /// This represents the maximum power that was negotiated with the source.
    /// The actual power consumption should not exceed this value.
    pub requested_power: Power,
}

impl core::fmt::Display for Statistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use uom::si::{
            electric_current::ampere, electric_potential::volt, power::watt,
            thermodynamic_temperature::degree_celsius,
        };

        writeln!(f, "Statistics {{")?;
        writeln!(f, "  current: {:.3} A", self.current.get::<ampere>())?;
        writeln!(f, "  voltage: {:.3} V", self.voltage.get::<volt>())?;
        writeln!(f, "  power: {:.3} W", self.power.get::<watt>())?;
        writeln!(
            f,
            "  temperature: {:.2} °C",
            self.temperature.get::<degree_celsius>()
        )?;
        writeln!(
            f,
            "  requested_voltage: {:.3} V",
            self.requested_voltage.get::<volt>()
        )?;
        writeln!(
            f,
            "  requested_current: {:.3} A",
            self.requested_current.get::<ampere>()
        )?;
        writeln!(
            f,
            "  requested_power: {:.3} W",
            self.requested_power.get::<watt>()
        )?;
        write!(f, "}}")
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Statistics {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statistics {{\n  current: {} A\n  voltage: {} V\n  power: {} W\n  temperature: {} °C\n  requested_voltage: {} V\n  requested_current: {} A\n  requested_power: {} W\n}}",
            self.current.get::<ampere>(),
            self.voltage.get::<volt>(),
            self.power.get::<watt>(),
            self.temperature.get::<degree_celsius>(),
            self.requested_voltage.get::<volt>(),
            self.requested_current.get::<ampere>(),
            self.requested_power.get::<watt>()
        );
    }
}

/// The Thermal Resistances for the Negative Temperature Coefficeint Resister (NTC)
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalResistances {
    /// The Resistance at 25 degrees (celsius)
    pub _25: ElectricalResistance,
    /// The Temperature at 25 degrees (celsius)
    pub _50: ElectricalResistance,
    /// The Temperature at 50 degrees (celsius)
    pub _75: ElectricalResistance,
    /// The Temperature at 100 degrees (celsius)
    pub _100: ElectricalResistance,
}

impl core::fmt::Display for ThermalResistances {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "ThermalResistances {{")?;
        writeln!(f, "  25°C: {:.3} Ω", self._25.get::<ohm>())?;
        writeln!(f, "  50°C: {:.3} Ω", self._50.get::<ohm>())?;
        writeln!(f, "  75°C: {:.3} Ω", self._75.get::<ohm>())?;
        writeln!(f, "  100°C: {:.3} Ω", self._100.get::<ohm>())?;
        write!(f, "}}")
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ThermalResistances {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ThermalResistances {{\n  25°C: {} Ω\n  50°C: {} Ω\n  75°C: {} Ω\n  100°C: {} Ω\n}}",
            self._25.get::<ohm>(),
            self._50.get::<ohm>(),
            self._75.get::<ohm>(),
            self._100.get::<ohm>(),
        );
    }
}
impl Default for ThermalResistances {
    /// Default values are taken from the AP33772S Registers which have known compile time default values
    fn default() -> Self {
        ThermalResistances {
            _25: ThermalResistance25::default().thermal_resistance(),
            _50: ThermalResistance50::default().thermal_resistance(),
            _75: ThermalResistance75::default().thermal_resistance(),
            _100: ThermalResistance100::default().thermal_resistance(),
        }
    }
}
pub use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageThreshold;

/// The Thresholds for the AP33772S for the different fault protections. All thresholds when applied will result in
/// the corresponding flags in the [Status](crate::commands::configuration::status::Status) being set
#[derive(Debug, Clone, PartialEq)]
pub struct Thresholds {
    /// The over voltage threshold in which the load will be disconnected through MOS Switch and the system will enter
    /// a fault state
    pub over_voltage: ElectricPotential,
    /// The minimum voltage at which point if the device goes below, the AP33772S will enter a fault state and disconnect
    /// the load
    pub under_voltage: UnderVoltageThreshold,
    /// The current at which the device will turn off the power to the load and enter a fault state
    pub over_current: ElectricCurrent,
    /// The temperature at which the MOSFET Switch will be turned off disconnecting the load, entering a fault states
    pub over_temperature: ThermodynamicTemperature,
    /// The Hot Spot Temperature in which the device will lower the input current by 50% in an attempt to cool
    /// the AP33772S down.
    pub derating: ThermodynamicTemperature,
}

impl core::fmt::Display for Thresholds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Thresholds {{")?;
        writeln!(
            f,
            "  over_voltage: {:.3} V",
            self.over_voltage.get::<volt>()
        )?;
        writeln!(f, "  under_voltage: {:?}", self.under_voltage)?;
        writeln!(
            f,
            "  over_current: {:.3} A",
            self.over_current.get::<ampere>()
        )?;
        writeln!(
            f,
            "  over_temperature: {:.2} °C",
            self.over_temperature.get::<degree_celsius>()
        )?;
        writeln!(
            f,
            "  derating: {:.2} °C",
            self.derating.get::<degree_celsius>()
        )?;
        write!(f, "}}")
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Thresholds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Thresholds {{\n  over_voltage: {} V\n  under_voltage: {:?}\n  over_current: {} A\n  over_temperature: {} °C\n  derating: {} °C\n}}",
            self.over_voltage.get::<volt>(),
            self.under_voltage,
            self.over_current.get::<ampere>(),
            self.over_temperature.get::<degree_celsius>(),
            self.derating.get::<degree_celsius>(),
        );
    }
}

impl Default for Thresholds {
    /// Default values are derived from the AP33772S Registers which have known compile time default values
    fn default() -> Self {
        Thresholds {
            over_voltage: OverVoltageProtectionThreshold::default()
                .voltage()
                .expect("This Should Not fail - Value Taken Directly from Datasheet Register"),
            under_voltage: UnderVoltageThreshold::default(),
            over_current: OverCurrentProtectionThreshold::default()
                .current()
                .expect("This Should Not fail - Value Taken Directly from Datasheet Register"),
            over_temperature: OverTemperatureProtectionThreshold::default().temperature(),
            derating: DeRatingThreshold::default().temperature(),
        }
    }
}
