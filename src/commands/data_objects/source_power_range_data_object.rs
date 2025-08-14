use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::commands::data_objects::extended_power_range_data_object::MinimumVoltage as ExtendedMinimumVoltage;
use crate::commands::data_objects::standard_power_range_data_object::MinimumVoltage as StandardMinimumVoltage;
use crate::errors::{Ap33772sError, RequestError};
use crate::types::command_structures::{
    ExtendedPowerRangeDataObject, StandardPowerRangeDataObject,
};
use crate::types::units::*;
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
    pub fn get_max_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => data_object.max_voltage(),
            SourcePowerRangeDataObject::Extended(data_object) => data_object.max_voltage(),
        }
    }
    pub fn get_max_current(&self) -> SourceMaximumCurrent {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => data_object.max_current(),
            SourcePowerRangeDataObject::Extended(data_object) => data_object.max_current(),
        }
    }
    pub fn get_min_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => {
                match data_object.minimum_voltage() {
                    Some(voltage) => match voltage {
                        StandardMinimumVoltage::_3_3 => {
                            Ok(ElectricPotential::new::<millivolt>(3300.0))
                        }
                        StandardMinimumVoltage::_3_3To5 => {
                            Ok(ElectricPotential::new::<millivolt>(5000.0))
                        }
                        _ => Err(Ap33772sError::ConversionFailed),
                    },
                    None => Err(Ap33772sError::InvalidRequest(RequestError::MissingArgument)),
                }
            }
            SourcePowerRangeDataObject::Extended(data_object) => {
                match data_object.minimum_voltage() {
                    Some(voltage) => match voltage {
                        ExtendedMinimumVoltage::Fifteen => {
                            Ok(ElectricPotential::new::<millivolt>(15000.0))
                        }
                        ExtendedMinimumVoltage::FifteenLessThanVoltageMinimumLessThanTwenty => {
                            Ok(ElectricPotential::new::<millivolt>(20000.0)) // TODO Check this!
                        }
                        _ => Err(Ap33772sError::ConversionFailed),
                    },
                    None => Err(Ap33772sError::InvalidRequest(RequestError::MissingArgument)),
                }
            }
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

/// The different operational modes that the USB C Specification Supports
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerType {
    /// In this mode, Programmable Power Supply (PPS) is not used and the voltage will at a fixed rate defined
    /// by the Power Data Object
    Fixed = 0,
    /// In this mode, Programmable Power Supply (PPS) can be used and the voltage can be tweaked dependent on
    /// the configuration of the Power Data Object
    Adjustable = 1,
}

// TODO: Add
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
pub enum SourceMaximumCurrent {
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

impl SourceMaximumCurrent {
    pub fn max_range(&self) -> ElectricCurrent {
        match self {
            SourceMaximumCurrent::LessThan1_24 => ElectricCurrent::new::<milliampere>(1240.0),
            SourceMaximumCurrent::_1_24To1_49 => ElectricCurrent::new::<milliampere>(1490.0),
            SourceMaximumCurrent::_1_50To1_74 => ElectricCurrent::new::<milliampere>(1740.0),
            SourceMaximumCurrent::_1_75To1_99 => ElectricCurrent::new::<milliampere>(1990.0),
            SourceMaximumCurrent::_2_00To2_24 => ElectricCurrent::new::<milliampere>(2240.0),
            SourceMaximumCurrent::_2_25To2_49 => ElectricCurrent::new::<milliampere>(2490.0),
            SourceMaximumCurrent::_2_50To2_74 => ElectricCurrent::new::<milliampere>(2740.0),
            SourceMaximumCurrent::_2_75To2_99 => ElectricCurrent::new::<milliampere>(2990.0),
            SourceMaximumCurrent::_3_00To3_24 => ElectricCurrent::new::<milliampere>(3240.0),
            SourceMaximumCurrent::_3_25To3_49 => ElectricCurrent::new::<milliampere>(3490.0),
            SourceMaximumCurrent::_3_50To3_74 => ElectricCurrent::new::<milliampere>(3740.0),
            SourceMaximumCurrent::_3_75To3_99 => ElectricCurrent::new::<milliampere>(3990.0),
            SourceMaximumCurrent::_4_00To4_24 => ElectricCurrent::new::<milliampere>(4240.0),
            SourceMaximumCurrent::_4_25To4_49 => ElectricCurrent::new::<milliampere>(4490.0),
            SourceMaximumCurrent::_4_50To4_99 => ElectricCurrent::new::<milliampere>(4990.0),
            SourceMaximumCurrent::Maximum => ElectricCurrent::new::<milliampere>(f32::INFINITY),
        }
    }
    pub fn min_range(&self) -> ElectricCurrent {
        match self {
            SourceMaximumCurrent::LessThan1_24 => ElectricCurrent::new::<milliampere>(0.0),
            SourceMaximumCurrent::_1_24To1_49 => ElectricCurrent::new::<milliampere>(1240.0),
            SourceMaximumCurrent::_1_50To1_74 => ElectricCurrent::new::<milliampere>(1500.0),
            SourceMaximumCurrent::_1_75To1_99 => ElectricCurrent::new::<milliampere>(1750.0),
            SourceMaximumCurrent::_2_00To2_24 => ElectricCurrent::new::<milliampere>(2000.0),
            SourceMaximumCurrent::_2_25To2_49 => ElectricCurrent::new::<milliampere>(2250.0),
            SourceMaximumCurrent::_2_50To2_74 => ElectricCurrent::new::<milliampere>(2500.0),
            SourceMaximumCurrent::_2_75To2_99 => ElectricCurrent::new::<milliampere>(2750.0),
            SourceMaximumCurrent::_3_00To3_24 => ElectricCurrent::new::<milliampere>(3000.0),
            SourceMaximumCurrent::_3_25To3_49 => ElectricCurrent::new::<milliampere>(3250.0),
            SourceMaximumCurrent::_3_50To3_74 => ElectricCurrent::new::<milliampere>(3500.0),
            SourceMaximumCurrent::_3_75To3_99 => ElectricCurrent::new::<milliampere>(3750.0),
            SourceMaximumCurrent::_4_00To4_24 => ElectricCurrent::new::<milliampere>(4000.0),
            SourceMaximumCurrent::_4_25To4_49 => ElectricCurrent::new::<milliampere>(4250.0),
            SourceMaximumCurrent::_4_50To4_99 => ElectricCurrent::new::<milliampere>(4500.0),
            SourceMaximumCurrent::Maximum => ElectricCurrent::new::<milliampere>(4990.0),
        }
    }
}
