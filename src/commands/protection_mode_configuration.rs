use arbitrary_int::u3;
use bitbybit::bitfield;

use crate::impl_register;

use super::command_map::Command;

/// CONFIG
#[bitfield(u8, default = 0xF8)]
#[derive(Debug, PartialEq)]
pub struct SystemModeConfiguration {
    /// Reserved
    #[bits(0..=2, rw)]
    reserved: u3,
    #[bit(3, rw)]
    under_voltage_protection_enabled: bool,
    #[bit(4, rw)]
    over_voltage_protection_enabled: bool,
    #[bit(5, rw)]
    over_current_protection_enabled: bool,
    #[bit(6, rw)]
    over_temperature_protection_enabled: bool,
    #[bit(7, rw)]
    derating_function_enabled: bool,
}


impl_register!(SystemModeConfiguration, Command::SystemConfiguration);

