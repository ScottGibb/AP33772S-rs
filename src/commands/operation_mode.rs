use arbitrary_int::u4;
use bitbybit::{bitenum, bitfield};

use crate::impl_register;

use super::command_map::Command;

/// STATUS register
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct OperationMode {
    /// LGCYMOD - Legacy source connected (non-PD)
    #[bit(0, r)]
    pub legacy_source_connected: bool,

    /// PDMOD - PD source connected
    #[bit(1, r)]
    pub power_delivery_source_connected: bool,

    /// Reserved bits [2..5]
    #[bits(2..=5,r)]
    pub reserved: u4,

    /// DR - Derating mode
    #[bit(6, r)]
    pub derating_mode: DeRatingMode,

    /// CCFLIP - CC1 or CC2 connected
    #[bit(7, r)]
    pub configuration_channel: ConfigurationChannel,
}

#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Default)]
pub enum DeRatingMode {
    #[default]
    Normal = 0,
    Derating = 1,
}
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Default)]
pub enum ConfigurationChannel {
    #[default]
    One = 0,
    Two = 1,
}

impl_register!(OperationMode, Command::OperationMode);
