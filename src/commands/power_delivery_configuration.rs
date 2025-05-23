use arbitrary_int::{u1, u4};
use bitbybit::bitfield;

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use super::command_map::Command;

/// Command: CONFIG
#[bitfield(u8, default = 0x03)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryConfiguration {
    /// EPR_MODE_EN
    #[bit(0, rw)]
    extended_power_delivery_enabled: bool,
    /// PPS_AVS_EN
    #[bit(1, rw)]
    programmable_power_delivery_and_adjustable_power_supply_enabled: bool,
    #[bit(2, rw)]
    reserved: u1,
    #[bit(3, rw)]
    reserved2: u1,
    #[bits(4..=7,rw)]
    reserved3: u4,
}

impl_one_byte_write_command!(
    PowerDeliveryConfiguration,
    Command::PowerDeliveryConfiguration
);
impl_one_byte_read_command!(
    PowerDeliveryConfiguration,
    Command::PowerDeliveryConfiguration
);
