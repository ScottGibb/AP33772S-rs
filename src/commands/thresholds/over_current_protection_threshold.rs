use bitbybit::bitfield;
use uom::si::{electric_current::milliampere, f32::ElectricCurrent};

use super::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
pub struct OverCurrentProtectionThreshold {
    #[bits(0..=7, rw)]
    raw_current: u8,
}

impl OverCurrentProtectionThreshold {
    const CURRENT_RESOLUTION: u16 = 50; // mA
    pub fn current(&self) -> ElectricCurrent {
        let scaled_current = u16::from(self.raw_current()) * Self::CURRENT_RESOLUTION;
        ElectricCurrent::new::<milliampere>(f32::from(scaled_current))
    }
    /// TODO: Look to generigy and combine into a helper function
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
