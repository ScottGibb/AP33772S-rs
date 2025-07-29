use super::hal::*;
use crate::Ap33772sError;
use crate::ap33772s::Ap33772s;
use crate::ap33772s::Statistics;
use crate::ap33772s::ThermalResistances;
use crate::ap33772s::Thresholds;
use crate::commands::command_map::Command;
use crate::commands::configuration::status::Status;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::configuration::system_control::VoltageOutputControl;
use crate::commands::data_objects::all_source_power_data_object::AllSourceDataPowerDataObject;
use crate::commands::data_objects::all_source_power_data_object::MAX_EXTENDED_POWER_DATA_OBJECTS;
use crate::commands::data_objects::all_source_power_data_object::MAX_SOURCE_POWER_DATA_OBJECTS;
use crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject;
use crate::commands::data_objects::source_power_data_object::SourcePowerDataObject;
use crate::commands::power_delivery::power_delivery_message_result::PowerDeliveryMessageResult;
use crate::commands::power_delivery::power_delivery_message_result::PowerDeliveryResponse;
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

impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn get_status(&mut self) -> Result<Status, Ap33772sError> {
        let status = self.read_one_byte_command::<Status>().await?;
        Ok(status)
    }
}
/// This module provides methods to read various statistics from the AP33772S device.
/// It includes methods to get the current, voltage, temperature, power,
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn get_voltage_out_override(
        &mut self,
    ) -> Result<VoltageOutputControl, Ap33772sError> {
        let system_control = self.read_one_byte_command::<SystemControl>().await?;
        let v_out_control = system_control
            .v_out_control()
            .map_err(|_| Ap33772sError::DataMalformed)?;
        Ok(v_out_control)
    }

    #[maybe_async::maybe_async]
    pub async fn get_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let current = self.read_one_byte_command::<Current>().await?;
        current.current()
    }

    #[maybe_async::maybe_async]
    pub async fn get_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage = self.read_two_byte_command::<Voltage>().await?;
        voltage.voltage()
    }

    #[maybe_async::maybe_async]
    pub async fn get_temperature(&mut self) -> Result<ThermodynamicTemperature, Ap33772sError> {
        let temperature = self.read_one_byte_command::<Temperature>().await?;
        Ok(temperature.temperature())
    }

    #[maybe_async::maybe_async]
    pub async fn get_power(&mut self) -> Result<uom::si::f32::Power, Ap33772sError> {
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

impl<I2C: I2c> Ap33772s<I2C> {
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
    #[maybe_async::maybe_async]
    pub async fn get_thresholds(&mut self) -> Result<Thresholds, Ap33772sError> {
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
        let under_voltage_threshold = under_voltage_threshold
            .threshold()
            .or(Err(Ap33772sError::DataMalformed))?;
        let de_rating_threshold = self.read_one_byte_command::<DeRatingThreshold>().await?;
        Ok(Thresholds {
            over_voltage: over_voltage_threshold.voltage()?,
            over_current: over_current_threshold.current()?,
            over_temperature: over_temperature_protection_threshold.temperature(),
            under_voltage: under_voltage_threshold,
            derating: de_rating_threshold.temperature(),
        })
    }

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
                &[u8::from(Command::AllSourcesPowerDataObject)],
                &mut buff,
            )
            .await?;
        let mut data_object = AllSourceDataPowerDataObject::default();

        for i in 0..MAX_SOURCE_POWER_DATA_OBJECTS {
            data_object.source_power[i] =
                SourcePowerDataObject::new_with_raw_value(u16::from_le_bytes([
                    buff[2 * i],
                    buff[2 * i + 1],
                ]));
        }
        for i in 0..MAX_EXTENDED_POWER_DATA_OBJECTS {
            data_object.extended_power[i] =
                ExtendedPowerRangeDataObject::new_with_raw_value(u16::from_le_bytes([
                    buff[2 * (i + MAX_SOURCE_POWER_DATA_OBJECTS)],
                    buff[2 * (i + MAX_SOURCE_POWER_DATA_OBJECTS) + 1],
                ]));
        }

        Ok(data_object)
    }

    #[maybe_async::maybe_async]
    pub async fn get_power_delivery_request_result(
        &mut self,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        let power_delivery_request_result = self
            .read_one_byte_command::<PowerDeliveryMessageResult>()
            .await?;

        power_delivery_request_result
            .response()
            .or(Err(Ap33772sError::DataMalformed))
    }
}
