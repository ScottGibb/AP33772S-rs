use super::command_map::Command;
use crate::{impl_two_byte_read_command, impl_two_byte_write_command};
use bitbybit::bitfield;
use uom::si::electrical_resistance::ohm;
use uom::si::f32::ElectricalResistance;

/// This command is used to read and write the thermal resistance value at 100 degrees Celsius.
/// It is represented as a 16-bit unsigned integer in ohms (Ω). The default value is set to 0x03CE,
/// which corresponds to 974 Ω. As for the AP33772S Sink Controller Evaluation Board, a negative temperature
/// coefficient (NTC) thermistor is used. It is stated that
/// "A 10kΩ NTC (negative temperature coefficient) thermistor is connected to the OTP pin and grounded nearby
/// the potential hot spot. The characteristic data of the NTC thermistor’s temperature and resistance values
/// need to be set by the user through I2C. Then the host MCU can read and calculate the temperature and
/// enable OTP (overtemperature protection) and de-rating functions through the I2C interface."
/// If a different thermistor is used, the value should be adjusted accordingly.
///
/// Datasheet Name: TR100
#[bitfield(u16, default = 0x03CE)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ThermalResistance100 {
    /// Raw thermal resistance value in LSB (Ω)
    ///
    /// Datasheet Name: TR100
    #[bits(0..=15, rw)]
    raw_thermal_resistance: u16,
}
impl ThermalResistance100 {
    /// Returns the thermal resistance value in ohms at 25 degrees Celsius.
    pub fn thermal_resistance(&self) -> ElectricalResistance {
        ElectricalResistance::new::<ohm>(f32::from(self.raw_thermal_resistance()))
    }
}
impl_two_byte_read_command!(ThermalResistance100, Command::ThermalResistance100);
impl_two_byte_write_command!(ThermalResistance100, Command::ThermalResistance100);
