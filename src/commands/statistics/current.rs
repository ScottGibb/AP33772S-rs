use bitbybit::bitfield;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;

#[bitfield(u8, default = 0x0)]
#[derive(Debug, PartialEq)]
pub struct Current {
    #[bits(0..=7, r)]
    raw_current: u8,
}

impl Current {
    pub const CURRENT_RESOLUTION: u16 = 24; // mA
    /// Returns the current value in milliamperes.
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = u16::from(self.raw_current()) * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::from(scaled_current))
    }
}
impl_one_byte_read_command!(Current, Command::Current);
