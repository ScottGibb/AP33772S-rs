use bitbybit::bitfield;

use crate::{impl_two_byte_read_command, impl_two_byte_write_command};

use super::command_map::Command;
use uom::si::electrical_resistance::ohm;
use uom::si::f32::ElectricalResistance;

#[bitfield(u16, default = 0x0788)]
#[derive(Debug, PartialEq)]
pub struct ThermalResistance75 {
    /// Raw thermal resistance value in LSB (Ω)
    #[bits(0..=15, rw)]
    raw_thermal_resistance: u16,
}
impl ThermalResistance75 {
    /// Returns the thermal resistance value in ohms at 25 degrees Celsius.
    pub fn thermal_resistance(&self) -> ElectricalResistance {
        ElectricalResistance::new::<ohm>(f32::from(self.raw_thermal_resistance()))
    }
}
impl_two_byte_read_command!(ThermalResistance75, Command::ThermalResistance75);
impl_two_byte_write_command!(ThermalResistance75, Command::ThermalResistance75);
