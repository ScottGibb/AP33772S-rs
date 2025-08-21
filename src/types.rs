//! # Public Types and Data Structures
//!
//! This module contains all public types used by the AP33772S driver API.
//! These types represent various device states, configurations, and measurement data
//! that can be read from or written to the AP33772S device.

/// # Command Structures
///
/// Re-exports of internal command types used by the getter and setter methods.
/// These types are used when configuring the device or interpreting responses.
///
/// **Available when using the `advanced` feature:**
/// - Full access to underlying register structures
/// - Direct register manipulation capabilities
/// - Low-level device configuration options
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
use crate::units::*;

/// # Power Delivery Mode Configuration
///
/// Represents the supported Power Delivery modes and capabilities of the connected USB-C device.
/// This struct indicates whether advanced power delivery features are enabled and supported.
///
/// ## Fields
///
/// - `programmable_power_supply_adjustable_voltage_supply_enabled`: Indicates if PPS (Programmable Power Supply) with AVS (Adjustable Voltage Supply) is supported
/// - `extended_power_range_mode_enabled`: Indicates if Extended Power Range (EPR) mode is supported for higher power delivery
///
/// ## Usage
///
/// **Get current configuration:**
/// ```rust
/// # use ap33772s_rs::{Ap33772s, types::PowerDeliveryMode};
/// # async fn example(mut ap33772s: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// let pd_mode: PowerDeliveryMode = ap33772s.get_power_delivery_configuration()?;
/// if pd_mode.programmable_power_supply_adjustable_voltage_supply_enabled {
///     println!("PPS with AVS is supported");
/// }
/// # Ok(())
/// # }
/// ```
///
/// **Set configuration:**
/// ```rust
/// # use ap33772s_rs::{Ap33772s, types::PowerDeliveryMode};
/// # async fn example(mut ap33772s: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// let pd_mode = PowerDeliveryMode {
///     programmable_power_supply_adjustable_voltage_supply_enabled: true,
///     extended_power_range_mode_enabled: false,
/// };
/// ap33772s.set_power_delivery_mode(pd_mode)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Related Methods
///
/// - [`crate::Ap33772s::get_power_delivery_configuration`] - Read current configuration
/// - [`crate::Ap33772s::set_power_delivery_mode`] - Update configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PowerDeliveryMode {
    pub programmable_power_supply_adjustable_voltage_supply_enabled: bool,
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

/// # Device Statistics and Measurements
///
/// Contains real-time operating statistics and measurements from the AP33772S device.
/// This includes both current operating values and the requested values from power negotiations.
///
/// ## Current Operating Values
///
/// - `current`: Current flowing through the device ([`ElectricCurrent`])
/// - `voltage`: Output voltage being supplied ([`ElectricPotential`])  
/// - `power`: Power being delivered ([`Power`])
/// - `temperature`: Internal temperature of the device ([`ThermodynamicTemperature`])
///
/// ## Requested Values (from PD Negotiation)
///
/// - `requested_voltage`: Voltage requested by the connected device ([`ElectricPotential`])
/// - `requested_current`: Current requested by the connected device ([`ElectricCurrent`])
/// - `requested_power`: Power requested by the connected device ([`Power`])
///
/// ## Usage
///
/// ```rust
/// # use ap33772s_rs::{Ap33772s, types::Statistics};
/// # use ap33772s_rs::units::*;
/// # async fn example(mut ap33772s: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// let stats: Statistics = ap33772s.get_statistics()?;
///
/// println!("Operating: {:.2}V @ {:.2}A = {:.2}W",
///          stats.voltage.get::<volt>(), stats.current.get::<ampere>(), stats.power.get::<watt>());
/// println!("Temperature: {:.1}°C", stats.temperature.get::<degree_celsius>());
/// println!("Requested: {:.2}V @ {:.2}A = {:.2}W",
///          stats.requested_voltage.get::<volt>(), stats.requested_current.get::<ampere>(), stats.requested_power.get::<watt>());
/// # Ok(())
/// # }
/// ```
///
/// ## Related Methods
///
/// - [`crate::Ap33772s::get_statistics`] - Read current device statistics
///
/// [`ElectricCurrent`]: crate::units::ElectricCurrent
/// [`ElectricPotential`]: crate::units::ElectricPotential
/// [`Power`]: crate::units::Power
/// [`ThermodynamicTemperature`]: crate::units::ThermodynamicTemperature
#[derive(Debug, Clone, PartialEq)]
pub struct Statistics {
    /// The operating Current
    pub current: ElectricCurrent,
    /// The operating Voltage
    pub voltage: ElectricPotential,
    /// The operating Power
    pub power: Power,
    /// The Current Temperature
    pub temperature: ThermodynamicTemperature,

