use super::command_map::Command;
use crate::impl_one_byte_read_command;
use bitbybit::{bitenum, bitfield};

#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerDeliveryMessageResult {
    #[bits(0..=2, r)]
    response: Option<PowerDeliveryResponse>,
    // #[bit(3, r)]
    // reserved: bool,
    // #[bits(4..=7, r)]
    // reserved1: u4,
}

#[bitenum(u3, exhaustive = false)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerDeliveryResponse {
    Busy = 0,
    Success = 1,
    Invalid = 2,
    NotSupported = 3,
    TransactionFailed = 4,
}

impl_one_byte_read_command!(
    PowerDeliveryMessageResult,
    Command::PowerDeliveryMessageResult
);
