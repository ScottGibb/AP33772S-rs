use crate::errors::Ap33772sError;
use crate::units::*;
pub mod thermal_resistance_100;
pub mod thermal_resistance_25;
pub mod thermal_resistance_50;
pub mod thermal_resistance_75;

// The following function validates that the resistance is finite and positive,
// and ensures the value fits within a u16 before conversion. This approach is
// chosen for simplicity and safety, as it covers the expected input range and
// failure modes. If more granular error handling is needed in the future,
// consider distinguishing between different failure cases.
pub fn convert_resistance_to_raw_resistance(
    resistance: ElectricalResistance,
) -> Result<u16, Ap33772sError> {
    if !resistance.is_finite() || !resistance.is_sign_positive() {
        return Err(Ap33772sError::ConversionFailed);
    }

    let raw_value = resistance.get::<ohm>();

    if raw_value > u16::MAX as f32 {
        return Err(Ap33772sError::ConversionFailed);
    }

    Ok(raw_value as u16)
}
