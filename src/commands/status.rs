use bitbybit::bitfield;

use crate::impl_register;

use super::command_map::Command;

/// MASK
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct Status{
    /// STARTED_MSK
    #[bit(0, r)]
    pub started: bool, 
    /// READY_MSK
    #[bit(1, r)]
    pub ready: bool,
    /// NEWPDO_MSK 
    #[bit(2, r)]
    pub new_power_data_object: bool,
    /// UVP
    #[bit(3, r)]
    pub under_voltage_protection : bool,
    /// OVP
    #[bit(4, r)]
    pub over_voltage_protection : bool,
    /// OCP
    #[bit(5, r)]
    pub over_current_protection : bool,
    /// OTP
    #[bit(6, r)]
    pub over_temperature_protection : bool,
    #[bit(7, r)]
    pub reserved: bool,

}

impl_register!(Status, Command::Status);