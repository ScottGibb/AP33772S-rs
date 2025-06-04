use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageThreshold;
use uom::si::f32::ElectricPotential;

use super::hal::*;
use uom::si::f32::ElectricCurrent;
use uom::si::f32::ElectricalResistance;
use uom::si::f32::Power;
use uom::si::f32::ThermodynamicTemperature;

#[derive(Debug)]
pub struct AP33772SStatistics {
    pub current: ElectricCurrent,
    pub voltage: ElectricPotential,
    pub power: Power,
    pub temperature: ThermodynamicTemperature,

    pub requested_voltage: ElectricPotential,
    pub requested_current: ElectricCurrent,
    pub requested_power: Power,
}

#[derive(Debug)]
pub struct AP33772SThermalResistances {
    pub resistance_25: ElectricalResistance,
    pub resistance_50: ElectricalResistance,
    pub resistance_75: ElectricalResistance,
    pub resistance_100: ElectricalResistance,
}

#[derive(Debug)]
pub struct AP33772SThresholds {
    pub over_voltage_threshold: ElectricPotential,
    pub under_voltage_threshold: UnderVoltageThreshold,
    pub over_current_threshold: ElectricCurrent,
    pub over_temperature_threshold: ThermodynamicTemperature,
    pub derating_threshold: ThermodynamicTemperature,
}
pub struct Ap33772s<I2C: I2c> {
    pub(crate) i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    pub const ADDRESS: u8 = 0x52;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Self {
        Self::new(i2c)
    }
}
