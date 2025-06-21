use bitbybit::bitfield;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;

/// This struct represents the current of the AP33772S device.
/// It contains the raw current value and provides a method to convert it to milliamperes.
///
/// Datasheet Name: CURRENT
#[bitfield(u8, default = 0x0)]
#[derive(Debug, PartialEq)]
pub struct Current {
    /// The raw current value.
    ///
    /// Datasheet Name: CURRENT
    #[bits(0..=7, r)]
    raw_current: u8,
}

impl Current {
    pub const CURRENT_RESOLUTION: u16 = 24; // mA
    /// Returns the current value in milliamperes.
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = u16::try_from(self.raw_current()).unwrap() * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::try_from(scaled_current).unwrap())
    }
}
impl_one_byte_read_command!(Current, Command::Current);
