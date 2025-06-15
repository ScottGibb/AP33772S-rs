use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use crate::commands::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

/// The MinimumSelectionVoltage command retrieves the minimum selection voltage
/// of the AP33772S. This voltage is used to determine the minimum voltage that can be selected
/// by the device for operation. The value is represented in raw format, which can be converted
/// to millivolts using the `voltage` method.
///
/// The datasheet states:
/// "The VSELMIN register is defined as the Minimum Selection Voltage. If the VREQ voltage is more
/// than or equal to the VSELMIN voltage, the VOUT MOS switches turn ON after the system is ready
///  (STATUS.READY = 1). The default value for VSELMIN is 19h (5000mV) and the LSB is 200mV"
///
/// /// Datasheet Name: VSELMIN
#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
pub struct MinimumSelectionVoltage {
    /// The raw voltage value representing the minimum selection voltage. The `raw_voltage` is represented with
    /// LSB as 200mV
    /// If VVREQ ≥ VVSELMIN, VOUT MOS switches turn on after system is ready (READY=1)
    #[bits(0..=7, rw)]
    raw_voltage: u8,
}

impl MinimumSelectionVoltage {
    pub const SELECTION_VOLTAGE_RESOLUTION: u16 = 200; // mV
    /// Returns the minimum selection voltage in millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = u16::from(self.raw_voltage()) * Self::SELECTION_VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }
    pub fn convert_voltage_to_raw_voltage(
        voltage: ElectricPotential,
    ) -> Result<u8, crate::Ap33772sError> {
        if !voltage.is_finite() || !voltage.is_sign_positive() {
            return Err(crate::Ap33772sError::ConversionError);
        }
        let raw_value =
            voltage.get::<millivolt>() / (Self::SELECTION_VOLTAGE_RESOLUTION as u8) as f32;

        if raw_value > u8::MAX as f32 {
            return Err(crate::Ap33772sError::ConversionError);
        }

        Ok(raw_value as u8)
    }
}

impl_one_byte_read_command!(MinimumSelectionVoltage, Command::MinimumSelectionVoltage);
impl_one_byte_write_command!(MinimumSelectionVoltage, Command::MinimumSelectionVoltage);
