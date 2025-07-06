use crate::{
    commands::command_map::Command, impl_one_byte_read_command, impl_one_byte_write_command,
};
use bitbybit::bitfield;

#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VDCTHR {
    #[bits(0..=7, rw)]
    /// percentage(%) difference between VREQ and VOLTAGE
    pub percentage: u8,
}

// TODO: Solve this mystery register
// Doesnt show up in datasheet but is in the arduino vendor support examples

impl_one_byte_read_command!(VDCTHR, Command::VDCTHR);
impl_one_byte_write_command!(VDCTHR, Command::VDCTHR);
