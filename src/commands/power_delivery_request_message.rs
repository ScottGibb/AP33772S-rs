use arbitrary_int::{u4};
use bitbybit::{bitenum, bitfield};

use crate::impl_dual_register;

use super::command_map::Command;


/// CONFIG
#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryRequestMessage {
    #[bits(0..=7, w)]
    voltage_selection: u8,
    #[bits(8..=11, w)]
    current_selection: u4,
    #[bits(12..=15, w)]
    power_data_object_index: PowerDataObject,
}

#[bitenum(u4, exhaustive=false)]
pub enum PowerDataObject{
    StandardPowerRange1 = 0x00,
    StandardPowerRange2 = 0x01,
    StandardPowerRange3 = 0x02,
    StandardPowerRange4 = 0x03,
    StandardPowerRange5 = 0x04,
    StandardPowerRange6 = 0x05,
    StandardPowerRange7 = 0x06,
    ExtendedPowerRange8 = 0x07,
    ExtendedPowerRange9 = 0x08,
    ExtendedPowerRange10 = 0x09,
    ExtendedPowerRange11 = 0x0A,
    ExtendedPowerRange12 = 0x0B,
    ExtendedPowerRange13 = 0x0C,
}

impl_dual_register!(
    PowerDeliveryRequestMessage,
    Command::PowerDeliveryRequestMessage
);
