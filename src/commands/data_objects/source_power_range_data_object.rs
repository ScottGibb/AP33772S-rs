use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::types::{ExtendedPowerRangeDataObject, StandardPowerRangeDataObject};
#[derive(Debug, PartialEq, Clone)]
pub enum SourcePowerRangeDataObject {
    Standard(StandardPowerRangeDataObject),
    Extended(ExtendedPowerRangeDataObject),
}

impl SourcePowerRangeDataObject {
    pub fn get_voltage_resolution(&self) -> u16 {
        match self {
            SourcePowerRangeDataObject::Standard(_) => {
                StandardPowerRangeDataObject::VOLTAGE_RESOLUTION
            }
            SourcePowerRangeDataObject::Extended(_) => {
                ExtendedPowerRangeDataObject::VOLTAGE_RESOLUTION
            }
        }
    }
    pub fn get_power_type(&self) -> PowerType {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => data_object.source_power_type(),
            SourcePowerRangeDataObject::Extended(data_object) => data_object.source_power_type(),
        }
    }
}
impl core::fmt::Display for SourcePowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => {
                write!(f, "Standard({})", data_object)
            }
            SourcePowerRangeDataObject::Extended(data_object) => {
                write!(f, "Extended({})", data_object)
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
        let peak_current = match value.value() {
            0 => PeakCurrent::ConditionOne,
            1 => PeakCurrent::ConditionTwo,
            2 => PeakCurrent::ConditionThree,
            3 => PeakCurrent::ConditionFour,
            _ => unreachable!("This will never happen due to rust type safety"),
        };
        peak_current
    }
}
