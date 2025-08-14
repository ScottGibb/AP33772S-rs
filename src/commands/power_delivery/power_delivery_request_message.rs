use super::command_map::Command;
use crate::error::Ap33772sError;
use crate::impl_two_byte_write_command;
use crate::types::units::*;
use bitbybit::{bitenum, bitfield};
#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerDeliveryRequestMessage {
    #[bits(0..=7, w)]
    pub voltage_selection: u8, // Handle the different units depending on the power mode
    #[bits(8..=11, w)]
    pub current_selection: OperatingCurrentSelection,
    #[bits(12..=15, w)]
    pub power_data_object_index: PowerDataObject,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bitenum(u4, exhaustive = false)]
pub enum PowerDataObject {
    StandardPowerRange1 = 1,
    StandardPowerRange2 = 2,
    StandardPowerRange3 = 3,
    StandardPowerRange4 = 4,
    StandardPowerRange5 = 5,
    StandardPowerRange6 = 6,
    StandardPowerRange7 = 7,
    ExtendedPowerRange8 = 8,
    ExtendedPowerRange9 = 9,
    ExtendedPowerRange10 = 10,
    ExtendedPowerRange11 = 11,
    ExtendedPowerRange12 = 12,
    ExtendedPowerRange13 = 13,
}

impl From<PowerDataObject> for usize {
    fn from(value: PowerDataObject) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for PowerDataObject {
    type Error = Ap33772sError;

    /// Converts a usize value to a PowerDataObject enum variant.
    /// Returns an error if the value doesn't correspond to a valid variant.
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerDataObject::StandardPowerRange1),
            1 => Ok(PowerDataObject::StandardPowerRange2),
            2 => Ok(PowerDataObject::StandardPowerRange3),
            3 => Ok(PowerDataObject::StandardPowerRange4),
            4 => Ok(PowerDataObject::StandardPowerRange5),
            5 => Ok(PowerDataObject::StandardPowerRange6),
            6 => Ok(PowerDataObject::StandardPowerRange7),
            7 => Ok(PowerDataObject::ExtendedPowerRange8),
            8 => Ok(PowerDataObject::ExtendedPowerRange9),
            9 => Ok(PowerDataObject::ExtendedPowerRange10),
            10 => Ok(PowerDataObject::ExtendedPowerRange11),
            11 => Ok(PowerDataObject::ExtendedPowerRange12),
            12 => Ok(PowerDataObject::ExtendedPowerRange13),
            _ => Err(Ap33772sError::ConversionFailed),
        }
    }
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

#[derive(Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bitenum(u4, exhaustive = true)]
pub enum OperatingCurrentSelection {
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
    Maximum = 15,
}
impl OperatingCurrentSelection {
    pub fn current(&self) -> ElectricCurrent {
        use OperatingCurrentSelection::*;
        let current = match self {
            _1A => 1000.0,
            _1_25A => 1250.0,
            _1_5A => 1500.0,
            _1_75A => 1750.0,
            _2A => 2000.0,
            _2_25A => 2250.0,
            _2_5A => 2500.0,
            _2_75A => 2750.0,
            _3A => 3000.0,
            _3_25A => 3250.0,
            _3_5A => 3500.0,
            _3_75A => 3750.0,
            _4A => 4000.0,
            _4_25A => 4250.0,
            _4_5A => 4500.0,
            Maximum => 5000.0, // or more
        };
        ElectricCurrent::new::<milliampere>(current)
    }
}

impl TryFrom<usize> for OperatingCurrentSelection {
    type Error = Ap33772sError;

    /// Converts a u8 value to a CurrentSelection enum variant.
    /// Returns an error if the value doesn't correspond to a valid variant.
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperatingCurrentSelection::_1A),
            1 => Ok(OperatingCurrentSelection::_1_25A),
            2 => Ok(OperatingCurrentSelection::_1_5A),
            3 => Ok(OperatingCurrentSelection::_1_75A),
            4 => Ok(OperatingCurrentSelection::_2A),
            5 => Ok(OperatingCurrentSelection::_2_25A),
            6 => Ok(OperatingCurrentSelection::_2_5A),
            7 => Ok(OperatingCurrentSelection::_2_75A),
            8 => Ok(OperatingCurrentSelection::_3A),
            9 => Ok(OperatingCurrentSelection::_3_25A),
            10 => Ok(OperatingCurrentSelection::_3_5A),
            11 => Ok(OperatingCurrentSelection::_3_75A),
            12 => Ok(OperatingCurrentSelection::_4A),
            13 => Ok(OperatingCurrentSelection::_4_25A),
            14 => Ok(OperatingCurrentSelection::_4_5A),
            15 => Ok(OperatingCurrentSelection::Maximum),
            _ => Err(Ap33772sError::ConversionFailed),
        }
    }
}

pub const CURRENT_SELECTIONS: [OperatingCurrentSelection; 16] = {
    use OperatingCurrentSelection::*;
    [
        _1A, _1_25A, _1_5A, _1_75A, _2A, _2_25A, _2_5A, _2_75A, _3A, _3_25A, _3_5A, _3_75A, _4A,
        _4_25A, _4_5A, Maximum,
    ]
};
impl core::fmt::Display for OperatingCurrentSelection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use OperatingCurrentSelection::*;
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
            Maximum => "5A or > 5A ",
        };
        write!(f, "{s}")
    }
}

impl_two_byte_write_command!(
    PowerDeliveryRequestMessage,
    Command::PowerDeliveryRequestMessage
);
