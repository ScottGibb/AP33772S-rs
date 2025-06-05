use crate::Ap33772sError;
use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use crate::commands::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

/// This command is used to read and write the Over Voltage Protection (OVP) Threshold Voltage.
/// The OVP Threshold Voltage is used to set the over voltage protection threshold for the AP33772S
/// Sink Controller. The OVP Threshold Voltage is the voltage at which the device will trigger an
/// over voltage protection event. The OVP Threshold Voltage is set as an offset from the VREQ voltage.
/// The default value is 19h (2000mV), which means the OVP Threshold Voltage is
/// [VoltageRequested](crate::commands::requested::voltage_requested::VoltageRequested) +2000mV by default.
#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
pub struct OverVoltageProtectionThreshold {
    #[bits(0..=7, rw)]
    /// The raw voltage value.
    raw_voltage: u8,
}

impl OverVoltageProtectionThreshold {
    const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Scales the raw voltage value to millivolts.
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() as u16 * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }

    pub fn convert_voltage_to_raw_voltage(voltage: ElectricPotential) -> Result<u8, Ap33772sError> {
        if !voltage.is_finite() || !voltage.is_sign_positive() {
            return Err(Ap33772sError::ConversionError);
        }
        let raw_value = voltage.get::<millivolt>() / Self::VOLTAGE_RESOLUTION as f32;

        if raw_value > u8::MAX as f32 {
            return Err(Ap33772sError::ConversionError);
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
