use crate::commands::command_map::Command;
use crate::errors::Ap33772sError;
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

/// The Power Data Onject (PDO) represents the different power modes that are supported in
/// the USB C Power Delivery Specificaiton for this chip AP33772S.  The chip supports
///
///
/// [StandardPowerRangeDataObject](crate::commands::data_objects::standard_power_range_data_object::StandardPowerRangeDataObject)
///
/// With a Maximum Voltage of 21V
///
/// [ExtendedPowerRangeDataObject][crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject]
///
/// with a Maximum Voltage of 28V
///
/// 7 Standard Power Ranges are provided along with 6 Extended Power Ranges. These are not gauranteed to be implemented by every charger
/// The underlying PowerDataObjects linked above provide a method `is_detected()` which outlines if it can be used
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

/// Represents the different Currents the device will limit itself too. The actual current the device can support is
/// defined by the [SourceMaximumCurrent](crate::commands::data_objects::source_power_range_data_object::SourceMaximumCurrent)
/// If the maximum current is requested, the device will use the highest available current level. This can be requested using the
/// [negotiate maximum power function](crate::ap33772s::Ap33772s::negotiate_maximum_power_delivery)
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
    /// Returns the Operating Current in a useable `uom` ElectricCurrent. Where 5A is the "Maximum" value
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
