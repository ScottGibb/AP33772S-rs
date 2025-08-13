//! This module specifically handles all Setter methods of the device and is focused on
//! setting the AP33772S in different states and modes
use super::hal::*;
use crate::ap33772s::Ap33772s;
use crate::commands::configuration::power_delivery_configuration::PowerDeliveryConfiguration;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_request_message::PowerDeliveryRequestMessage;
use crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::thermal_resistances::convert_resistance_to_raw_resistance;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thermal_resistances::thermal_resistance_50::ThermalResistance50;
use crate::commands::thermal_resistances::thermal_resistance_75::ThermalResistance75;
use crate::commands::thermal_resistances::thermal_resistance_100::ThermalResistance100;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold;
use crate::error::Ap33772sError;

// Public API Types
use crate::types::api_commands::*;
use crate::types::units::*;
use crate::types::*;

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
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
    pub async fn set_power_delivery_mode(
        &mut self,
        mode: PowerDeliveryMode,
    ) -> Result<(), Ap33772sError> {
        let command = PowerDeliveryConfiguration::builder()
            .with_extended_power_delivery_enabled(mode.extended_power_range_mode_enabled)
            .with_programmable_power_delivery_and_adjustable_power_supply_enabled(
                mode.programmable_power_supply_adjustable_voltage_supply_enabled,
            )
            .build();
        self.write_one_byte_command(command).await
    }

    #[maybe_async::maybe_async]
    #[cfg_attr(feature = "advanced", visibility::make(pub))]
    pub(crate) async fn send_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
        // If the Power Data Object is in Fixed Mode, the voltage selection is not needed.
        voltage_selection: Option<ElectricPotential>,
        current_selection: CurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<(), Ap33772sError> {
        let data_object = data_objects.get_power_data_object(power_data_object_index);

        let delivery_message = if data_object.source_power_type() == PowerType::Fixed {
            // If we are in fixed PDO Mode, the voltage selection is not needed.
            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(0)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        } else {
            let scaling_value = f32::from(data_object.voltage_resolution());
            let voltage_selection = voltage_selection.ok_or(Ap33772sError::InvalidRequest)?;
            let scaled_voltage = scaling_value * voltage_selection.get::<millivolt>();
            // Check for overflow
            let scaled_voltage = if scaled_voltage > f32::from(u8::MAX) {
                Err(Ap33772sError::ConversionFailed)
            } else {
                Ok(scaled_voltage as u8)
            }?;

            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(scaled_voltage)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        };
        self.write_two_byte_command(delivery_message).await
    }

    #[maybe_async::maybe_async]
    #[cfg_attr(feature = "advanced", visibility::make(pub))]
    /// Will Attempt to get the maximum Power Output from the Power Data Object provided
    pub(crate) async fn send_maximum_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
    ) -> Result<(), Ap33772sError> {
        // Special message outlined in the AP33772S Datasheet Page 22
        let delivery_message = PowerDeliveryRequestMessage::builder()
            .with_voltage_selection(0xFF) // No Voltage Selection in Fixed Mode
            .with_current_selection(CurrentSelection::Maximum)
            .with_power_data_object_index(power_data_object_index)
            .build();
        self.write_two_byte_command(delivery_message).await
    }
}

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    #[maybe_async::maybe_async]
    pub async fn set_thermal_resistances(
        &mut self,
        resistances: ThermalResistances,
    ) -> Result<(), Ap33772sError> {
        let resistance_25 = ThermalResistance25::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._25)?)
            .build();
        self.write_two_byte_command(resistance_25).await?;
        let resistance_50 = ThermalResistance50::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._50)?)
            .build();
        self.write_two_byte_command(resistance_50).await?;
        let resistance_75 = ThermalResistance75::builder()
            .with_raw_thermal_resistance(convert_resistance_to_raw_resistance(resistances._75)?)
            .build();
        self.write_two_byte_command(resistance_75).await?;
        let resistance_100 = ThermalResistance100::builder()
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
