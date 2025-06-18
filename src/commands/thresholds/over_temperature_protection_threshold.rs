use bitbybit::bitfield;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use super::command_map::Command;

/// The OTPTHR register is defined as the OTP Threshold Temperature (°C) that triggers OTP protection function.
/// The default value for the OTPTHR is 78h (120°C).
/// Please refer to the “Overtemperature Protection and De-Rating” section for more details.
///
/// // Datasheet Name: OTPTHR
#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
pub struct OverTemperatureProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_temperature: u8,
}

impl OverTemperatureProtectionThreshold {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(u16::from(self.raw_temperature()));
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
    /// Converts a temperature in degrees Celsius to the raw temperature value.
    // TODO: Consider Better Error Handling of the different conversion failures
    pub fn convert_temperature_to_raw_temperature(
        temperature: ThermodynamicTemperature,
    ) -> Result<u8, crate::Ap33772sError> {
        if !temperature.is_finite() || !temperature.is_sign_positive() {
            return Err(crate::Ap33772sError::ConversionFailed);
        }
        let raw_value = temperature.get::<degree_celsius>() as u16;

        if raw_value > u8::MAX as u16 {
            return Err(crate::Ap33772sError::ConversionFailed);
        }

        Ok(raw_value as u8)
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
