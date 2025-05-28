use super::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
pub struct DeRatingThreshold {
    #[bits(0..=7, rw)]
    pub raw_temperature: u8,
}

impl DeRatingThreshold {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(u16::from(self.raw_temperature()));
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(DeRatingThreshold, Command::DeRatingThreshold);
impl_one_byte_write_command!(DeRatingThreshold, Command::DeRatingThreshold);
