use crate::commands::command_map::Command;
use crate::errors::Ap33772sError;
use crate::types::units::*;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// The OCPTHR register is defined as the OCP Threshold Current that triggers OCP protection function.
/// The OCP Threshold Current is 110% of the OCPTHR current value. The default value for the OCPTHR is
/// 00h and the LSB is 50mA.
///
/// If the OCPTHR is set to 0, the OCP Threshold Current will be updated to 110%
/// of the selected [Source Power Data Object](crate::commands::data_objects::source_power_range_data_object::SourcePowerRangeDataObject)
/// (POD) maximum current after successful negotiation with the PD source.
/// Please refer to the “Overcurrent Protection” section for more details
///
/// // Datasheet Name: OCPTHR
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OverCurrentProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_current: u8,
}

// Maximum Current of 5000 mA (5A) with 50mA resolution
// U8 can hold values up to 255
// 0 = 0mA
// 1 = 50mA
// 2 = 100mA
// ...
// 100 = 5000mA (5A)
// This means the maximum raw value is 100
// 100 * 50 = 5000mA U16 required
// Therefore the current should be checked multiplied
impl OverCurrentProtectionThreshold {
    const CURRENT_RESOLUTION: u16 = 50; // mA

    /// Returns the current value in milliampere.
    pub fn current(&self) -> Result<ElectricCurrent, Ap33772sError> {
        u16::from(self.raw_current())
            .checked_mul(Self::CURRENT_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)
            .map(|scaled_current| ElectricCurrent::new::<milliampere>(f32::from(scaled_current)))
    }
    /// TODO: Look to generigy and combine into a helper function
    // TODO: Consider Better Error Handling of the different conversion failures
    pub fn convert_current_to_raw_current(current: ElectricCurrent) -> Result<u8, Ap33772sError> {
        if !current.is_finite() || !current.is_sign_positive() {
            return Err(Ap33772sError::ConversionFailed);
        }
        let raw_value = current.get::<milliampere>() / Self::CURRENT_RESOLUTION as f32;

        if raw_value > f32::from(u8::MAX) {
            return Err(Ap33772sError::ConversionFailed);
        }

        Ok(raw_value as u8)
    }
}
impl_one_byte_read_command!(
    OverCurrentProtectionThreshold,
    Command::OverCurrentProtectionThreshold
);
impl_one_byte_write_command!(
    OverCurrentProtectionThreshold,
    Command::OverCurrentProtectionThreshold
);
