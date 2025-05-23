use bitbybit::bitfield;

use crate::{impl_one_byte_read_command};

use super::command_map::Command;


#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
pub struct MinimumSelectionVoltage{
    #[bits(0..=7, r)]
    pub voltage: u8,
}

impl_one_byte_read_command!(MinimumSelectionVoltage, Command::MinimumSelectionVoltage);