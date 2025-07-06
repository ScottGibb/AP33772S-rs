use bitbybit::{bitenum, bitfield};

use crate::commands::data_objects::all_source_power_data_object::PowerType;

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct SourcePowerDataObject {
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

/// VOLTAGE_MIN
/// For AVS APDO (bit\[14\]=1)
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq)]
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
