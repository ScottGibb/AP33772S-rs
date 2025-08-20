use crate::commands::command_map::Command;
use crate::errors::Ap33772sError;
use crate::units::*;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// This command is used to read and write the Over Voltage Protection (OVP) Threshold Voltage.
/// The OVP Threshold Voltage is used to set the over voltage protection threshold for the AP33772S
/// Sink Controller. The OVP Threshold Voltage is the voltage at which the device will trigger an
/// over voltage protection event. The OVP Threshold Voltage is set as an offset from the VREQ voltage.
/// The default value is 19h (2000mV), which means the OVP Threshold Voltage is
/// [VoltageRequested](crate::commands::requested::voltage_requested::VoltageRequested) +2000mV by default.
///
/// Datasheet Name: OVPTHR
#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OverVoltageProtectionThreshold {
    #[bits(0..=7, rw)]
    /// The raw voltage value.
    raw_voltage: u8,
}

// Maximum Voltage of 30000 mV (30V) with 80mV resolution
// U8 can hold values up to 255
// 0 = 0mV
// 1 = 80mV
// 2 = 160mV
// ...
// 375 = 30000mV (30V)
// This means the maximum raw value is 375
// 375 * 80 = 30000mV U16 is required
// Therefore the voltage should be checked multiplied
impl OverVoltageProtectionThreshold {
    const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Scales the raw voltage value to millivolts.
    pub fn voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        u16::from(self.raw_voltage())
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)
            .map(|scaled_voltage| ElectricPotential::new::<millivolt>(f32::from(scaled_voltage)))
    }
    pub fn convert_voltage_to_raw_voltage(voltage: ElectricPotential) -> Result<u8, Ap33772sError> {
        if !voltage.is_finite() || !voltage.is_sign_positive() {
            return Err(Ap33772sError::ConversionFailed);
        }
        let raw_value = voltage.get::<millivolt>() / f32::from(Self::VOLTAGE_RESOLUTION);

        if raw_value > f32::from(u8::MAX) {
            return Err(Ap33772sError::ConversionFailed);
        }

        Ok(raw_value as u8)
    }
}

impl_one_byte_read_command!(
    OverVoltageProtectionThreshold,
    Command::OverVoltageProtectionThreshold
);
impl_one_byte_write_command!(
    OverVoltageProtectionThreshold,
    Command::OverVoltageProtectionThreshold
);
