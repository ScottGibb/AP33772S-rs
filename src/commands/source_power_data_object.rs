use bitbybit::bitfield;
use arbitrary_int::{u104};

use crate::impl_register;

use super::command_map::Command;
#[bitfield(u104, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct AllSourceDataPowerDataObject {
    #[bits(0..=7, rw)]
    pub source_power_data_object: [SourcePowerDataObject; 13],
}

#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct SourcePowerDataObject {

}

// TODO: Resolve this multiple defintions for one enum
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange1);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange2);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange3);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange4);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange5);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange6);
impl_register!(SourcePowerDataObject, Command::SourceStandardPowerRange7);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange8);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange9);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange10);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange11);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange12);
impl_register!(SourcePowerDataObject, Command::SourceExtendedPowerRange13);


