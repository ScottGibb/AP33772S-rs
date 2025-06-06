use super::hal::*;
use crate::Ap33772sError;
use crate::ap33772s;
use crate::ap33772s::AP33772SThermalResistances;
use crate::ap33772s::AP33772SThresholds;
use crate::ap33772s::Ap33772s;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thermal_resistances::thermal_resistance_50::ThermalResistance50;
use crate::commands::thermal_resistances::thermal_resistance_75::ThermalResistance75;
use crate::commands::thermal_resistances::thermal_resistance_100::ThermalResistance100;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold;

use crate::commands::requested::current_requested::CurrentRequested;
use crate::commands::requested::voltage_requested::VoltageRequested;
use crate::commands::statistics::current::Current;
use crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::statistics::temperature::Temperature;
use crate::commands::statistics::voltage::Voltage;

use uom::si::f32::ElectricCurrent;
use uom::si::f32::ElectricPotential;
use uom::si::f32::ThermodynamicTemperature;

/// This module provides methods to read various statistics from the AP33772S device.
/// It includes methods to get the current, voltage, temperature, power,
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn get_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let current = self.read_one_byte_command::<Current>().await?;
        Ok(current.current())
    }

    #[maybe_async::maybe_async]
    pub async fn get_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage = self.read_two_byte_command::<Voltage>().await?;
        Ok(voltage.voltage())
    }

    #[maybe_async::maybe_async]
    pub async fn get_temperature(&mut self) -> Result<ThermodynamicTemperature, Ap33772sError> {
        let temperature = self.read_one_byte_command::<Temperature>().await?;
        Ok(temperature.temperature())
    }

    #[maybe_async::maybe_async]
    pub async fn get_power(mut self) -> Result<uom::si::f32::Power, Ap33772sError> {
        let current = self.get_current().await?;
        let voltage = self.get_voltage().await?;
        let power = current * voltage;
        Ok(power)
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let requested_voltage = self.read_two_byte_command::<VoltageRequested>().await?;
        Ok(requested_voltage.voltage())
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let requested_current = self.read_two_byte_command::<CurrentRequested>().await?;
        Ok(requested_current.current())
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_power(&mut self) -> Result<uom::si::f32::Power, Ap33772sError> {
        let requested_voltage = self.get_requested_voltage().await?;
        let requested_current = self.get_requested_current().await?;
        let requested_power = requested_voltage * requested_current;
        Ok(requested_power)
    }
    #[maybe_async::maybe_async]
    pub async fn get_statistics(&mut self) -> Result<ap33772s::AP33772SStatistics, Ap33772sError> {
        let current = self.get_current().await?;
        let voltage = self.get_voltage().await?;
        let temperature = self.get_temperature().await?;
        let requested_voltage = self.get_requested_voltage().await?;
        let requested_current = self.get_requested_current().await?;
        Ok(ap33772s::AP33772SStatistics {
            current,
            voltage,
            temperature,
            power: current * voltage,
            requested_voltage,
            requested_current,
            requested_power: requested_voltage * requested_current,
        })
    }
    #[maybe_async::maybe_async]
    pub async fn get_minimum_selection_voltage(
        &mut self,
    ) -> Result<ElectricPotential, Ap33772sError> {
        let voltage_selection = self
            .read_one_byte_command::<MinimumSelectionVoltage>()
            .await?;
        Ok(voltage_selection.voltage())
    }
}

impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn get_thermal_resistances(
        &mut self,
    ) -> Result<AP33772SThermalResistances, Ap33772sError> {
        let resistance_25 = self.read_two_byte_command::<ThermalResistance25>().await?;
        let resistance_50 = self.read_two_byte_command::<ThermalResistance50>().await?;
        let resistance_75 = self.read_two_byte_command::<ThermalResistance75>().await?;
        let resistance_100 = self.read_two_byte_command::<ThermalResistance100>().await?;

        Ok(AP33772SThermalResistances {
            resistance_25: resistance_25.thermal_resistance(),
            resistance_50: resistance_50.thermal_resistance(),
            resistance_75: resistance_75.thermal_resistance(),
            resistance_100: resistance_100.thermal_resistance(),
        })
    }
    #[maybe_async::maybe_async]
    pub async fn get_thresholds(&mut self) -> Result<AP33772SThresholds, Ap33772sError> {
        let over_voltage_threshold = self
            .read_one_byte_command::<OverVoltageProtectionThreshold>()
            .await?;
        let over_current_threshold = self
            .read_one_byte_command::<OverCurrentProtectionThreshold>()
            .await?;
        let over_temperature_protection_threshold = self
            .read_one_byte_command::<over_temperature_protection_threshold::OverTemperatureProtectionThreshold>()
            .await
           ?;
        let under_voltage_threshold = self
            .read_one_byte_command::<UnderVoltageProtectionThreshold>()
            .await?;
        let under_voltage_threshold = under_voltage_threshold.threshold().unwrap();
        let de_rating_threshold = self.read_one_byte_command::<DeRatingThreshold>().await?;
        Ok(AP33772SThresholds {
            over_voltage_threshold: over_voltage_threshold.voltage(),
            over_current_threshold: over_current_threshold.current(),
            over_temperature_threshold: over_temperature_protection_threshold.temperature(),
            under_voltage_threshold: under_voltage_threshold,
            derating_threshold: de_rating_threshold.temperature(),
        })
    }
}
