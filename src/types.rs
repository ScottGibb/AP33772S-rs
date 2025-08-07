// Types required by the Getters and Setters
// Expose the requireed types for the functions to be used externally.
pub use crate::commands::data_objects::all_source_power_data_object::PowerType;
pub use crate::commands::power_delivery::power_delivery_request_message::{
    CURRENT_SELECTIONS, CurrentSelection, PowerDataObject,
};

use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageThreshold;
pub mod units {
    pub use uom::si::electric_current::ampere;
    pub use uom::si::electric_potential::millivolt;
    pub use uom::si::f32::ElectricCurrent;
    pub use uom::si::f32::ElectricPotential;
    pub use uom::si::f32::ElectricalResistance;
    pub use uom::si::f32::Power;
    pub use uom::si::f32::ThermodynamicTemperature;
    pub use uom::si::power::watt;
    pub use uom::si::thermodynamic_temperature::degree_celsius;
}
use units::*;
#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Statistics {
    pub current: ElectricCurrent,
    pub voltage: ElectricPotential,
    pub power: Power,
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
            self.current.get::<uom::si::electric_current::ampere>(),
            self.voltage.get::<uom::si::electric_potential::volt>(),
            self.power.get::<uom::si::power::watt>(),
            self.temperature
                .get::<uom::si::thermodynamic_temperature::degree_celsius>(),
            self.requested_voltage
                .get::<uom::si::electric_potential::volt>(),
            self.requested_current
                .get::<uom::si::electric_current::ampere>(),
            self.requested_power.get::<uom::si::power::watt>()
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ThermalResistances {
    pub _25: ElectricalResistance,
    pub _50: ElectricalResistance,
    pub _75: ElectricalResistance,
    pub _100: ElectricalResistance,
}

#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Thresholds {
    pub over_voltage: ElectricPotential,
    pub under_voltage: UnderVoltageThreshold,
    pub over_current: ElectricCurrent,
    pub over_temperature: ThermodynamicTemperature,
    pub derating: ThermodynamicTemperature,
}
