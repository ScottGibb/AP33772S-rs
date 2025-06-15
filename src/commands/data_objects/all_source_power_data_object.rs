use bitbybit::bitenum;
use uom::si::{electric_potential::millivolt, f32::ElectricPotential};

use crate::commands::{
    data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject,
    power_delivery::power_delivery_request_message::PowerDataObject,
};

use super::source_power_data_object::SourcePowerDataObject;

pub(crate) const MAX_SOURCE_POWER_DATA_OBJECTS: usize = 7;
pub(crate) const MAX_EXTENDED_POWER_DATA_OBJECTS: usize = 5;
#[derive(Debug, PartialEq, Clone)]
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

impl AllSourceDataPowerDataObject {
    const EXTENDER_POWER_RANGE_RESOLUTION: u8 = 100; // mV per Unit
    const STANDARD_POWER_RANGE_RESOLUTION: u8 = 200; // mV per Unit
    pub fn get_power_mode(&self, selected_data_object: &PowerDataObject) -> PowerType {
        let power_index: usize = u8::from(selected_data_object.raw_value()) as usize;
        match selected_data_object {
            PowerDataObject::StandardPowerRange1
            | PowerDataObject::StandardPowerRange2
            | PowerDataObject::StandardPowerRange3
            | PowerDataObject::StandardPowerRange4
            | PowerDataObject::StandardPowerRange5
            | PowerDataObject::StandardPowerRange6
            | PowerDataObject::StandardPowerRange7 => {
                self.source_power[power_index].source_power_type()
            }
            PowerDataObject::ExtendedPowerRange8
            | PowerDataObject::ExtendedPowerRange9
            | PowerDataObject::ExtendedPowerRange10
            | PowerDataObject::ExtendedPowerRange11
            | PowerDataObject::ExtendedPowerRange12
            | PowerDataObject::ExtendedPowerRange13 => {
                self.source_power[power_index - MAX_SOURCE_POWER_DATA_OBJECTS].source_power_type()
            }
        }
    }
    pub fn get_voltage_scaling(
        &self,
        selected_data_object: &PowerDataObject,
    ) -> Option<ElectricPotential> {
        let power_index: usize = u8::from(selected_data_object.raw_value()) as usize;
        match selected_data_object {
            PowerDataObject::StandardPowerRange1
            | PowerDataObject::StandardPowerRange2
            | PowerDataObject::StandardPowerRange3
            | PowerDataObject::StandardPowerRange4
            | PowerDataObject::StandardPowerRange5
            | PowerDataObject::StandardPowerRange6
            | PowerDataObject::StandardPowerRange7 => {
                let power_type = self.source_power[power_index].source_power_type();
                if power_type == PowerType::Adjustable {
                    Some(ElectricPotential::new::<millivolt>(f32::from(
                        Self::STANDARD_POWER_RANGE_RESOLUTION,
                    )))
                } else {
                    None
                }
            }
            PowerDataObject::ExtendedPowerRange8
            | PowerDataObject::ExtendedPowerRange9
            | PowerDataObject::ExtendedPowerRange10
            | PowerDataObject::ExtendedPowerRange11
            | PowerDataObject::ExtendedPowerRange12
            | PowerDataObject::ExtendedPowerRange13 => {
                let power_type = self.extended_power[power_index - MAX_SOURCE_POWER_DATA_OBJECTS]
                    .source_power_type();
                if power_type == PowerType::Adjustable {
                    Some(ElectricPotential::new::<millivolt>(f32::from(
                        Self::EXTENDER_POWER_RANGE_RESOLUTION,
                    )))
                } else {
                    None
                }
            }
        }
    }
}

#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq)]
pub enum PowerType {
    Fixed = 0,
    Adjustable = 1,
}
