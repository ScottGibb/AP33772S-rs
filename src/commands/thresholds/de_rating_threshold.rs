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

    /// Converts a temperature in degrees Celsius to the raw temperature value.
    pub fn convert_temperature_to_raw_temperature(
        temperature: ThermodynamicTemperature,
    ) -> Result<u8, crate::Ap33772sError> {
        if !temperature.is_finite() || !temperature.is_sign_positive() {
            return Err(crate::Ap33772sError::ConversionError);
        }
        let raw_value = temperature.get::<degree_celsius>() as u16;

        if raw_value > u8::MAX as u16 {
            return Err(crate::Ap33772sError::ConversionError);
        }

        Ok(raw_value as u8)
    }
}

impl_one_byte_read_command!(DeRatingThreshold, Command::DeRatingThreshold);
impl_one_byte_write_command!(DeRatingThreshold, Command::DeRatingThreshold);
