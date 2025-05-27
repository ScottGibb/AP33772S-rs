use bitbybit::bitfield;

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use super::command_map::Command;

/// Command: MASK
#[bitfield(u8, default = 0x03)]
#[derive(Debug, PartialEq)]
pub struct InterruptEnable{
    /// STARTED_MSK
    #[bit(0, rw)]
    pub started: bool, 
    /// READY_MSK
    #[bit(1, rw)]
    pub ready: bool,
    /// NEWPDO_MSK 
    #[bit(2, rw)]
    pub new_power_data_object: bool,
    /// UVP
    #[bit(3, rw)]
    pub under_voltage_protection : bool,
    /// OVP
    #[bit(4, rw)]
    pub over_voltage_protection : bool,
    /// OCP
    #[bit(5, rw)]
    pub over_current_protection : bool,
    /// OTP
    #[bit(6, rw)]
    pub over_temperature_protection : bool,
    #[bit(7, rw)]
    reserved: bool,

}

impl_one_byte_write_command!(InterruptEnable, Command::InterruptEnableMask);
impl_one_byte_read_command!(InterruptEnable, Command::InterruptEnableMask);