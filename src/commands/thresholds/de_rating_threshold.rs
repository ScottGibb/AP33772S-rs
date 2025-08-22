use crate::commands::command_map::Command;
use crate::errors::Ap33772sError;
use crate::units::*;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// The DeRatingThreshold register is defined as the De-Rating Threshold Temperature (°C) that triggers
/// the De-Rating function. The default value for the DeRatingThreshold is 78h (120°C).
/// Please refer to the “Overtemperature Protection and De-Rating” section for more details.
///
/// // Datasheet Name: DRTHR
#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeRatingThreshold {
    /// The temperature threshold triggers De-Rating function; the default value for DRTHR is 78h (120°C)
    #[bits(0..=7, rw)]
    pub raw_temperature: u8,
}

impl DeRatingThreshold {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        Self::convert_raw_temperature_to_temperature(self.raw_temperature())
    }

    pub fn convert_temperature_to_raw_temperature(
        temperature: ThermodynamicTemperature,
    ) -> Result<u8, Ap33772sError> {
        let temp_celsius = temperature.get::<degree_celsius>();

        if temp_celsius > u32::from(u8::MAX) {
            return Err(Ap33772sError::ConversionFailed);
        }

        Ok(temp_celsius as u8)
    }
    pub fn convert_raw_temperature_to_temperature(raw_temperature: u8) -> ThermodynamicTemperature {
        let scaled_temperature = u32::from(raw_temperature);
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(DeRatingThreshold, Command::DeRatingThreshold);
impl_one_byte_write_command!(DeRatingThreshold, Command::DeRatingThreshold);
