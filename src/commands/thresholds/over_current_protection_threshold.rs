use bitbybit::bitfield;
use uom::si::{electric_current::milliampere, f32::ElectricCurrent};

use super::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

/// The OCPTHR register is defined as the OCP Threshold Current that triggers OCP protection function.
/// The OCP Threshold Current is 110% of the OCPTHR current value. The default value for the OCPTHR is
/// 00h and the LSB is 50mA.
///
/// If the OCPTHR is set to 0, the OCP Threshold Current will be updated to 110%
/// of the selected [Power Data Object](crate::commands::data_objects::source_power_data_object::SourcePowerDataObject)
/// (POD) maximum current after successful negotiation with the PD source.
/// Please refer to the “Overcurrent Protection” section for more details
///
/// // Datasheet Name: OCPTHR
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct OverCurrentProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_current: u8,
}

impl OverCurrentProtectionThreshold {
    const CURRENT_RESOLUTION: u16 = 50; // mA

    /// Returns the current value in milliampere.
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = u16::from(self.raw_current()) * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::from(scaled_current))
    }
    /// TODO: Look to generigy and combine into a helper function
    // TODO: Consider Better Error Handling of the different conversion failures
    pub fn convert_current_to_raw_current(
        current: ElectricCurrent,
    ) -> Result<u8, crate::Ap33772sError> {
        if !current.is_finite() || !current.is_sign_positive() {
            return Err(crate::Ap33772sError::ConversionError);
        }
        let raw_value = current.get::<milliampere>() / Self::CURRENT_RESOLUTION as f32;

        if raw_value > u8::MAX as f32 {
            return Err(crate::Ap33772sError::ConversionError);
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
