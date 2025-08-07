use crate::Ap33772sError;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageThreshold;
use uom::si::f32::ElectricPotential;

use super::hal::*;
use uom::si::f32::ElectricCurrent;
use uom::si::f32::ElectricalResistance;
use uom::si::f32::Power;
use uom::si::f32::ThermodynamicTemperature;

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

        write!(f, "Statistics {{\n")?;
        write!(f, "  current: {:.3} A\n", self.current.get::<ampere>())?;
        write!(f, "  voltage: {:.3} V\n", self.voltage.get::<volt>())?;
        write!(f, "  power: {:.3} W\n", self.power.get::<watt>())?;
        write!(
            f,
            "  temperature: {:.2} °C\n",
            self.temperature.get::<degree_celsius>()
        )?;
        write!(
            f,
            "  requested_voltage: {:.3} V\n",
            self.requested_voltage.get::<volt>()
        )?;
        write!(
            f,
            "  requested_current: {:.3} A\n",
            self.requested_current.get::<ampere>()
        )?;
        write!(
            f,
            "  requested_power: {:.3} W\n",
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

// Types required by the Getters and Setters
// Expose the requireed types for the functions to be used externally.
pub use crate::commands::data_objects::all_source_power_data_object::PowerType;
pub use crate::commands::power_delivery::power_delivery_request_message::{
    CURRENT_SELECTIONS, CurrentSelection, PowerDataObject,
};
pub struct Ap33772s<I2C: I2c> {
    pub(crate) i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    pub const ADDRESS: SevenBitAddress = 0x52;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c);
        device.is_device_present().await?;
        Ok(device)
    }

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

    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await?;
        Ok(())
    }
}
