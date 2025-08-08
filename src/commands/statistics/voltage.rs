use crate::commands::command_map::Command;
use crate::error::Ap33772sError;
use crate::impl_two_byte_read_command;
use crate::types::units::*;
use bitbybit::bitfield;

/// This struct represents the voltage of the AP33772S device.
/// It contains the raw voltage value and provides a method to convert it to millivolts.
///
/// Datasheet Name: VOLTAGE
#[bitfield(u16, default = 0x0)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Voltage {
    #[bits(0..=15, r)]
    /// The raw voltage value.
    ///
    /// Datasheet Name: VOLTAGE
    raw_voltage: u16,
}

// Maximum Voltage of 30000 mV (30V) with 80mV resolution
// U16 can hold values up to 65535
// 0 = 0mV
// 1 = 80mV
// 2 = 160mV
// ...
// 375 = 30000mV (30V)
// This means the maximum raw value is 375
// 375 * 80 = 30000mV
// Therefore the voltage should be checked multiplied
impl Voltage {
    pub const VOLTAGE_RESOLUTION: u16 = 80; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        self.raw_voltage()
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .map(|scaled_voltage| ElectricPotential::new::<millivolt>(f32::from(scaled_voltage)))
            .ok_or(Ap33772sError::ConversionFailed)
    }
}

impl_two_byte_read_command!(Voltage, Command::Voltage);
