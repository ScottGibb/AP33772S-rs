use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::types::api_commands::{ExtendedPowerRangeDataObject, StandardPowerRangeDataObject};

#[derive(Debug, PartialEq, Clone)]
pub enum SourcePowerRangeDataObject {
    Standard(StandardPowerRangeDataObject),
    Extended(ExtendedPowerRangeDataObject),
}

impl SourcePowerRangeDataObject {
    pub fn voltage_resolution(&self) -> u16 {
        match self {
            SourcePowerRangeDataObject::Standard(_) => {
                StandardPowerRangeDataObject::VOLTAGE_RESOLUTION
            }
            SourcePowerRangeDataObject::Extended(_) => {
                ExtendedPowerRangeDataObject::VOLTAGE_RESOLUTION
            }
        }
    }
    pub fn source_power_type(&self) -> PowerType {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => data_object.source_power_type(),
            SourcePowerRangeDataObject::Extended(data_object) => data_object.source_power_type(),
        }
    }
    pub fn is_detected(&self) -> bool {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => data_object.is_detected(),
            SourcePowerRangeDataObject::Extended(data_object) => data_object.is_detected(),
        }
    }
}
impl core::fmt::Display for SourcePowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => {
                write!(f, "Standard({data_object})")
            }
            SourcePowerRangeDataObject::Extended(data_object) => {
                write!(f, "Extended({data_object})")
            }
        }
    }
}

#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerType {
    Fixed = 0,
    Adjustable = 1,
}

// TODO: Add
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq)]
pub enum PeakCurrent {
    ConditionOne = 0,
    ConditionTwo = 1,
    ConditionThree = 2,
    ConditionFour = 3,
}

impl From<u2> for PeakCurrent {
    fn from(value: u2) -> Self {
        match value.value() {
            0 => PeakCurrent::ConditionOne,
            1 => PeakCurrent::ConditionTwo,
            2 => PeakCurrent::ConditionThree,
            3 => PeakCurrent::ConditionFour,
            _ => unreachable!("This will never happen due to rust type safety"),
        }
    }
}

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
    Maximum = 15,
}
