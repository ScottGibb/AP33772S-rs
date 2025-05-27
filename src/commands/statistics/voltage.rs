use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;

#[bitfield(u8, default = 0x0)]
#[derive(Debug, PartialEq)]
pub struct Voltage {
    #[bits(0..=7, r)]
    /// The raw voltage value.
    raw_voltage: u8,
}

impl Voltage {
    pub const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() as u16 * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }
}

impl_one_byte_read_command!(Voltage, Command::Voltage);
