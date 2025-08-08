//! This module specifically handles all Setter methods of the device and is focused on
//! setting the AP33772S in different states and modes
use super::hal::*;
use crate::ap33772s::{Ap33772s, Ap33772sError};
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_request_message::PowerDeliveryRequestMessage;
use crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::thermal_resistances::convert_resistance_to_raw_resistance;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold;
use crate::types::units::*;
use crate::types::*;
use uom::ConversionFactor;

impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn override_output_voltage(
        &mut self,
        voltage_output: VoltageOutputControl,
    ) -> Result<(), Ap33772sError> {
        let system_control: SystemControl = SystemControl::builder()
            .with_v_out_control(voltage_output)
            .build();
        self.write_one_byte_command(system_control).await
    }
    #[maybe_async::maybe_async]
    pub async fn set_minimum_selection_voltage(
        &mut self,
        voltage: ElectricPotential,
    ) -> Result<(), Ap33772sError> {
        let raw_voltage = MinimumSelectionVoltage::convert_voltage_to_raw_voltage(voltage)?;
        let minimum_selection_voltage = MinimumSelectionVoltage::builder()
            .with_raw_voltage(raw_voltage)
            .build();
        self.write_one_byte_command(minimum_selection_voltage).await
    }
    #[maybe_async::maybe_async]
    pub async fn send_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
        voltage_selection: Option<ElectricPotential>,
        current_selection: CurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<(), Ap33772sError> {
        let power_type = data_objects.get_power_mode(&power_data_object_index);
        let scaling_value = data_objects.get_voltage_scaling(&power_data_object_index);
        let delivery_message = if power_type == PowerType::Fixed {
            // If we are in fixed PDO Mode, the voltage selection is not needed.
            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(0)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        } else {
            let voltage_selection = voltage_selection.ok_or(Ap33772sError::InvalidRequest)?;
            let scaling_value = scaling_value.ok_or(Ap33772sError::InvalidRequest)?;
            let scaled_voltage = scaling_value * voltage_selection;

            // Check for overflow
            let scaled_voltage = if scaled_voltage.value.value() > f32::from(u8::MAX) {
                Err(Ap33772sError::ConversionFailed)
            } else {
                Ok(scaled_voltage.value.value() as u8)
            }?;

            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(scaled_voltage)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        };
        self.write_two_byte_command(delivery_message).await
    }
}

impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn set_thermal_resistances(
        &mut self,
        resistances: ThermalResistances,
    ) -> Result<(), Ap33772sError> {
        let resistance_25 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._25)?)
            .build();
        self.write_two_byte_command(resistance_25).await?;
        let resistance_50 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._50)?)
            .build();
        self.write_two_byte_command(resistance_50).await?;
        let resistance_75 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._75)?)
            .build();
        self.write_two_byte_command(resistance_75).await?;
        let resistance_100 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._100)?)
            .build();
        self.write_two_byte_command(resistance_100).await
    }

    #[maybe_async::maybe_async]
    pub async fn set_thresholds(&mut self, thresholds: Thresholds) -> Result<(), Ap33772sError> {
        let over_voltage_threshold: OverVoltageProtectionThreshold =
            OverVoltageProtectionThreshold::builder()
                .with_raw_voltage(
                    OverVoltageProtectionThreshold::convert_voltage_to_raw_voltage(
                        thresholds.over_voltage,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_voltage_threshold).await?;

        let over_current_threshold: OverCurrentProtectionThreshold =
            OverCurrentProtectionThreshold::builder()
                .with_raw_current(
                    OverCurrentProtectionThreshold::convert_current_to_raw_current(
                        thresholds.over_current,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_current_threshold).await?;

        let under_voltage_threshold: UnderVoltageProtectionThreshold =
            UnderVoltageProtectionThreshold::builder()
                .with_threshold(thresholds.under_voltage)
                .build();
        self.write_one_byte_command(under_voltage_threshold).await?;

        let over_temperature_threshold: OverTemperatureProtectionThreshold =
            OverTemperatureProtectionThreshold::builder()
                .with_raw_temperature(
                    OverTemperatureProtectionThreshold::convert_temperature_to_raw_temperature(
                        thresholds.over_temperature,
                    )?,
                )
                .build();
        self.write_one_byte_command(over_temperature_threshold)
            .await?;

        let derating_threshold: DeRatingThreshold = DeRatingThreshold::builder()
            .with_raw_temperature(DeRatingThreshold::convert_temperature_to_raw_temperature(
                thresholds.derating,
            )?)
            .build();
        self.write_one_byte_command(derating_threshold).await
    }
}
