use bitbybit::bitfield;
use uom::si::electric_potential::{millivolt};
use uom::si::f32::ElectricPotential;

use crate::{impl_one_byte_read_command};

use super::command_map::Command;


#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
pub struct MinimumSelectionVoltage{
    #[bits(0..=7, r)]
    raw_voltage: u8,
}

impl MinimumSelectionVoltage {
    pub const SELECTION_VOLTAGE_RESOLUTION: u16 = 200; // mV
    /// Returns the minimum selection voltage in millivolts.
    pub fn voltage(&self) -> ElectricPotential  {
        let scaled_voltage = u16::from(self.raw_voltage())  * Self::SELECTION_VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))

    }
}

impl_one_byte_read_command!(MinimumSelectionVoltage, Command::MinimumSelectionVoltage);