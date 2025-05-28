use bitbybit::bitfield;
use uom::si::{electric_current::milliampere, f32::ElectricCurrent};

use super::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
pub struct OverCurrentProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_current: u8,
}

impl OverCurrentProtectionThreshold {
    const CURRENT_RESOLUTION: u16 = 50; // mA
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = u16::from(self.raw_current()) * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::from(scaled_current))
    }
}
impl_one_byte_read_command!(
    OverCurrentProtectionThreshold,
    Command::OverCurrentProtectionThreshold
);
impl_one_byte_write_command!(
    OverCurrentProtectionThreshold,
    Command::OverCurrentProtectionThreshold
);
