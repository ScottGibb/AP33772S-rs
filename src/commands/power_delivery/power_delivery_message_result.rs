use bitbybit::{bitenum, bitfield};

use crate::impl_one_byte_read_command;

use super::command_map::Command;

#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryMessageResult {
    #[bits(0..=2, r)]
    response: Option<Response>,
    // #[bit(3, r)]
    // reserved: bool,
    // #[bits(4..=7, r)]
    // reserved1: u4,
}

#[bitenum(u3, exhaustive = false)]
#[derive(Debug, PartialEq)]
pub enum Response {
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
