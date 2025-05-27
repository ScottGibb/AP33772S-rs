use arbitrary_int::u4;
use bitbybit::{bitenum, bitfield};

use crate::{
    commands::command_map::Command, impl_one_byte_read_command, impl_one_byte_write_command,
};

#[bitfield(u8, default = 0x01)]
#[derive(Debug, PartialEq)]
pub struct UnderVoltageProtectionThreshold {
    #[bits(0..=3, rw)]
    threshold: Option<UnderVoltageThreshold>,
    #[bits(4..=7, rw)]
    reserved: u4,
}

#[bitenum(u4, exhaustive = false)]
pub enum UnderVoltageThreshold {
    EightyPercent = 0,
    SeventyFivePercent = 1,
    SeventyPercent = 2,
}

impl_one_byte_read_command!(
    UnderVoltageProtectionThreshold,
    Command::UnderVoltageProtectionThreshold
);
impl_one_byte_write_command!(
    UnderVoltageProtectionThreshold,
    Command::UnderVoltageProtectionThreshold
);
