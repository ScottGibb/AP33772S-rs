use bitbybit::bitfield;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use super::command_map::Command;

#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
pub struct OverTemperatureProrectionThreshold {
    #[bits(0..=7, rw)]
    raw_threshold: u8,
}

impl OverTemperatureProrectionThreshold {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(u16::from(self.raw_threshold()));
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(
    OverTemperatureProrectionThreshold,
    Command::OverTemperatureProtectionThreshold
);
impl_one_byte_write_command!(
    OverTemperatureProrectionThreshold,
    Command::OverTemperatureProtectionThreshold
);
