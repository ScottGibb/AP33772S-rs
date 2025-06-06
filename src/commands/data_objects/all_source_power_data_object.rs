use bitbybit::bitenum;

use crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject;

use super::source_power_data_object::SourcePowerDataObject;

pub(crate) const MAX_SOURCE_POWER_DATA_OBJECTS: usize = 7;
pub(crate) const MAX_EXTENDED_POWER_DATA_OBJECTS: usize = 5;
#[derive(Debug, PartialEq)]
pub struct AllSourceDataPowerDataObject {
    pub source_power: [SourcePowerDataObject; MAX_SOURCE_POWER_DATA_OBJECTS],
    pub extended_power: [ExtendedPowerRangeDataObject; MAX_EXTENDED_POWER_DATA_OBJECTS],
}

impl Default for AllSourceDataPowerDataObject {
    fn default() -> Self {
        AllSourceDataPowerDataObject {
            source_power: [SourcePowerDataObject::default(); MAX_SOURCE_POWER_DATA_OBJECTS],
            extended_power: [ExtendedPowerRangeDataObject::default();
                MAX_EXTENDED_POWER_DATA_OBJECTS],
        }
    }
}

#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq)]
pub enum PowerType {
    Fixed = 0,
    Adjustable = 1,
}
