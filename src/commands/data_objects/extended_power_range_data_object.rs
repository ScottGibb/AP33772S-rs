use super::source_power_data_object::{MinimumVoltage, SourcePowerCurrent};
use crate::ap33772s::Ap33772sError;
use crate::commands::data_objects::all_source_power_data_object::PowerType;
use crate::types::units::*;
use bitbybit::bitfield;

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct ExtendedPowerRangeDataObject {
    #[bits(0..=7, r)]
    pub raw_max_voltage: u8,
    #[bits(8..=9, r)]
    // This is either Minimum Voltage or Peak Current.... TODO: Find out a way to handle this
    pub minimum_voltage: MinimumVoltage,
    #[bits(10..=13, r)]
    pub max_current: SourcePowerCurrent,
    #[bit(14, r)]
    pub source_power_type: PowerType,
    #[bit(15, r)]
    pub is_detected: bool,
}
/// Maximum Voltage of 30V (30000 mV) with 200mV resolution
/// U8 can hold values up to 255
/// 0 = 0mV
/// 1 = 200mV
/// 2 = 400mV
/// ...
/// 150 = 30000mV (30V)
/// This means the maximum raw value is 150
/// 150 * 200 = 30000mV U16 is required
/// Therefore the voltage should be checked multiplied
impl ExtendedPowerRangeDataObject {
    const VOLTAGE_RESOLUTION: u16 = 200; // mV per Unit
    pub fn max_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        let scaled_voltage = u16::from(self.raw_max_voltage())
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)?;
        Ok(ElectricPotential::new::<millivolt>(f32::from(
            scaled_voltage,
        )))
    }
}
impl core::fmt::Display for ExtendedPowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ExtendedPowerRangeDataObject {{ max_voltage: {:?}, minimum_voltage: {:?}, max_current: {:?}, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .map_err(|_| core::fmt::Error)?
                .get::<volt>(),
            self.minimum_voltage(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ExtendedPowerRangeDataObject {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ExtendedPowerRangeDataObject {{ max_voltage: {}, minimum_voltage: {:?}, max_current: {:?}, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .map_err(|_| core::fmt::Error)?
                .get::<volt>(),
            self.minimum_voltage(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}
