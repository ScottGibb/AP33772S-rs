use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;
use bitbybit::{bitenum, bitfield};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The OperationMode register is defined as the AP33772S’s operation mode.
///
/// When `ConfigurationChannel::One`, the CC2 is connected to the CC line. See [ConfigurationChannel](crate::commands::configuration::operation_mode::ConfigurationChannel) for more details.
///
/// When `DeRatingMode::One`, the AP33772S works in de-rating (DR) mode. See [DeRatingMode](crate::commands::configuration::operation_mode::DeRatingMode) for more details.
///
/// When `power_delivery_source_connected` = `true`, the AP33772S works in Power Delivery mode.
///
/// When `legacy_source_connected` = `true`, the AP33772S works in Legacy mode (non Power Delivery Modes).
///
/// Datasheet Name: OPMODE
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct OperationMode {
    /// Leagacy Source Connected
    ///
    /// Datasheet Name: LGCYMOD
    #[bit(0, r)]
    pub legacy_source_connected: bool,
    /// Power Delivery Source Connected
    ///
    /// Datasheet Name: PDMOD
    #[bit(1, r)]
    pub power_delivery_source_connected: bool,
    // /// Reserved bits [2..5]
    // #[bits(2..=5,r)]
    // reserved: u4,
    /// Derating Mode
    ///
    /// Datasheet Name: DR
    #[bit(6, r)]
    pub derating_mode: DeRatingMode,
    /// CCFLIP - CC1 or CC2 connected
    #[bit(7, r)]
    pub configuration_channel: ConfigurationChannel,
}

/// The AP33772S supports two operation modes: Normal and Derating.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Default, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DeRatingMode {
    #[default]
    Normal = 0,
    Derating = 1,
}

/// The AP33772S supports two configuration channels: CC1 and CC2.
///
/// When `ConfigurationChannel::One`, the CC1 is connected to the CC line.
///
/// When `ConfigurationChannel::Two`, the CC2 is connected to the CC line.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigurationChannel {
    #[default]
    One = 0,
    Two = 1,
}

impl_one_byte_read_command!(OperationMode, Command::OperationMode);

impl core::fmt::Display for OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "OperationMode {{\n\
             legacy_source_connected: {},\n\
             power_delivery_source_connected: {},\n\
             derating_mode: {:?},\n\
             configuration_channel: {:?}\n\
             }}",
            self.legacy_source_connected(),
            self.power_delivery_source_connected(),
            self.derating_mode(),
            self.configuration_channel()
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OperationMode {{\n\
             legacy_source_connected: {},\n\
             power_delivery_source_connected: {},\n\
             derating_mode: {:?},\n\
             configuration_channel: {:?}\n\
             }}",
            self.legacy_source_connected(),
            self.power_delivery_source_connected(),
            self.derating_mode(),
            self.configuration_channel()
        );
    }
}
