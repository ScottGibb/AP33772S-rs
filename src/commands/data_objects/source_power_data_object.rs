use crate::commands::data_objects::all_source_power_data_object::PowerType;
use crate::error::Ap33772sError;
use crate::types::units::*;
use bitbybit::{bitenum, bitfield};

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct StandardPowerRangeDataObject {
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

/// Maximum Voltage of 30V (30000 mV) with 100mV resolution
/// U16 can hold values up to 65535
/// 0 = 0mV
/// 1 = 100mV
/// 2 = 200mV
/// ...
/// 300 = 30000mV (30V)
/// This means the maximum raw value is 300
/// 300 * 100 = 30000mV
/// Therefore the voltage should be checked multiplied and stored in a U16
impl StandardPowerRangeDataObject {
    const VOLTAGE_RESOLUTION: u16 = 100; // mV per Unit

    pub fn max_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        let scaled_voltage = u16::from(self.raw_max_voltage())
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)?;
        Ok(ElectricPotential::new::<millivolt>(f32::from(
            scaled_voltage,
        )))
    }
}
/// VOLTAGE_MIN
/// For AVS APDO (bit\[14\]=1)
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinimumVoltage {
    Reserved = 0,
    _3_3 = 1,
    _3_3To5 = 2,
    Others = 3,
}

// #[bitenum(u2, exhaustive=true)]
// #[derive(Debug, PartialEq)]
// pub enum PeakCurrent{
//     CurrentEqualsIoc =0,

// }

#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SourcePowerCurrent {
    LessThan1_24 = 0,
    _1_24To1_49 = 1,
    _1_50To1_74 = 2,
    _1_75To1_99 = 3,
    _2_00To2_24 = 4,
    _2_25To2_49 = 5,
    _2_50To2_74 = 6,
    _2_75To2_99 = 7,
    _3_00To3_24 = 8,
    _3_25To3_49 = 9,
    _3_50To3_74 = 10,
    _3_75To3_99 = 11,
    _4_00To4_24 = 12,
    _4_25To4_49 = 13,
    _4_50To4_99 = 14,
    MoreThan5 = 15,
}

impl core::fmt::Display for StandardPowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SourcePowerDataObject {{ max_voltage: {:?} V, minimum_voltage: {:?}, max_current: {:?} A, source_power_type: {:?}, is_detected: {} }}",
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
impl defmt::Format for StandardPowerRangeDataObject {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SourcePowerDataObject {{ max_voltage: {:?}, minimum_voltage: {:?}, max_current: {:?} A, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .map_err(|_| core::fmt::Error)
                .unwrap() //TODO: Fix this
                .get::<volt>(),
            self.minimum_voltage(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}
