use bitbybit::{bitenum, bitfield};

use super::source_power_data_object::{MinimumVoltage, SourcePowerCurrent};

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct ExtendedPowerRangeDataObject {
    #[bits(0..=7, r)]
    pub max_voltage: u8,
    #[bits(8..=9, r)]
    // This is either Minimum Voltage or Peak Current.... TODO: Find out a way to handle this
    pub minimum_voltage: MinimumVoltage,
    #[bits(10..=13, r)]
    pub max_current: SourcePowerCurrent,
    #[bit(14, r)]
    pub source_power_type: ExtendedPowerSourcePowerType,
    #[bit(15, r)]
    pub is_detected: bool,
}
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq)]

pub enum ExtendedPowerSourcePowerType {
    FixedPowerDataObject = 0,
    AdjustableVoltageSupplyAdjustablePowerDataObject = 1,
}
