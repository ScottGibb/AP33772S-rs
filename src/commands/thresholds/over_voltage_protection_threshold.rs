use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use crate::commands::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
pub struct OverVoltageProtectionThreshold {
    #[bits(0..=7, r)]
    /// The raw voltage value.
    raw_voltage: u8,
}

impl OverVoltageProtectionThreshold {
    const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() as u16 * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }
}

impl_one_byte_read_command!(
    OverVoltageProtectionThreshold,
    Command::OverVoltageProtectionThreshold
);
impl_one_byte_write_command!(
    OverVoltageProtectionThreshold,
    Command::OverVoltageProtectionThreshold
);
