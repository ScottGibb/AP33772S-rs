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
