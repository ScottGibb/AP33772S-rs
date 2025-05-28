use bitbybit::bitfield;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricPotential;

use super::command_map::Command;
use crate::{impl_two_byte_read_command};

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
pub struct VoltageRequested {
    /// The raw voltage value.
    /// he latest requested voltage; LSB 50mV The value depends on the 
    /// PD negotiation result after setting PD_REQMSG (VOLTAGE_SEL).
    /// 
    /// Datasheet Name: VREQ
    #[bits(0..=15, r)]
    raw_voltage: u16,
}

impl VoltageRequested {
    pub const VOLTAGE_RESOLUTION: u16 = 50; //mV
    /// Returns the voltage value in millivolts.
    /// 
    pub fn voltage(&self) -> ElectricPotential {
        let scaled_voltage = self.raw_voltage() * Self::VOLTAGE_RESOLUTION;
        ElectricPotential::new::<millivolt>(f32::from(scaled_voltage))
    }
}
impl_two_byte_read_command!(VoltageRequested, Command::VoltageRequested);
