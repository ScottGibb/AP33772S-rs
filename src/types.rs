//! Types required by the Getters and Setters
//! This module exposes the requireed types for the functions to be used externally.

/// Module Reexports the internal api command types that are used for the getters and setters
pub mod ap33772s_structures {
    pub use crate::commands::configuration::operation_mode::{
        ConfigurationChannel, DeRatingMode, OperationMode,
    };
    pub use crate::commands::configuration::status::Status;
    pub use crate::commands::configuration::system_control::VoltageOutputControl;
    pub use crate::commands::data_objects::all_source_power_data_object::AllSourceDataPowerDataObject;
    pub use crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject;
    pub use crate::commands::data_objects::source_power_range_data_object::PowerType;
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

/// This particular module is responsible for exposing the relevant UOM types for use with this library.
pub mod units {
    pub use uom::si::electric_current::ampere;
    pub use uom::si::electric_current::milliampere;
    pub use uom::si::electric_potential::millivolt;
    pub use uom::si::electric_potential::volt;
    pub use uom::si::electrical_resistance::milliohm;
    pub use uom::si::electrical_resistance::ohm;
    pub use uom::si::f32::ElectricCurrent;
    pub use uom::si::f32::ElectricPotential;
    pub use uom::si::f32::ElectricalResistance;
    pub use uom::si::f32::Power;
    pub use uom::si::f32::ThermodynamicTemperature;
    pub use uom::si::power::watt;
    pub use uom::si::thermodynamic_temperature::degree_celsius;
}
use units::*;

/// This struct represents the Power Delivery Modes and specifically,
/// allows the user to query if Adjustable Voltage Supply and or Extended Power Supply
/// is enabled and supported by the Type C device plugged into the AP33772S.
/// Use the following method to get this struct
///
///  **Getter**
/// `crate::getters::get_power_delivery_configuration()`
///
/// **Setter**
/// `crate::setters::set_power_delivery_configuration()`
///
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

/// The statistics struct contains all the current information about the device
/// specifically outlining what its doing currently and what its being requested to do
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

    pub requested_voltage: ElectricPotential,
    pub requested_current: ElectricCurrent,
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
    /// Default values are derrived from the AP33772S Registers which have known compile time default values
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
