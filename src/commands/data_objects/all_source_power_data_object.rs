use crate::commands::{
    data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject,
    power_delivery::power_delivery_request_message::PowerDataObject,
};
use bitbybit::bitenum;
use uom::si::{electric_potential::millivolt, f32::ElectricPotential};

use super::source_power_data_object::StandardPowerRangeDataObject;

pub(crate) const MAX_SOURCE_POWER_DATA_OBJECTS: usize = 7;
pub(crate) const MAX_EXTENDED_POWER_DATA_OBJECTS: usize = 5;
#[derive(Debug, PartialEq, Clone)]
pub struct AllSourceDataPowerDataObject {
    pub standard_power: [StandardPowerRangeDataObject; MAX_SOURCE_POWER_DATA_OBJECTS],
    pub extended_power: [ExtendedPowerRangeDataObject; MAX_EXTENDED_POWER_DATA_OBJECTS],
}

#[derive(Debug, PartialEq, Clone)]
pub enum SourcePowerRangeDataObject {
    Standard(StandardPowerRangeDataObject),
    Extended(ExtendedPowerRangeDataObject),
}

impl core::fmt::Display for AllSourceDataPowerDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "AllSourceDataPowerDataObject {{")?;
        writeln!(f, "  source_power: [")?;
        for (i, power_obj) in self.standard_power.iter().enumerate() {
            writeln!(f, "    [{i}]: {power_obj}")?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "  extended_power: [")?;
        for (i, power_obj) in self.extended_power.iter().enumerate() {
            writeln!(f, "    [{i}]: {power_obj}")?;
        }
        writeln!(f, "  ]")?;
        write!(f, "}}")
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AllSourceDataPowerDataObject {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AllSourceDataPowerDataObject {{\n");
        defmt::write!(f, "  source_power: [\n");
        for i in 0..MAX_SOURCE_POWER_DATA_OBJECTS {
            defmt::write!(f, "    [{}]: {}\n", i, self.source_power[i]);
        }
        defmt::write!(f, "  ]\n");
        defmt::write!(f, "  extended_power: [\n");
        for i in 0..MAX_EXTENDED_POWER_DATA_OBJECTS {
            defmt::write!(f, "    [{}]: {}\n", i, self.extended_power[i]);
        }
        defmt::write!(f, "  ]\n");
        defmt::write!(f, "}}");
    }
}

impl Default for AllSourceDataPowerDataObject {
    fn default() -> Self {
        AllSourceDataPowerDataObject {
            standard_power: [StandardPowerRangeDataObject::default();
                MAX_SOURCE_POWER_DATA_OBJECTS],
            extended_power: [ExtendedPowerRangeDataObject::default();
                MAX_EXTENDED_POWER_DATA_OBJECTS],
        }
    }
}

impl AllSourceDataPowerDataObject {
    const EXTENDER_POWER_RANGE_RESOLUTION: u8 = 100; // mV per Unit
    const STANDARD_POWER_RANGE_RESOLUTION: u8 = 200; // mV per Unit
    pub fn get_power_mode(&self, selected_data_object: &PowerDataObject) -> PowerType {
        let power_index: usize = u8::from(selected_data_object.raw_value()).into();
        match selected_data_object {
            PowerDataObject::StandardPowerRange1
            | PowerDataObject::StandardPowerRange2
            | PowerDataObject::StandardPowerRange3
            | PowerDataObject::StandardPowerRange4
            | PowerDataObject::StandardPowerRange5
            | PowerDataObject::StandardPowerRange6
            | PowerDataObject::StandardPowerRange7 => {
                self.standard_power[power_index].source_power_type()
            }
            PowerDataObject::ExtendedPowerRange8
            | PowerDataObject::ExtendedPowerRange9
            | PowerDataObject::ExtendedPowerRange10
            | PowerDataObject::ExtendedPowerRange11
            | PowerDataObject::ExtendedPowerRange12
            | PowerDataObject::ExtendedPowerRange13 => {
                self.standard_power[power_index - MAX_SOURCE_POWER_DATA_OBJECTS].source_power_type()
            }
        }
    }
    pub fn get_voltage_scaling(
        &self,
        selected_data_object: &PowerDataObject,
    ) -> Option<ElectricPotential> {
        let power_index: usize = u8::from(selected_data_object.raw_value()).into();
        match selected_data_object {
            PowerDataObject::StandardPowerRange1
            | PowerDataObject::StandardPowerRange2
            | PowerDataObject::StandardPowerRange3
            | PowerDataObject::StandardPowerRange4
            | PowerDataObject::StandardPowerRange5
            | PowerDataObject::StandardPowerRange6
            | PowerDataObject::StandardPowerRange7 => {
                let power_type = self.standard_power[power_index].source_power_type();
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerType {
    Fixed = 0,
    Adjustable = 1,
}
