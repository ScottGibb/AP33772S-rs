use uom::si::f32::ElectricalResistance;

use crate::Ap33772sError;

use super::command_map;

pub mod thermal_resistance_100;
pub mod thermal_resistance_25;
pub mod thermal_resistance_50;
pub mod thermal_resistance_75;

// TODO: Fix This / is there a better way to do this?
// TODO: Consider Better Error Handling of the different conversion failures
pub fn convert_resistance_to_raw_resistance(
    resistance: ElectricalResistance,
) -> Result<u16, Ap33772sError> {
    if !resistance.is_finite() || !resistance.is_sign_positive() {
        return Err(Ap33772sError::ConversionError);
    }

    let raw_value = resistance.get::<uom::si::electrical_resistance::ohm>();

    if raw_value > u16::MAX as f32 {
        return Err(Ap33772sError::ConversionError);
    }

    Ok(raw_value as u16)
}
