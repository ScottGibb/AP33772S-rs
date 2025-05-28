
use super::hal::*;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::Ap33772sError;
use crate::ap33772s::{AP33772SThermalResistances, Ap33772s};
use uom::si::electrical_resistance::ohm;
use uom::si::f32::ElectricalResistance;

impl<I2C: I2c> Ap33772s<I2C> {
    pub async fn set_thermal_resistances(
        &mut self,
        resistances: &AP33772SThermalResistances,
    ) -> Result<(), Ap33772sError> {
        let thermal_resistance_25_value = resistances.resistance_25.get::<ohm>();
        let thermal_resistance_25 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(value)
            .build();
     

        Ok(())
    }
}
