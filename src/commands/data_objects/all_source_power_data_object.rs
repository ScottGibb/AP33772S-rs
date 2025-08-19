use super::standard_power_range_data_object::StandardPowerRangeDataObject;
use crate::commands::data_objects::source_power_range_data_object::SourcePowerRangeDataObject;
use crate::commands::{
    data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject,
    power_delivery::power_delivery_request_message::PowerDataObject,
};

/// The amount of Standard Power Data Objects
pub(crate) const MAX_STANDARD_POWER_DATA_OBJECTS: usize = 7;
/// The amount of Extended Power Data Objects
pub(crate) const MAX_EXTENDED_POWER_DATA_OBJECTS: usize = 6;

/// Represents all source power data objects for the AP33772S.
/// Each USB C Power Supply device will have some of these objects.
/// In the event that it does not the underlying power data objects `is_detected` methods will be false.
#[derive(Debug, PartialEq, Clone)]
pub struct AllSourceDataPowerDataObject {
    pub power_data_objects: [SourcePowerRangeDataObject;
        MAX_STANDARD_POWER_DATA_OBJECTS + MAX_EXTENDED_POWER_DATA_OBJECTS],
}

impl core::fmt::Display for AllSourceDataPowerDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "AllSourceDataPowerDataObject {{")?;
        writeln!(f, "  power_data_objects: [")?;
        for (i, power_obj) in self.power_data_objects.iter().enumerate() {
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
        defmt::write!(f, "  power_data_objects: [\n");
        for (i, power_obj) in self.power_data_objects.iter().enumerate() {
            defmt::write!(f, "    [{:?}] : {=?}\n", i, power_obj);
        }
        defmt::write!(f, "  ]\n");
        defmt::write!(f, "}}");
    }
}

impl Default for AllSourceDataPowerDataObject {
    /// Returns an empty AllSourceDataPowerDataObject.
    fn default() -> Self {
        AllSourceDataPowerDataObject {
            power_data_objects: [
                // Source Standard Power Data Objects 1-7
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Standard(StandardPowerRangeDataObject::default()),
                // Source Extended Power Data Objects 8-13
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
                SourcePowerRangeDataObject::Extended(ExtendedPowerRangeDataObject::default()),
            ],
        }
    }
}

impl AllSourceDataPowerDataObject {
    /// Returns a reference to the power data object at the given index.
    pub fn get_power_data_object(&self, index: PowerDataObject) -> &SourcePowerRangeDataObject {
        // These assertions should never fire, but we include them for safety. The PowerDataObjects are always in the range 1-13
        assert!(
            usize::from(index) < self.power_data_objects.len(),
            "Index out of bounds for power data objects"
        );
        assert_ne!(
            usize::from(index),
            0,
            "Power Data Object Should never be zero!"
        );
        let index = usize::from(index) - 1;
        &self.power_data_objects[index]
    }
}
