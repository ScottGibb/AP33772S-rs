use super::command_map::Command;
use crate::ap33772s::Ap33772sError;
use crate::types::units::*;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// The OTPTHR register is defined as the OTP Threshold Temperature (°C) that triggers OTP protection function.
/// The default value for the OTPTHR is 78h (120°C).
/// Please refer to the “Overtemperature Protection and De-Rating” section for more details.
///
/// // Datasheet Name: OTPTHR
#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OverTemperatureProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_temperature: u8,
}

impl OverTemperatureProtectionThreshold {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        Self::convert_raw_temperature_to_temperature(self.raw_temperature())
    }
    /// Converts a temperature in degrees Celsius to the raw temperature value.
    // TODO: Consider Better Error Handling of the different conversion failures
    pub fn convert_temperature_to_raw_temperature(
        temperature: ThermodynamicTemperature,
    ) -> Result<u8, Ap33772sError> {
        if !temperature.is_finite() || !temperature.is_sign_positive() {
            return Err(Ap33772sError::ConversionFailed);
        }
        let raw_value = temperature.get::<degree_celsius>() as u16;

        if raw_value > u8::MAX as u16 {
            return Err(Ap33772sError::ConversionFailed);
        }

        Ok(raw_value as u8)
    }

    pub fn convert_raw_temperature_to_temperature(raw_temperature: u8) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(raw_temperature);
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(
    OverTemperatureProtectionThreshold,
    Command::OverTemperatureProtectionThreshold
);
impl_one_byte_write_command!(
    OverTemperatureProtectionThreshold,
    Command::OverTemperatureProtectionThreshold
);
