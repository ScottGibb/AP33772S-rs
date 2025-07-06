use super::source_power_data_object::{MinimumVoltage, SourcePowerCurrent};
use crate::commands::data_objects::all_source_power_data_object::PowerType;
use bitbybit::bitfield;

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExtendedPowerRangeDataObject {
    #[bits(0..=7, r)]
    pub max_voltage: u8,
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
