use super::hal::*;
use crate::Ap33772sError;
use crate::ap33772s::{AP33772SThermalResistances, AP33772SThresholds, Ap33772s};
use crate::commands::power_delivery::power_delivery_request_message::{
    CurrentSelection, PowerDataObject, PowerDeliveryRequestMessage,
};
use crate::commands::thermal_resistances::convert_resistance_to_raw_resistance;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold;
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn send_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
        current_selection: CurrentSelection,
        voltage_selection: u8,
    ) -> Result<(), Ap33772sError> {
        let delivery_message = PowerDeliveryRequestMessage::builder()
            .with_voltage_selection(voltage_selection)
            .with_current_selection(current_selection)
            .with_power_data_object_index(power_data_object_index)
            .build();
        self.write_two_byte_command(delivery_message).await?;
        Ok(())
    }
}
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn set_thermal_resistances(
        &mut self,
        resistances: AP33772SThermalResistances,
    ) -> Result<(), Ap33772sError> {
        let resistance_25 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(
                resistances.resistance_25,
            )?)
            .build();
        self.write_two_byte_command(resistance_25).await?;
        let resistance_50 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(
                resistances.resistance_50,
            )?)
            .build();
        self.write_two_byte_command(resistance_50).await?;
        let resistance_75 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(
                resistances.resistance_75,
            )?)
            .build();
        self.write_two_byte_command(resistance_75).await?;
        let resistance_100 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(
                resistances.resistance_100,
            )?)
            .build();
        self.write_two_byte_command(resistance_100).await?;
        Ok(())
    }

    #[maybe_async::maybe_async]
    pub async fn set_thresholds(
        &mut self,
        thresholds: AP33772SThresholds,
    ) -> Result<(), Ap33772sError> {
        let over_voltage_threshold: OverVoltageProtectionThreshold =
            OverVoltageProtectionThreshold::builder()
                .with_raw_voltage(
                    OverVoltageProtectionThreshold::convert_voltage_to_raw_voltage(
                        thresholds.over_voltage_threshold,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_voltage_threshold).await?;

        let over_current_threshold: OverCurrentProtectionThreshold =
            OverCurrentProtectionThreshold::builder()
                .with_raw_current(
                    OverCurrentProtectionThreshold::convert_current_to_raw_current(
                        thresholds.over_current_threshold,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_current_threshold).await?;

        let under_voltage_threshold: UnderVoltageProtectionThreshold =
            UnderVoltageProtectionThreshold::builder()
                .with_threshold(thresholds.under_voltage_threshold)
                .build();
        self.write_one_byte_command(under_voltage_threshold).await?;

        let over_temperature_threshold: OverTemperatureProtectionThreshold =
            OverTemperatureProtectionThreshold::builder()
                .with_raw_temperature(
                    OverTemperatureProtectionThreshold::convert_temperature_to_raw_temperature(
                        thresholds.over_temperature_threshold,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_temperature_threshold)
            .await?;

        let derating_threshold: DeRatingThreshold = DeRatingThreshold::builder()
            .with_raw_temperature(DeRatingThreshold::convert_temperature_to_raw_temperature(
                thresholds.derating_threshold,
            )?)
            .build();
        self.write_one_byte_command(derating_threshold).await?;
        Ok(())
    }
}