    /// The requested voltage from power delivery negotiation
    pub requested_voltage: ElectricPotential,
    /// The requested current from power delivery negotiation  
    pub requested_current: ElectricCurrent,
    /// The requested power from power delivery negotiation
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

/// # Thermal Resistance Configuration
///
/// Represents the thermal resistance values for the NTC (Negative Temperature Coefficient) thermistor
/// at different temperature points. These values are used by the AP33772S for thermal protection and
/// temperature monitoring.
///
/// ## Fields
///
/// - `_25`: Resistance at 25°C ([`ElectricalResistance`])
/// - `_50`: Resistance at 50°C ([`ElectricalResistance`])  
/// - `_75`: Resistance at 75°C ([`ElectricalResistance`])
/// - `_100`: Resistance at 100°C ([`ElectricalResistance`])
///
/// ## Usage
///
/// ```rust
/// # use ap33772s_rs::{Ap33772s, types::ThermalResistances, units::*};
/// # async fn example(mut ap33772s: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// // Use default values based on typical NTC characteristics
/// let thermal_resistances = ThermalResistances::default();
/// ap33772s.set_thermal_resistances(thermal_resistances)?;
///
/// // Or specify custom values
/// let custom_resistances = ThermalResistances {
///     _25: ElectricalResistance::new::<ohm>(10000.0),
///     _50: ElectricalResistance::new::<ohm>(3893.0),
///     _75: ElectricalResistance::new::<ohm>(1622.0),
///     _100: ElectricalResistance::new::<ohm>(779.0),
/// };
/// ap33772s.set_thermal_resistances(custom_resistances)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Related Methods
///
/// - [`crate::Ap33772s::get_thermal_resistances`] - Read current thermal resistance settings
/// - [`crate::Ap33772s::set_thermal_resistances`] - Update thermal resistance settings
///
/// [`ElectricalResistance`]: crate::units::ElectricalResistance
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalResistances {
    /// The resistance at 25°C
    pub _25: ElectricalResistance,
    /// The resistance at 50°C
    pub _50: ElectricalResistance,
    /// The resistance at 75°C
    pub _75: ElectricalResistance,
    /// The resistance at 100°C
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

/// # Protection Thresholds Configuration
///
/// Defines the safety and protection thresholds for the AP33772S device. When any of these
/// thresholds are exceeded, the device will take protective action and set corresponding
/// flags in the device [`Status`].
///
/// ## Protection Types
///
/// - `over_voltage`: Maximum voltage threshold - triggers load disconnection ([`ElectricPotential`])
/// - `under_voltage`: Minimum voltage threshold - triggers fault state ([`UnderVoltageThreshold`])
/// - `over_current`: Maximum current threshold - triggers power cutoff ([`ElectricCurrent`])
/// - `over_temperature`: Maximum temperature threshold - triggers thermal shutdown ([`ThermodynamicTemperature`])
/// - `derating`: Temperature threshold for current derating (50% reduction) ([`ThermodynamicTemperature`])
///
/// ## Usage
///
/// ```rust
/// # use ap33772s_rs::{Ap33772s, types::{Thresholds, UnderVoltageThreshold}, units::*};
/// # async fn example(mut ap33772s: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
/// let thresholds = Thresholds {
///     over_voltage: ElectricPotential::new::<volt>(22.0),
///     under_voltage: UnderVoltageThreshold::default(),
///     over_current: ElectricCurrent::new::<ampere>(5.0),
///     over_temperature: ThermodynamicTemperature::new::<degree_celsius>(85.0),
///     derating: ThermodynamicTemperature::new::<degree_celsius>(75.0),
/// };
///
/// ap33772s.set_thresholds(thresholds)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Related Methods
///
/// - [`crate::Ap33772s::get_thresholds`] - Read current threshold settings
/// - [`crate::Ap33772s::set_thresholds`] - Update threshold settings
///
/// [`Status`]: crate::types::command_structures::Status
/// [`ElectricPotential`]: crate::units::ElectricPotential  
/// [`ElectricCurrent`]: crate::units::ElectricCurrent
/// [`ThermodynamicTemperature`]: crate::units::ThermodynamicTemperature
/// [`UnderVoltageThreshold`]: crate::types::UnderVoltageThreshold
#[derive(Debug, Clone, PartialEq)]
pub struct Thresholds {
    /// Maximum voltage threshold - triggers load disconnection via MOSFET switch
    pub over_voltage: ElectricPotential,
    /// Minimum voltage threshold - triggers fault state and load disconnection  
    pub under_voltage: UnderVoltageThreshold,
    /// Maximum current threshold - triggers power cutoff and fault state
    pub over_current: ElectricCurrent,
    /// Maximum temperature threshold - triggers thermal shutdown via MOSFET switch
    pub over_temperature: ThermodynamicTemperature,
    /// Temperature threshold for current derating - reduces input current by 50%
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
