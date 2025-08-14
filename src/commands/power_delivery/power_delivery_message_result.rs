use super::command_map::Command;
use crate::impl_one_byte_read_command;
use bitbybit::{bitenum, bitfield};

/// The Power Delivery Message Result Register is defined as the message processing results of
/// the [PowerDeliveryRequestMessage](crate::commands::power_delivery::power_delivery_request_message::PowerDeliveryRequestMessage)
/// and the [PowerDeliveryCommandMessage](crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage)
///
/// Datasheet: PD_MSGRLT
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

/// The different responses that the device can give when in operation
#[bitenum(u3, exhaustive = false)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerDeliveryResponse {
    Busy = 0,
    Success = 1,
    /// Invalid Command Or Argument Provided
    Invalid = 2,
    /// Command Not Supported Or Rejected
    NotSupported = 3,
    /// No good Cyclic Redundancy Check (CRC) received after sending
    TransactionFailed = 4,
}

impl_one_byte_read_command!(
    PowerDeliveryMessageResult,
    Command::PowerDeliveryMessageResult
);
