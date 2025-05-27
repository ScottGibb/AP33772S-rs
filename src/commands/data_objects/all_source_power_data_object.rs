use bitbybit::bitfield;
use arbitrary_int::{u104};

use super::source_power_data_object::SourcePowerDataObject;


#[bitfield(u104, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct AllSourceDataPowerDataObject {
    #[bits(0..=15, rw)]
    pub source_power_data_object: [SourcePowerDataObject; 7],
    #[bits(0..=15, rw)]
    pub extended_power_data_object: [SourcePowerDataObject; 5],

}




#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct ExtendedPowerDataObject {

}



