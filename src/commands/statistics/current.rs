use crate::commands::command_map::Command;
use crate::{Ap33772sError, impl_one_byte_read_command};
use bitbybit::bitfield;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

/// This struct represents the current of the AP33772S device.
/// It contains the raw current value and provides a method to convert it to milliamperes.
///
/// Datasheet Name: CURRENT
#[bitfield(u8, default = 0x0)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Current {
    /// The raw current value.
    ///
    /// Datasheet Name: CURRENT
    #[bits(0..=7, r)]
    raw_current: u8,
}

/// Full Resolution Required is: 5000mA based on the datasheet with CURRENT_SEL being 5.00A
/// 8 Bit Unsigned Integer is 0 - 255
/// 24mA Resolution means:
/// - 0 = 0mA
/// - 1 = 24mA
/// - 2 = 48mA
/// - ...
/// - 208 = 4992mA
/// - 209 * 24 = 5016mA this requires U16
/// This means the multiplication should never surpass u16 and thus should be a checked multiplication
impl Current {
    pub const CURRENT_RESOLUTION: u16 = 24; // mA
    /// Returns the current value in milliamperes.
    pub fn current(&self) -> Result<ElectricCurrent, Ap33772sError> {
        u16::from(self.raw_current())
            .checked_mul(Self::CURRENT_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)
            .map(|scaled_current| ElectricCurrent::new::<milliampere>(f32::from(scaled_current)))
    }
}
impl_one_byte_read_command!(Current, Command::Current);
