use super::command_map::Command;
use crate::{impl_two_byte_read_command, impl_two_byte_write_command};
use bitbybit::bitfield;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
pub struct CurrentRequested {
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
impl_two_byte_write_command!(CurrentRequested, Command::CurrentRequested);
