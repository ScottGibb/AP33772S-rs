use super::command_map::Command;
use crate::error::Ap33772sError;
use crate::impl_two_byte_read_command;
use crate::types::units::*;
use bitbybit::bitfield;

/// This struct represents the requested voltage of the AP33772S device.
/// It contains the raw voltage value and provides a method to convert it to millivolts.
/// The requested voltage is the voltage that the device is set to output.
///
/// According to the datasheet:
/// "The VREQ register is defined as the latest request voltage, which depends on the PD negotiation
/// result after setting the request message (PD_REQMSG). The LSB is 50mV. The Host MCU writes PD_REQMSG
/// and will initiate a PD negotiation process. If negotiation is successful (PD_MSGRLT.RESPONSE = 1),
/// VREQ will be updated to the requested voltage (PD_REQMSG.VOLTAGE_SEL). If negotiation is unsuccessful
/// , VREQ will not be updated"
///
/// Datasheet Name: VREQ
#[bitfield(u16, default = 0x0000)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VoltageRequested {
    /// The raw voltage value.
    /// he latest requested voltage; LSB 50mV The value depends on the
    /// PD negotiation result after setting PD_REQMSG (VOLTAGE_SEL).
    ///
    /// Datasheet Name: VREQ
    #[bits(0..=15, r)]
    raw_voltage: u16,
}

// Maximum Voltage of 30V (30000 mV) with 50mV resolution
// U16 can hold values up to 65535
// 0 = 0mV
// 1 = 50mV
// 2 = 100mV
// ...
// 600 = 30000mV (30V)
// This means the maximum raw value is 600
// 600 * 50 = 30000mV
// Therefore the voltage should be checked multiplied
impl VoltageRequested {
    pub const VOLTAGE_RESOLUTION: u16 = 50; //mV
    /// Returns the voltage value in millivolts.
    pub fn voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        let scaled_voltage = self
            .raw_voltage()
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)?;
        Ok(ElectricPotential::new::<millivolt>(f32::from(
            scaled_voltage,
        )))
    }
}
impl_two_byte_read_command!(VoltageRequested, Command::VoltageRequested);
