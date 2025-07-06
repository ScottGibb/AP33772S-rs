use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;
use bitbybit::bitfield;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

/// This struct represents the temperature of the AP33772S device.
/// It contains the raw temperature value and provides a method to convert it to degrees Celsius.
/// The temperature reported is the temperature near the NTC thermistor, which is typically located near the potential hot spot of the device.
/// The default value is set to 0x19, which corresponds to approximately 25 degrees Celsius. If the NTC Thermistor is different, the value should be adjusted accordingly.
/// See ThermalResistance100](crate::commands::thermal_resistances) for more information on the NTC thermistor used.
///
/// Datasheet Name: TEMP
#[bitfield(u8, default = 0x19)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Temperature {
    #[bits(0..=7, r)]
    raw_temperature: u8,
}
impl Temperature {
    /// Returns the temperature value in degrees Celsius.
    pub fn temperature(&self) -> ThermodynamicTemperature {
        let scaled_temperature = f32::from(self.raw_temperature());
        ThermodynamicTemperature::new::<degree_celsius>(scaled_temperature)
    }
}

impl_one_byte_read_command!(Temperature, Command::Temperature);
