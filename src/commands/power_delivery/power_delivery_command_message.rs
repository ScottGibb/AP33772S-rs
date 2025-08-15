use crate::commands::command_map::Command;
use crate::impl_one_byte_write_command;
use bitbybit::bitfield;

/// Command: PD_CMDMSG
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerDeliveryCommandMessage {
    /// Issue Hard Reset Command
    #[bit(0, w)]
    pub HardResetEnable: bool,
    // #[bit(1, w)]
    // reserved: bool,
    // #[bit(2, w)]
    // reserved2: bool,
    // #[bit(3, w)]
    // reserved3: bool,
    // #[bits(4..=7, w)]
    // reserved4: u4,
}

impl_one_byte_write_command!(
    PowerDeliveryCommandMessage,
    Command::PowerDeliveryCommandMessage
);
