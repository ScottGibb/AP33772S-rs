use super::command_map::Command;
use crate::{impl_two_byte_read_command, impl_two_byte_write_command};
use bitbybit::bitfield;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

/// This struct represents the requested current of the AP33772S device.
/// It contains the raw current value and provides a method to convert it to milliamperes.
/// The requested current is the current that the device is set to output.
/// According to the datasheet:
///
/// "The IREQ register is defined as the latest request current, which depends on the PD negotiation result
/// after setting the request message (PD_REQMSG). The LSB is 10mA. If negotiation is successful
/// (PD_MSGRLT.RESPONSE=1), IREQ will be updated to the requested current (PD_REQMSG.CURRENT_SEL).
/// If negotiation is unsuccessful, IREQ will not be updated."
///
/// Datasheet Name: IREQ
#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
pub struct CurrentRequested {
    /// The raw current value.
    ///
    /// he latest requested current; LSB 10mA The value depends on the PD
    /// negotiation result after setting PD_REQMSG (CURRENT_SEL).
    ///
    /// Datasheet Name: IREQ
    #[bits(0..=15, r)]
    raw_current: u16,
}

impl CurrentRequested {
    pub const CURRENT_RESOLUTION: u16 = 10; // mA
    /// Returns the current value in milliamperes.
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = self.raw_current() * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::from(scaled_current))
    }
}

impl_two_byte_read_command!(CurrentRequested, Command::CurrentRequested);
