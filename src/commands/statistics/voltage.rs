use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use crate::commands::command_map::Command;
use crate::impl_two_byte_read_command;

/// This struct represents the voltage of the AP33772S device.
/// It contains the raw voltage value and provides a method to convert it to millivolts.
///
/// Datasheet Name: VOLTAGE
#[bitfield(u16, default = 0x0)]
#[derive(Debug, PartialEq)]
pub struct Voltage {
    #[bits(0..=15, r)]
    /// The raw voltage value.
    ///
    /// Datasheet Name: VOLTAGE
    raw_voltage: u16,
}

impl Voltage {
    pub const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::try_from(scaled_voltage).unwrap())
    }
}

impl_two_byte_read_command!(Voltage, Command::Voltage);
