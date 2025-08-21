//! This module provides methods to read various statistics from the AP33772S device.
//! It includes methods to get the current, voltage, temperature, power,
use super::hal::*;
use crate::ap33772s::Ap33772s;
use crate::commands::command_map::Command;
use crate::commands::configuration::operation_mode::OperationMode;
use crate::commands::configuration::power_delivery_configuration::PowerDeliveryConfiguration;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::data_objects::all_source_power_data_object::AllSourceDataPowerDataObject;
use crate::commands::data_objects::all_source_power_data_object::MAX_EXTENDED_POWER_DATA_OBJECTS;
use crate::commands::data_objects::all_source_power_data_object::MAX_STANDARD_POWER_DATA_OBJECTS;
use crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject;
use crate::commands::data_objects::source_power_range_data_object::SourcePowerRangeDataObject;
use crate::commands::data_objects::standard_power_range_data_object::StandardPowerRangeDataObject;
use crate::commands::power_delivery::power_delivery_message_result::PowerDeliveryMessageResult;
use crate::commands::requested::current_requested::CurrentRequested;
use crate::commands::requested::voltage_requested::VoltageRequested;
use crate::commands::statistics::current::Current;
use crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::statistics::temperature::Temperature;
use crate::commands::statistics::voltage::Voltage;
use crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25;
use crate::commands::thermal_resistances::thermal_resistance_50::ThermalResistance50;
use crate::commands::thermal_resistances::thermal_resistance_75::ThermalResistance75;
use crate::commands::thermal_resistances::thermal_resistance_100::ThermalResistance100;
use crate::commands::thresholds::de_rating_threshold::DeRatingThreshold;
use crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold;
use crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold;
use crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold;
use crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold;

