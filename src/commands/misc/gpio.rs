use bitbybit::bitfield;

use crate::{
    commands::command_map::Command, impl_one_byte_read_command, impl_one_byte_write_command,
};

/// TODO: Find out more about this mystery struct
///
/// Datasheet Name: GPIO
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct Gpio {
    #[bits(0..=7, rw)]
    pub unknown: u8,
}

impl_one_byte_read_command!(Gpio, Command::Gpio);
impl_one_byte_write_command!(Gpio, Command::Gpio);
