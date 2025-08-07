use super::command_map::Command;
use crate::impl_two_byte_write_command;
use bitbybit::{bitenum, bitfield};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerDeliveryRequestMessage {
    #[bits(0..=7, w)]
    pub voltage_selection: u8, // Handle the different units depending on the power mode
    #[bits(8..=11, w)]
    pub current_selection: CurrentSelection,
    #[bits(12..=15, w)]
    pub power_data_object_index: PowerDataObject,
}

#[derive(Debug, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bitenum(u4, exhaustive = false)]
#[repr(u8)]
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

impl core::fmt::Display for PowerDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            PowerDataObject::StandardPowerRange1 => "Standard Power Range 1",
            PowerDataObject::StandardPowerRange2 => "Standard Power Range 2",
            PowerDataObject::StandardPowerRange3 => "Standard Power Range 3",
            PowerDataObject::StandardPowerRange4 => "Standard Power Range 4",
            PowerDataObject::StandardPowerRange5 => "Standard Power Range 5",
            PowerDataObject::StandardPowerRange6 => "Standard Power Range 6",
            PowerDataObject::StandardPowerRange7 => "Standard Power Range 7",
            PowerDataObject::ExtendedPowerRange8 => "Extended Power Range 8",
            PowerDataObject::ExtendedPowerRange9 => "Extended Power Range 9",
            PowerDataObject::ExtendedPowerRange10 => "Extended Power Range 10",
            PowerDataObject::ExtendedPowerRange11 => "Extended Power Range 11",
            PowerDataObject::ExtendedPowerRange12 => "Extended Power Range 12",
            PowerDataObject::ExtendedPowerRange13 => "Extended Power Range 13",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bitenum(u4, exhaustive = true)]
#[repr(u8)]
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
pub const CURRENT_SELECTIONS: [CurrentSelection; 16] = [
    CurrentSelection::_1A,
    CurrentSelection::_1_25A,
    CurrentSelection::_1_5A,
    CurrentSelection::_1_75A,
    CurrentSelection::_2A,
    CurrentSelection::_2_25A,
    CurrentSelection::_2_5A,
    CurrentSelection::_2_75A,
    CurrentSelection::_3A,
    CurrentSelection::_3_25A,
    CurrentSelection::_3_5A,
    CurrentSelection::_3_75A,
    CurrentSelection::_4A,
    CurrentSelection::_4_25A,
    CurrentSelection::_4_5A,
    CurrentSelection::_5AOrMore,
];
impl core::fmt::Display for CurrentSelection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use CurrentSelection::*;
        let s = match self {
            _1A => "1A",
            _1_25A => "1.25A",
            _1_5A => "1.5A",
            _1_75A => "1.75A",
            _2A => "2A",
            _2_25A => "2.25A",
            _2_5A => "2.5A",
            _2_75A => "2.75A",
            _3A => "3A",
            _3_25A => "3.25A",
            _3_5A => "3.5A",
            _3_75A => "3.75A",
            _4A => "4A",
            _4_25A => "4.25A",
            _4_5A => "4.5A",
            _5AOrMore => "5A or > 5A ",
        };
        write!(f, "{s}")
    }
}

impl_two_byte_write_command!(
    PowerDeliveryRequestMessage,
    Command::PowerDeliveryRequestMessage
);
