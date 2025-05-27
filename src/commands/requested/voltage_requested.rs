use bitbybit::bitfield;
use uom::si::electric_potential::{millivolt};
use uom::si::f32::ElectricPotential;

use super::command_map::Command;
use crate::{impl_two_byte_read_command, impl_two_byte_write_command};

#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
pub struct VoltageRequested {
    #[bits(0..=15, r)]
    raw_voltage: u16,
}

impl VoltageRequested {
    pub const VOLTAGE_RESOLUTION: u16 = 50; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }
}
impl_two_byte_read_command!(VoltageRequested, Command::VoltageRequested);
impl_two_byte_write_command!(VoltageRequested, Command::VoltageRequested);