// Public API Types
use crate::types::command_structures::*;
use crate::types::*;
use crate::units::*;

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    /// Reads the current device status register.
    ///
    /// Returns detailed information about the device state including error flags,
    /// communication status, and power delivery state.
    ///
    /// # Returns
    ///
    /// [`Status`] containing device status flags, or [`Ap33772sError`] on communication error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = device.get_status()?;
    ///
    /// if status.i2c_ready() {
    ///     println!("Device is ready for communication");
    /// }
    /// if status.started() {
    ///     println!("Device has completed initialization");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Status`]: crate::types::command_structures::Status
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_status(&mut self) -> Result<Status, Ap33772sError> {
        self.read_one_byte_command::<Status>().await
    }

    /// Reads the current operation mode of the device.
    ///
    /// Returns information about the device's current operational configuration,
    /// including channel settings and de-rating mode.
    ///
    /// # Returns
    ///
    /// [`OperationMode`] containing the current operation settings, or [`Ap33772sError`] on communication error.
    ///
    /// [`OperationMode`]: crate::types::command_structures::OperationMode
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_operating_mode(&mut self) -> Result<OperationMode, Ap33772sError> {
        self.read_one_byte_command::<OperationMode>().await
    }

    /// Reads the power delivery configuration capabilities.
    ///
    /// Returns information about supported power delivery modes including
    /// Programmable Power Supply (PPS) and Extended Power Range (EPR) support.
    ///
    /// # Returns
    ///
    /// [`PowerDeliveryMode`] indicating supported PD features, or [`Ap33772sError`] on communication error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let pd_config = device.get_power_delivery_configuration()?;
    ///
    /// if pd_config.programmable_power_supply_adjustable_voltage_supply_enabled {
    ///     println!("PPS with AVS is supported");
    /// }
    /// if pd_config.extended_power_range_mode_enabled {
    ///     println!("Extended Power Range mode is supported");  
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`PowerDeliveryMode`]: crate::types::PowerDeliveryMode
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_power_delivery_configuration(
        &mut self,
    ) -> Result<PowerDeliveryMode, Ap33772sError> {
        let command_result = self
            .read_one_byte_command::<PowerDeliveryConfiguration>()
            .await?;
        Ok(PowerDeliveryMode {
            programmable_power_supply_adjustable_voltage_supply_enabled: command_result
                .programmable_power_supply_and_adjustable_power_supply_enabled(),
            extended_power_range_mode_enabled: command_result.extended_power_delivery_enabled(),
        })
    }

    /// Retrieves comprehensive device statistics and measurements.
    ///
    /// This is a convenience method that collects all major device measurements
    /// into a single [`Statistics`] struct. It reads current, voltage, temperature,
    /// and requested values from the device.
    ///
    /// # Returns
    ///
    /// [`Statistics`] containing all current measurements and requested values,
    /// or [`Ap33772sError`] on communication error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ap33772s_rs::Ap33772s;
    /// # use ap33772s_rs::units::*;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let stats = device.get_statistics()?;
    ///
    /// println!("Current operation: {:.2}V @ {:.2}A = {:.2}W",
    ///          stats.voltage.get::<volt>(), stats.current.get::<ampere>(), stats.power.get::<watt>());
    /// println!("Device temperature: {:.1}°C", stats.temperature.get::<degree_celsius>());
    /// println!("Requested: {:.2}V @ {:.2}A",
    ///          stats.requested_voltage.get::<volt>(), stats.requested_current.get::<ampere>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// **Note**: This method performs multiple I2C reads. For better performance,
    /// use individual getter methods if you only need specific measurements.
    ///
    /// [`Statistics`]: crate::types::Statistics
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_statistics(&mut self) -> Result<Statistics, Ap33772sError> {
        let current = self.get_current().await?;
        let voltage = self.get_voltage().await?;
        let temperature = self.get_temperature().await?;
        let requested_voltage = self.get_requested_voltage().await?;
        let requested_current = self.get_requested_current().await?;
        Ok(Statistics {
            current,
            voltage,
            temperature,
            power: current * voltage,
            requested_voltage,
            requested_current,
            requested_power: requested_voltage * requested_current,
        })
    }

    /// This function is feature gated by the `advanced` feature. This can be called after calling
    /// [send_power_delivery_request](crate::setters::Ap33772s::send_power_delivery_request). It is recommended
    /// to give the system time to respond before calling this function otherwise the response may be `Busy`.
    #[maybe_async::maybe_async]
    #[cfg_attr(feature = "advanced", visibility::make(pub))]
    pub(crate) async fn get_power_delivery_request_result(
        &mut self,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        let power_delivery_request_result = self
            .read_one_byte_command::<PowerDeliveryMessageResult>()
            .await?;

        power_delivery_request_result
            .response()
            .map_err(Ap33772sError::DataMalformed)
    }
}

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    /// Reads the current voltage output override setting.
    ///
    /// Returns the current state of the voltage output control, indicating
    /// whether the output is enabled, disabled, or in auto mode.
    ///
    /// # Returns
    ///
    /// [`VoltageOutputControl`] indicating the current output state, or [`Ap33772sError`] on communication error.
    ///
    /// [`VoltageOutputControl`]: crate::types::command_structures::VoltageOutputControl
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_voltage_out_override(
        &mut self,
    ) -> Result<VoltageOutputControl, Ap33772sError> {
        let system_control = self.read_one_byte_command::<SystemControl>().await?;
        system_control
            .v_out_control()
            .map_err(Ap33772sError::DataMalformed)
    }

    /// Reads the current flowing through the device.
    ///
    /// # Returns
    ///
    /// [`ElectricCurrent`] measurement, or [`Ap33772sError`] on communication error.
    ///
    /// [`ElectricCurrent`]: crate::units::ElectricCurrent
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let current = self.read_one_byte_command::<Current>().await?;
        current.current()
    }

    /// Reads the output voltage of the device.
    ///
    /// # Returns
    ///
    /// [`ElectricPotential`] measurement, or [`Ap33772sError`] on communication error.
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage = self.read_two_byte_command::<Voltage>().await?;
        voltage.voltage()
    }

    /// Reads the internal temperature of the device.
    ///
    /// # Returns
    ///
    /// [`ThermodynamicTemperature`] measurement, or [`Ap33772sError`] on communication error.
    ///
    /// [`ThermodynamicTemperature`]: crate::units::ThermodynamicTemperature
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_temperature(&mut self) -> Result<ThermodynamicTemperature, Ap33772sError> {
        let temperature = self.read_one_byte_command::<Temperature>().await?;
        Ok(temperature.temperature())
    }

    /// Calculates the current power consumption.
    ///
    /// This method reads both current and voltage, then calculates power as P = I × V.
    ///
    /// # Returns
    ///
    /// [`Power`] calculation (current × voltage), or [`Ap33772sError`] on communication error.
    ///
    /// **Note**: This method performs two I2C reads. For better performance when reading
    /// multiple values, consider using [`get_statistics`] instead.
    ///
    /// [`Power`]: crate::units::Power
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_power(&mut self) -> Result<Power, Ap33772sError> {
        let current = self.get_current().await?;
        let voltage = self.get_voltage().await?;
        let power = current * voltage;
        Ok(power)
    }
    /// Reads the voltage requested by the connected device during power delivery negotiation.
    ///
    /// # Returns
    ///
    /// [`ElectricPotential`] requested by the device, or [`Ap33772sError`] on communication error.
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_requested_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let requested_voltage = self.read_two_byte_command::<VoltageRequested>().await?;
        requested_voltage.voltage()
    }
    /// Reads the current requested by the connected device during power delivery negotiation.
    ///
    /// # Returns
    ///
    /// [`ElectricCurrent`] requested by the device, or [`Ap33772sError`] on communication error.
    ///
    /// [`ElectricCurrent`]: crate::units::ElectricCurrent
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_requested_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let requested_current = self.read_two_byte_command::<CurrentRequested>().await?;
        requested_current.current()
    }
    /// Calculates the power requested by the connected device during power delivery negotiation.
    ///
    /// This method reads both requested voltage and current, then calculates power as P = I × V.
    ///
    /// # Returns
    ///
    /// [`Power`] calculation (requested_current × requested_voltage), or [`Ap33772sError`] on communication error.
    ///
    /// **Note**: This method performs two I2C reads. For better performance when reading
    /// multiple values, consider using [`get_statistics`] instead.
    ///
    /// [`Power`]: crate::units::Power
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_requested_power(&mut self) -> Result<Power, Ap33772sError> {
        let requested_voltage = self.get_requested_voltage().await?;
        let requested_current = self.get_requested_current().await?;
        let requested_power = requested_voltage * requested_current;
        Ok(requested_power)
    }

    /// Reads the minimum selection voltage setting.
    ///
    /// This voltage represents the lowest voltage that the device will negotiate
    /// during power delivery.
    ///
    /// # Returns
    ///
    /// [`ElectricPotential`] minimum voltage setting, or [`Ap33772sError`] on communication error.
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_minimum_selection_voltage(
        &mut self,
    ) -> Result<ElectricPotential, Ap33772sError> {
        let voltage_selection = self
            .read_one_byte_command::<MinimumSelectionVoltage>()
            .await?;
        voltage_selection.voltage()
    }
}

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    /// Reads the current thermal resistance configuration for the NTC thermistor.
    ///
    /// Returns resistance values at different temperature points used for thermal
    /// protection and temperature monitoring.
    ///
    /// # Returns
    ///
    /// [`ThermalResistances`] containing resistance values at 25°C, 50°C, 75°C, and 100°C,
    /// or [`Ap33772sError`] on communication error.
    ///
    /// **Note**: This method performs four I2C reads.
    ///
    /// [`ThermalResistances`]: crate::types::ThermalResistances
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_thermal_resistances(&mut self) -> Result<ThermalResistances, Ap33772sError> {
        let resistance_25 = self.read_two_byte_command::<ThermalResistance25>().await?;
        let resistance_50 = self.read_two_byte_command::<ThermalResistance50>().await?;
        let resistance_75 = self.read_two_byte_command::<ThermalResistance75>().await?;
        let resistance_100 = self.read_two_byte_command::<ThermalResistance100>().await?;

        Ok(ThermalResistances {
            _25: resistance_25.thermal_resistance(),
            _50: resistance_50.thermal_resistance(),
            _75: resistance_75.thermal_resistance(),
            _100: resistance_100.thermal_resistance(),
        })
    }
    /// Reads all protection threshold settings from the device.
    ///
    /// Returns comprehensive threshold configuration including over-voltage,
    /// over-current, over-temperature, under-voltage, and de-rating thresholds.
    ///
    /// # Returns
    ///
    /// [`Thresholds`] containing all protection threshold values, or [`Ap33772sError`] on communication error.
    ///
    /// **Note**: This method performs five I2C reads.
    ///
    /// [`Thresholds`]: crate::types::Thresholds
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_thresholds(&mut self) -> Result<Thresholds, Ap33772sError> {
        let over_voltage_threshold = self
            .read_one_byte_command::<OverVoltageProtectionThreshold>()
            .await?;
        let over_current_threshold = self
            .read_one_byte_command::<OverCurrentProtectionThreshold>()
            .await?;
        let over_temperature_protection_threshold = self
            .read_one_byte_command::<OverTemperatureProtectionThreshold>()
            .await?;
        let under_voltage_threshold = self
            .read_one_byte_command::<UnderVoltageProtectionThreshold>()
            .await?;
        let under_voltage_threshold = under_voltage_threshold
            .threshold()
            .map_err(Ap33772sError::DataMalformed)?;
        let de_rating_threshold = self.read_one_byte_command::<DeRatingThreshold>().await?;
        Ok(Thresholds {
            over_voltage: over_voltage_threshold.voltage()?,
            over_current: over_current_threshold.current()?,
            over_temperature: over_temperature_protection_threshold.temperature(),
            under_voltage: under_voltage_threshold,
            derating: de_rating_threshold.temperature(),
        })
    }

    /// Reads all available power source capabilities from the connected USB-C device.
    ///
    /// This method retrieves the complete list of Power Data Objects (PDOs) that the
    /// connected device supports, including standard and extended power ranges.
    ///
    /// # Returns
    ///
    /// [`AllSourceDataPowerDataObject`] containing all supported power capabilities,
    /// or [`Ap33772sError`] on communication error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let capabilities = device.get_all_source_power_capabilities()?;
    ///
    /// println!("Available power capabilities:");
    /// for (i, pdo) in capabilities.power_data_objects.iter().enumerate() {
    ///     println!("  PDO {}: {:?}", i + 1, pdo);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AllSourceDataPowerDataObject`]: crate::types::command_structures::AllSourceDataPowerDataObject
    /// [`Ap33772sError`]: crate::errors::Ap33772sError
    #[maybe_async::maybe_async]
    pub async fn get_all_source_power_capabilities(
        &mut self,
    ) -> Result<AllSourceDataPowerDataObject, Ap33772sError> {
        const NUM_SOURCE_DATA_POWER_OBJECT_BYTES: usize = 26;
        let mut buff: [u8; NUM_SOURCE_DATA_POWER_OBJECT_BYTES] =
            [0; NUM_SOURCE_DATA_POWER_OBJECT_BYTES];
        self.i2c
            .write_read(
                Self::ADDRESS,
                &[Command::AllSourcesPowerDataObject as u8],
                &mut buff,
            )
            .await?;
        let mut data_object = AllSourceDataPowerDataObject::default();

        for i in 0..MAX_STANDARD_POWER_DATA_OBJECTS {
            data_object.power_data_objects[i] = SourcePowerRangeDataObject::Standard(
                StandardPowerRangeDataObject::new_with_raw_value(u16::from_le_bytes([
                    buff[2 * i],
                    buff[2 * i + 1],
                ])),
            );
        }
        for i in MAX_STANDARD_POWER_DATA_OBJECTS
            ..MAX_EXTENDED_POWER_DATA_OBJECTS + MAX_STANDARD_POWER_DATA_OBJECTS
        {
            data_object.power_data_objects[i] = SourcePowerRangeDataObject::Extended(
                ExtendedPowerRangeDataObject::new_with_raw_value(u16::from_le_bytes([
                    buff[2 * (i)],
                    buff[2 * (i) + 1],
                ])),
            );
        }

        Ok(data_object)
    }
}
