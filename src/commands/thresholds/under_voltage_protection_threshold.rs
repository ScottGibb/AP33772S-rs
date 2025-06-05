use bitbybit::{bitenum, bitfield};

use crate::{
    commands::command_map::Command, impl_one_byte_read_command, impl_one_byte_write_command,
};

/// The UVPTHR register is defined as the UVP Threshold Voltage that triggers UVP protection function.
/// The UVP Threshold Voltage is the UVPTHR percentage (%) of the VREQ voltage.
/// The default value for the UVPTHR is 01h (80%).
///
/// Please refer to the “Undervoltage Protection” section for more details.
///
/// // Datasheet Name: UVPTHR
#[bitfield(u8, default = 0x01)]
#[derive(Debug, PartialEq)]
pub struct UnderVoltageProtectionThreshold {
    #[bits(0..=3, rw)]
    threshold: Option<UnderVoltageThreshold>,
    // #[bits(4..=7, rw)]
    // reserved: u4,
}

/// The UnderVoltageThreshold enum defines the possible values for the UVP threshold.
/// The UVP Threshold Voltage is UVPTHR percentage (%) of [VoltageRequested](crate::commands::requested::voltage_requested::VoltageRequested)
#[derive(Debug, PartialEq)]
#[bitenum(u4, exhaustive = false)]
pub enum UnderVoltageThreshold {
    EightyPercent = 0,
    SeventyFivePercent = 1,
    SeventyPercent = 2,
    // Other values are reserved and should not be used.
}

impl_one_byte_read_command!(
    UnderVoltageProtectionThreshold,
    Command::UnderVoltageProtectionThreshold
);
impl_one_byte_write_command!(
    UnderVoltageProtectionThreshold,
    Command::UnderVoltageProtectionThreshold
);
