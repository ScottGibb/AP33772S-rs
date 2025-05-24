use bitbybit::bitfield;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::impl_one_byte_read_command;

use super::command_map::Command;

#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct Temperature {
    #[bits(0..=7, r)]
    raw_temperature: u8,
}
impl Temperature {
    pub const TEMPERATURE_RESOLUTION:u16 = 1; // °C

    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(u16::from(self.raw_temperature()) * Self::TEMPERATURE_RESOLUTION);
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(Temperature,Command::Temperature);