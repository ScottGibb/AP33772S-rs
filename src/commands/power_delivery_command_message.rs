use arbitrary_int::{u4};
use bitbybit::{bitfield};

use crate::impl_one_byte_write_command;

use super::command_map::Command;


/// Command: PD_CMDMSG
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryCommandMessage {
    /// Issue Hard Reset Command
    #[bit(0, w)]
    pub HardResetEnable: bool,
    #[bit(1, w)]
    pub reserved: bool,
    #[bit(2, w)]
    pub reserved2: bool,
    #[bit(3, w)]
    pub reserved3: bool,
    #[bits(4..=7, w)]
    pub reserved4: u4,

}

impl_one_byte_write_command!(
    PowerDeliveryCommandMessage,
    Command::PowerDeliveryCommandMessage
);