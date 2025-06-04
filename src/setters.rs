use super::hal::*;
use crate::Ap33772sError;
use crate::ap33772s::{AP33772SThermalResistances, AP33772SThresholds, Ap33772s};
use crate::commands::thermal_resistances::convert_resistance_to_raw_resistance;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;

impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn set_thermal_resistances(
        &mut self,
        resistances: AP33772SThermalResistances,
    ) -> Result<(), Ap33772sError> {
        let resistance_25 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(
                convert_resistance_to_raw_resistance(resistances.resistance_25)?,
            )
            .build();
        self.write_two_byte_command(resistance_25).await?;
        let resistance_50 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(
                convert_resistance_to_raw_resistance(resistances.resistance_50)?,
            )
            .build();
        self.write_two_byte_command(resistance_50).await?;
        let resistance_75 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(
                convert_resistance_to_raw_resistance(resistances.resistance_75)?,
            )
            .build();
        self.write_two_byte_command(resistance_75).await?;
        let resistance_100 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(
                convert_resistance_to_raw_resistance(resistances.resistance_100)?,
            )
            .build();
        self.write_two_byte_command(resistance_100).await?;
        Ok(())
    }

    #[maybe_async::maybe_async]
    pub async fn set_thresholds(
        &mut self,
        thresholds: AP33772SThresholds,
    ) -> Result<(), Ap33772sError> {
         //TODO: Finish this function
        Ok(())
    }
}
