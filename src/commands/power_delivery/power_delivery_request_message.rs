use arbitrary_int::u4;
use bitbybit::{bitenum, bitfield};

use crate::impl_two_byte_write_command;

use super::command_map::Command;

#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryRequestMessage {
    #[bits(0..=7, w)]
    pub voltage_selection: u8,
    #[bits(8..=11, w)]
    pub current_selection: u4,
    #[bits(12..=15, w)]
    pub power_data_object_index: PowerDataObject,
}

#[bitenum(u4, exhaustive = false)]
pub enum PowerDataObject {
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
#[bitenum(u4, exhaustive = true)]
pub enum CurrentSelection {
    _1A = 0,
    _1_25A = 1,
    _1_5A = 2,
    _1_75A = 3,
    _2A = 4,
    _2_25A = 5,
    _2_5A = 6,
    _2_75A = 7,
    _3A = 8,
    _3_25A = 9,
    _3_5A = 10,
    _3_75A = 11,
    _4A = 12,
    _4_25A = 13,
    _4_5A = 14,
    _5AOrMore = 15,
}
impl_two_byte_write_command!(
    PowerDeliveryRequestMessage,
    Command::PowerDeliveryRequestMessage
);
