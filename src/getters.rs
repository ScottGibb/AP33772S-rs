//! # Getter Methods for AP33772S Device
//!
//! This module provides comprehensive methods to read various parameters and statistics from
//! the AP33772S USB-C Power Delivery controller. These functions allow you to monitor device
//! state, power delivery configuration, electrical measurements, and protection status.
//!
//! ## Categories of Getters
//!
//! ### Device Status and Configuration
//! - [`get_status`] - Overall device status including protection flags
//! - [`get_operating_mode`] - Current operation mode (PD vs Legacy, CC configuration)  
//! - [`get_power_delivery_configuration`] - PD mode settings (EPR, PPS/AVS enabled)
//!
//! ### Electrical Measurements  
//! - [`get_statistics`] - Complete electrical state (V, I, P, T)
//! - [`get_current`] - Output current measurement
//! - [`get_voltage`] - Output voltage measurement  
//! - [`get_temperature`] - Junction temperature
//! - [`get_power`] - Calculated output power
//!
//! ### Power Delivery Information
//! - [`get_requested_voltage`] - Voltage requested from source
//! - [`get_requested_current`] - Current requested from source
//! - [`get_requested_power`] - Power requested from source  
//! - [`get_all_source_power_capabilities`] - Available PDOs from source
//! - [`get_minimum_selection_voltage`] - Minimum voltage threshold
//!
//! ### Protection and Thermal Settings
//! - [`get_thermal_resistances`] - Thermal resistance values at different temperatures
//! - [`get_thresholds`] - Protection threshold values (OVP, OCP, OTP, UVP, derating)
//! - [`get_voltage_out_override`] - Output voltage switch control state
//!
//! ## Usage Examples
//!
//! ```rust,no_run
//! # use ap33772s_rs::{Ap33772s, units::*};
//! # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
//! // Get complete device statistics
//! let stats = device.get_statistics()?;
//! println!("Power: {:.1}W @ {:.1}V, {:.2}A",
//!          stats.power.get::<watt>(),
//!          stats.voltage.get::<volt>(),
//!          stats.current.get::<ampere>());
//!
//! // Check device status for any protection events
//! let status = device.get_status()?;
//! if status.over_voltage_protection() {
//!     println!("Warning: Over voltage protection active!");
//! }
//!
//! // Get available power capabilities from source  
//! let capabilities = device.get_all_source_power_capabilities()?;
//! # Ok(())
//! # }
//! ```
//!
//! [`get_status`]: crate::Ap33772s::get_status
//! [`get_operating_mode`]: crate::Ap33772s::get_operating_mode
//! [`get_power_delivery_configuration`]: crate::Ap33772s::get_power_delivery_configuration  
//! [`get_statistics`]: crate::Ap33772s::get_statistics
//! [`get_current`]: crate::Ap33772s::get_current
//! [`get_voltage`]: crate::Ap33772s::get_voltage
//! [`get_temperature`]: crate::Ap33772s::get_temperature
//! [`get_power`]: crate::Ap33772s::get_power
//! [`get_requested_voltage`]: crate::Ap33772s::get_requested_voltage
//! [`get_requested_current`]: crate::Ap33772s::get_requested_current
//! [`get_requested_power`]: crate::Ap33772s::get_requested_power
//! [`get_all_source_power_capabilities`]: crate::Ap33772s::get_all_source_power_capabilities
//! [`get_minimum_selection_voltage`]: crate::Ap33772s::get_minimum_selection_voltage
//! [`get_thermal_resistances`]: crate::Ap33772s::get_thermal_resistances
//! [`get_thresholds`]: crate::Ap33772s::get_thresholds
//! [`get_voltage_out_override`]: crate::Ap33772s::get_voltage_out_override
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
    /// The status register provides information about the device's operational state,
    /// including startup status, I2C readiness, and protection event flags. This register
    /// is automatically cleared after being read.
    ///
    /// # Returns
    ///
    /// Returns a [`Status`] struct containing:
    /// - System startup status
    /// - I2C communication readiness  
    /// - New Power Data Object availability
    /// - Protection flags (UVP, OVP, OCP, OTP)
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = device.get_status()?;
    ///
    /// if status.started() {
    ///     println!("Device has started successfully");
    /// }
    ///
    /// if status.over_voltage_protection() {
    ///     println!("Over voltage protection triggered!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_operating_mode`] for operational mode information
    /// - [`get_thresholds`] for protection threshold values
    ///
    /// [`Status`]: crate::types::command_structures::Status
    /// [`get_operating_mode`]: Self::get_operating_mode
    /// [`get_thresholds`]: Self::get_thresholds
    #[maybe_async::maybe_async]
    pub async fn get_status(&mut self) -> Result<Status, Ap33772sError> {
        self.read_one_byte_command::<Status>().await
    }

    /// Reads the current operation mode of the device.
    ///
    /// The operation mode indicates how the device is currently configured and what
    /// type of source is connected (Power Delivery vs Legacy), which CC line is active,
    /// and whether derating mode is enabled.
    ///
    /// # Returns
    ///
    /// Returns an [`OperationMode`] struct containing:
    /// - Legacy source connection status
    /// - Power Delivery source connection status  
    /// - Active configuration channel (CC1 or CC2)
    /// - Derating mode status
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let mode = device.get_operating_mode()?;
    ///
    /// if mode.power_delivery_source_connected() {
    ///     println!("USB-C PD source detected on CC{}",
    ///              if mode.configuration_channel() == ConfigurationChannel::One { 1 } else { 2 });
    /// } else if mode.legacy_source_connected() {
    ///     println!("Legacy USB source detected");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_status`] for device status information
    /// - [`get_power_delivery_configuration`] for PD-specific settings
    ///
    /// [`OperationMode`]: crate::types::command_structures::OperationMode
    /// [`get_status`]: Self::get_status
    /// [`get_power_delivery_configuration`]: Self::get_power_delivery_configuration
    #[maybe_async::maybe_async]
    pub async fn get_operating_mode(&mut self) -> Result<OperationMode, Ap33772sError> {
        self.read_one_byte_command::<OperationMode>().await
    }

    /// Reads the Power Delivery configuration settings.
    ///
    /// This function retrieves the current Power Delivery mode configuration,
    /// indicating which advanced PD features are enabled such as Extended Power Range
    /// (EPR) and Programmable/Adjustable Power Supply modes.
    ///
    /// # Returns
    ///
    /// Returns a [`PowerDeliveryMode`] struct containing:
    /// - `extended_power_range_mode_enabled` - Whether EPR (up to 28V) is supported
    /// - `programmable_power_supply_adjustable_voltage_supply_enabled` - Whether PPS/AVS is available
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::Ap33772s;
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let pd_config = device.get_power_delivery_configuration()?;
    ///
    /// if pd_config.extended_power_range_mode_enabled {
    ///     println!("Extended Power Range (EPR) mode available - up to 28V");
    /// }
    ///
    /// if pd_config.programmable_power_supply_adjustable_voltage_supply_enabled {
    ///     println!("Programmable Power Supply (PPS) mode available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`set_power_delivery_mode`] to configure PD settings
    /// - [`get_all_source_power_capabilities`] for available power options
    ///
    /// [`PowerDeliveryMode`]: crate::types::PowerDeliveryMode
    /// [`set_power_delivery_mode`]: crate::Ap33772s::set_power_delivery_mode
    /// [`get_all_source_power_capabilities`]: Self::get_all_source_power_capabilities
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

    /// Retrieves comprehensive electrical and thermal statistics from the device.
    ///
    /// This is a convenient high-level function that collects all current operational
    /// measurements and requested parameters into a single structure. It combines
    /// multiple individual sensor readings and calculations.
    ///
    /// # Returns
    ///
    /// Returns a [`Statistics`] struct containing:
    /// - `current` - Actual output current to the load
    /// - `voltage` - Actual output voltage to the load  
    /// - `temperature` - Device junction temperature
    /// - `power` - Calculated output power (V × I)
    /// - `requested_voltage` - Voltage requested from PD source
    /// - `requested_current` - Current requested from PD source
    /// - `requested_power` - Calculated requested power
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if any of the underlying I2C communications fail.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let stats = device.get_statistics()?;
    ///
    /// println!("Output: {:.2}V @ {:.2}A = {:.1}W",
    ///          stats.voltage.get::<volt>(),
    ///          stats.current.get::<ampere>(),
    ///          stats.power.get::<watt>());
    ///          
    /// println!("Requested: {:.2}V @ {:.2}A = {:.1}W",
    ///          stats.requested_voltage.get::<volt>(),
    ///          stats.requested_current.get::<ampere>(),
    ///          stats.requested_power.get::<watt>());
    ///          
    /// println!("Temperature: {:.1}°C", stats.temperature.get::<degree_celsius>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Note
    ///
    /// This function performs multiple I2C transactions. For applications requiring
    /// only specific measurements, consider using individual getter functions instead.
    ///
    /// # See Also
    ///
    /// - [`get_current`], [`get_voltage`], [`get_temperature`] for individual measurements
    /// - [`get_power`] for just the calculated power value
    /// - [`get_requested_voltage`], [`get_requested_current`] for PD request values
    ///
    /// [`Statistics`]: crate::types::Statistics
    /// [`get_current`]: Self::get_current
    /// [`get_voltage`]: Self::get_voltage  
    /// [`get_temperature`]: Self::get_temperature
    /// [`get_power`]: Self::get_power
    /// [`get_requested_voltage`]: Self::get_requested_voltage
    /// [`get_requested_current`]: Self::get_requested_current
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

    /// Gets the result of a Power Delivery request message.
    ///
    /// This function retrieves the response from the AP33772S after a power delivery
    /// request has been sent. It should be called after [`send_power_delivery_request`]
    /// with appropriate timing to allow the device to process the request.
    ///
    /// **Note**: This function is only publicly available when the `advanced` feature is enabled.
    /// In normal mode, power delivery is handled through higher-level functions like
    /// [`negotiate_power_delivery`].
    ///
    /// # Returns
    ///
    /// Returns a [`PowerDeliveryResponse`] indicating:
    /// - `Accepted` - Request was accepted by the source
    /// - `Rejected` - Request was rejected by the source  
    /// - `Busy` - Device is still processing (try again later)
    /// - Other response codes as defined in the USB PD specification
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::DataMalformed`] if the response contains invalid data
    ///
    /// # Timing
    ///
    /// It's recommended to wait at least 100ms after sending a power delivery request
    /// before calling this function to allow the negotiation to complete.
    ///
    /// # Example (Advanced Mode)
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "advanced")]
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::types::command_structures::*;
    ///
    /// // Send a power delivery request
    /// device.send_power_delivery_request(
    ///     PowerDataObject::Two,
    ///     None, // Fixed PDO
    ///     OperatingCurrentSelection::Maximum,
    ///     &source_capabilities
    /// ).await?;
    ///
    /// // Wait for processing
    /// device.delay.delay_ms(100).await;
    ///
    /// // Check the result
    /// match device.get_power_delivery_request_result().await? {
    ///     PowerDeliveryResponse::Success => println!("Request accepted!"),
    ///     PowerDeliveryResponse::NotSupported => println!("Request rejected"),
    ///     PowerDeliveryResponse::Busy => println!("Still processing..."),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`send_power_delivery_request`] to initiate a PD request
    /// - [`negotiate_power_delivery`] for high-level PD negotiation
    ///
    /// [`PowerDeliveryResponse`]: crate::types::command_structures::PowerDeliveryResponse
    /// [`send_power_delivery_request`]: crate::Ap33772s::send_power_delivery_request
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
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
    /// Gets the current output voltage control override setting.
    ///
    /// This function reads the system control register to determine whether the
    /// output voltage switch (VOUT) is currently enabled, disabled, or under
    /// automatic control.
    ///
    /// # Returns
    ///
    /// Returns a [`VoltageOutputControl`] enum indicating:
    /// - `Enabled` - Output voltage is forcibly enabled
    /// - `Disabled` - Output voltage is forcibly disabled  
    /// - `Auto` - Output voltage is under automatic PD control
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::DataMalformed`] if the register contains invalid data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::command_structures::VoltageOutputControl};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let control = device.get_voltage_out_override().await?;
    ///
    /// match control {
    ///     VoltageOutputControl::Enabled => println!("Output forcibly enabled"),
    ///     VoltageOutputControl::Disabled => println!("Output forcibly disabled"),  
    ///     VoltageOutputControl::Auto => println!("Output under PD control"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`override_output_voltage`] to set the voltage output control
    ///
    /// [`VoltageOutputControl`]: crate::types::command_structures::VoltageOutputControl
    /// [`override_output_voltage`]: crate::Ap33772s::override_output_voltage
    #[maybe_async::maybe_async]
    pub async fn get_voltage_out_override(
        &mut self,
    ) -> Result<VoltageOutputControl, Ap33772sError> {
        let system_control = self.read_one_byte_command::<SystemControl>().await?;
        system_control
            .v_out_control()
            .map_err(Ap33772sError::DataMalformed)
    }

    /// Reads the current output current measurement.
    ///
    /// This function provides the instantaneous current being delivered to the load.
    /// The measurement is taken from the device's internal current sensing circuitry.
    ///
    /// # Returns
    ///
    /// Returns the output current as an [`ElectricCurrent`] with appropriate units.
    /// The value can be accessed using `.get::<ampere>()` or `.get::<milliampere>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the raw value cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let current = device.get_current().await?;
    ///
    /// println!("Output current: {:.2}A ({:.0}mA)",
    ///          current.get::<ampere>(),
    ///          current.get::<milliampere>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_voltage`] for voltage measurement
    /// - [`get_power`] for calculated power (V × I)
    /// - [`get_statistics`] for all measurements together
    ///
    /// [`ElectricCurrent`]: crate::units::ElectricCurrent
    /// [`get_voltage`]: Self::get_voltage
    /// [`get_power`]: Self::get_power
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let current = self.read_one_byte_command::<Current>().await?;
        current.current()
    }

    /// Reads the current output voltage measurement.
    ///
    /// This function provides the instantaneous voltage being delivered to the load.
    /// The measurement represents the actual output voltage after any regulation
    /// and load-dependent voltage drops.
    ///
    /// # Returns
    ///
    /// Returns the output voltage as an [`ElectricPotential`] with appropriate units.
    /// The value can be accessed using `.get::<volt>()` or `.get::<millivolt>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the raw value cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let voltage = device.get_voltage().await?;
    ///
    /// println!("Output voltage: {:.2}V ({:.0}mV)",
    ///          voltage.get::<volt>(),
    ///          voltage.get::<millivolt>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_current`] for current measurement  
    /// - [`get_power`] for calculated power (V × I)
    /// - [`get_requested_voltage`] for the voltage requested from source
    /// - [`get_statistics`] for all measurements together
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`get_current`]: Self::get_current
    /// [`get_power`]: Self::get_power
    /// [`get_requested_voltage`]: Self::get_requested_voltage
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage = self.read_two_byte_command::<Voltage>().await?;
        voltage.voltage()
    }

    /// Reads the device junction temperature.
    ///
    /// This function provides the internal temperature of the AP33772S die, which
    /// is used for thermal protection and derating decisions. The temperature
    /// measurement is critical for safe operation and thermal management.
    ///
    /// # Returns
    ///
    /// Returns the junction temperature as a [`ThermodynamicTemperature`].
    /// The value can be accessed using `.get::<degree_celsius>()`.
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let temperature = device.get_temperature().await?;
    ///
    /// println!("Junction temperature: {:.1}°C", temperature.get::<degree_celsius>());
    ///
    /// if temperature.get::<degree_celsius>() > 85.0 {
    ///     println!("Warning: High temperature detected!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Thermal Management
    ///
    /// The temperature reading is used internally for:
    /// - Over-temperature protection (OTP)
    /// - Derating threshold monitoring
    /// - Thermal resistance calculations
    ///
    /// # See Also
    ///
    /// - [`get_thresholds`] for temperature protection thresholds
    /// - [`get_thermal_resistances`] for thermal resistance values
    /// - [`get_statistics`] for all measurements together
    ///
    /// [`ThermodynamicTemperature`]: crate::units::ThermodynamicTemperature
    /// [`get_thresholds`]: Self::get_thresholds
    /// [`get_thermal_resistances`]: Self::get_thermal_resistances
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_temperature(&mut self) -> Result<ThermodynamicTemperature, Ap33772sError> {
        let temperature = self.read_one_byte_command::<Temperature>().await?;
        Ok(temperature.temperature())
    }

    /// Calculates the current output power.
    ///
    /// This function reads both voltage and current measurements and calculates
    /// the instantaneous power being delivered to the load using P = V × I.
    ///
    /// # Returns
    ///
    /// Returns the calculated power as a [`Power`] value.
    /// The value can be accessed using `.get::<watt>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails during voltage or current reading
    /// - [`Ap33772sError::ConversionFailed`] if raw values cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let power = device.get_power().await?;
    ///
    /// println!("Output power: {:.2}W", power.get::<watt>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Note
    ///
    /// This function performs two I2C transactions (voltage and current).
    /// If you need all measurements, consider using [`get_statistics`] instead.
    ///
    /// # See Also
    ///
    /// - [`get_current`] and [`get_voltage`] for individual measurements
    /// - [`get_requested_power`] for the power requested from source
    /// - [`get_statistics`] for all measurements including power
    ///
    /// [`Power`]: crate::units::Power
    /// [`get_current`]: Self::get_current
    /// [`get_voltage`]: Self::get_voltage
    /// [`get_requested_power`]: Self::get_requested_power
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_power(&mut self) -> Result<Power, Ap33772sError> {
        let current = self.get_current().await?;
        let voltage = self.get_voltage().await?;
        let power = current * voltage;
        Ok(power)
    }
    /// Reads the voltage currently requested from the PD source.
    ///
    /// This function returns the voltage value that was requested from the Power Delivery
    /// source during the last negotiation. This may differ from the actual output voltage
    /// due to load regulation or other factors.
    ///
    /// # Returns
    ///
    /// Returns the requested voltage as an [`ElectricPotential`].
    /// The value can be accessed using `.get::<volt>()` or `.get::<millivolt>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the raw value cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let requested = device.get_requested_voltage().await?;
    /// let actual = device.get_voltage().await?;
    ///
    /// println!("Requested: {:.2}V, Actual: {:.2}V",
    ///          requested.get::<volt>(),
    ///          actual.get::<volt>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_voltage`] for actual output voltage
    /// - [`get_requested_current`] for requested current
    /// - [`negotiate_power_delivery`] to request different voltage/current
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`get_voltage`]: Self::get_voltage
    /// [`get_requested_current`]: Self::get_requested_current
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
    #[maybe_async::maybe_async]
    pub async fn get_requested_voltage(&mut self) -> Result<ElectricPotential, Ap33772sError> {
        let requested_voltage = self.read_two_byte_command::<VoltageRequested>().await?;
        requested_voltage.voltage()
    }

    /// Reads the current currently requested from the PD source.
    ///
    /// This function returns the current value that was requested from the Power Delivery
    /// source during the last negotiation. This represents the maximum current that
    /// the device is allowed to draw from the source.
    ///
    /// # Returns
    ///
    /// Returns the requested current as an [`ElectricCurrent`].
    /// The value can be accessed using `.get::<ampere>()` or `.get::<milliampere>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the raw value cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let requested = device.get_requested_current().await?;
    /// let actual = device.get_current().await?;
    ///
    /// println!("Current limit: {:.2}A, Actual draw: {:.2}A",
    ///          requested.get::<ampere>(),
    ///          actual.get::<ampere>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_current`] for actual output current
    /// - [`get_requested_voltage`] for requested voltage
    /// - [`negotiate_power_delivery`] to request different voltage/current
    ///
    /// [`ElectricCurrent`]: crate::units::ElectricCurrent
    /// [`get_current`]: Self::get_current
    /// [`get_requested_voltage`]: Self::get_requested_voltage
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
    #[maybe_async::maybe_async]
    pub async fn get_requested_current(&mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let requested_current = self.read_two_byte_command::<CurrentRequested>().await?;
        requested_current.current()
    }

    /// Calculates the power currently requested from the PD source.
    ///
    /// This function reads both requested voltage and current values and calculates
    /// the power that was negotiated with the Power Delivery source using P = V × I.
    /// This represents the maximum power the device is allowed to draw.
    ///
    /// # Returns
    ///
    /// Returns the calculated requested power as a [`Power`] value.
    /// The value can be accessed using `.get::<watt>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails during either read operation
    /// - [`Ap33772sError::ConversionFailed`] if raw values cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let requested_power = device.get_requested_power().await?;
    /// let actual_power = device.get_power().await?;
    ///
    /// println!("Power budget: {:.1}W, Actual usage: {:.1}W",
    ///          requested_power.get::<watt>(),
    ///          actual_power.get::<watt>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Note
    ///
    /// This function performs two I2C transactions. If you need all measurements,
    /// consider using [`get_statistics`] instead.
    ///
    /// # See Also
    ///
    /// - [`get_requested_voltage`] and [`get_requested_current`] for individual values
    /// - [`get_power`] for actual power consumption
    /// - [`get_statistics`] for all measurements including requested power
    ///
    /// [`Power`]: crate::units::Power
    /// [`get_requested_voltage`]: Self::get_requested_voltage
    /// [`get_requested_current`]: Self::get_requested_current
    /// [`get_power`]: Self::get_power
    /// [`get_statistics`]: Self::get_statistics
    #[maybe_async::maybe_async]
    pub async fn get_requested_power(&mut self) -> Result<Power, Ap33772sError> {
        let requested_voltage = self.get_requested_voltage().await?;
        let requested_current = self.get_requested_current().await?;
        let requested_power = requested_voltage * requested_current;
        Ok(requested_power)
    }

    /// Reads the minimum voltage selection threshold.
    ///
    /// This function retrieves the minimum voltage threshold that has been configured
    /// for Power Delivery negotiation. The device will not accept PDOs below this voltage.
    ///
    /// # Returns
    ///
    /// Returns the minimum selection voltage as an [`ElectricPotential`].
    /// The value can be accessed using `.get::<volt>()` or `.get::<millivolt>()`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the raw value cannot be converted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let min_voltage = device.get_minimum_selection_voltage().await?;
    ///
    /// println!("Minimum acceptable voltage: {:.1}V", min_voltage.get::<volt>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`set_minimum_selection_voltage`] to configure the minimum voltage
    /// - [`get_all_source_power_capabilities`] to see available voltage options
    ///
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`set_minimum_selection_voltage`]: crate::Ap33772s::set_minimum_selection_voltage
    /// [`get_all_source_power_capabilities`]: Self::get_all_source_power_capabilities
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
    /// Reads all thermal resistance values from the device.
    ///
    /// Thermal resistances are used by the AP33772S for thermal modeling and protection.
    /// These values represent the thermal resistance from junction to ambient at different
    /// operating points and are critical for accurate thermal management.
    ///
    /// # Returns
    ///
    /// Returns a [`ThermalResistances`] struct containing resistance values at:
    /// - `_25` - Thermal resistance at 25°C ambient
    /// - `_50` - Thermal resistance at 50°C ambient  
    /// - `_75` - Thermal resistance at 75°C ambient
    /// - `_100` - Thermal resistance at 100°C ambient
    ///
    /// All values are in units of [`ElectricalResistance`] and can be accessed using
    /// `.get::<ohm>()` or `.get::<milliohm>()`.
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if any of the I2C communications fail.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let thermal = device.get_thermal_resistances().await?;
    ///
    /// println!("Thermal resistances:");
    /// println!("  25°C: {:.2}Ω", thermal._25.get::<ohm>());
    /// println!("  50°C: {:.2}Ω", thermal._50.get::<ohm>());
    /// println!("  75°C: {:.2}Ω", thermal._75.get::<ohm>());
    /// println!(" 100°C: {:.2}Ω", thermal._100.get::<ohm>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Thermal Modeling
    ///
    /// These values are used internally by the device to:
    /// - Predict junction temperature rise
    /// - Implement thermal derating
    /// - Provide accurate thermal protection
    ///
    /// # See Also
    ///
    /// - [`set_thermal_resistances`] to configure thermal resistance values
    /// - [`get_temperature`] for current junction temperature
    /// - [`get_thresholds`] for thermal protection thresholds
    ///
    /// [`ThermalResistances`]: crate::types::ThermalResistances
    /// [`ElectricalResistance`]: crate::units::ElectricalResistance
    /// [`set_thermal_resistances`]: crate::Ap33772s::set_thermal_resistances
    /// [`get_temperature`]: Self::get_temperature
    /// [`get_thresholds`]: Self::get_thresholds
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

    /// Reads all protection threshold values from the device.
    ///
    /// Protection thresholds define the limits at which the device will take protective
    /// action to prevent damage from overcurrent, overvoltage, overtemperature, or
    /// undervoltage conditions.
    ///
    /// # Returns
    ///
    /// Returns a [`Thresholds`] struct containing:
    /// - `over_voltage` - Over-voltage protection threshold [`ElectricPotential`]
    /// - `over_current` - Over-current protection threshold [`ElectricCurrent`]  
    /// - `over_temperature` - Over-temperature protection threshold [`ThermodynamicTemperature`]
    /// - `under_voltage` - Under-voltage protection threshold [`ElectricPotential`]
    /// - `derating` - Temperature derating threshold [`ThermodynamicTemperature`]
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if any I2C communication fails
    /// - [`Ap33772sError::DataMalformed`] if any threshold register contains invalid data
    /// - [`Ap33772sError::ConversionFailed`] if raw values cannot be converted to proper units
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let thresholds = device.get_thresholds().await?;
    ///
    /// println!("Protection thresholds:");
    /// println!("  Over voltage: {:.1}V", thresholds.over_voltage.get::<volt>());
    /// println!("  Over current: {:.2}A", thresholds.over_current.get::<ampere>());
    /// println!("  Over temperature: {:.0}°C", thresholds.over_temperature.get::<degree_celsius>());
    /// println!("  Under voltage: {:.1}V", thresholds.under_voltage.get::<volt>());
    /// println!("  Derating temp: {:.0}°C", thresholds.derating.get::<degree_celsius>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Protection Behavior
    ///
    /// When thresholds are exceeded:
    /// - **Over voltage/current/temperature**: Device immediately shuts down output
    /// - **Under voltage**: Device may shut down or reduce output power
    /// - **Derating temperature**: Device reduces current limit by 50%
    ///
    /// # See Also
    ///
    /// - [`set_thresholds`] to configure protection thresholds
    /// - [`get_status`] to check for active protection events
    /// - [`get_operating_mode`] to check derating mode status
    ///
    /// [`Thresholds`]: crate::types::Thresholds
    /// [`ElectricPotential`]: crate::units::ElectricPotential
    /// [`ElectricCurrent`]: crate::units::ElectricCurrent
    /// [`ThermodynamicTemperature`]: crate::units::ThermodynamicTemperature
    /// [`set_thresholds`]: crate::Ap33772s::set_thresholds
    /// [`get_status`]: Self::get_status
    /// [`get_operating_mode`]: Self::get_operating_mode
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

    /// Retrieves all available Power Data Objects (PDOs) from the connected source.
    ///
    /// This function reads the complete list of power capabilities advertised by the
    /// USB-C Power Delivery source. The PDOs define what voltage, current, and power
    /// combinations are available for negotiation.
    ///
    /// # Returns
    ///
    /// Returns an [`AllSourceDataPowerDataObject`] containing:
    /// - Up to 7 Standard Power Range (SPR) PDOs (5V, 9V, 12V, 15V, 20V)
    /// - Up to 6 Extended Power Range (EPR) PDOs (15V-28V)
    /// - Each PDO contains voltage, current, and power specifications
    /// - PDO type information (Fixed, Variable, Battery, or PPS/AVS)
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::command_structures::PowerDataObject};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// let capabilities = device.get_all_source_power_capabilities().await?;
    ///
    /// // Check what PDOs are available
    /// for (i, pdo) in capabilities.power_data_objects.iter().enumerate() {
    ///     if let Ok(max_voltage) = pdo.get_max_voltage() {
    ///         if let Ok(max_current) = pdo.get_max_current() {
    ///             println!("PDO {}: {:.1}V @ {:.2}A",
    ///                      i, max_voltage.get::<volt>(), max_current.get::<ampere>());
    ///         }
    ///     }
    /// }
    ///
    /// // Use a specific PDO for negotiation
    /// device.negotiate_power_delivery(
    ///     PowerDataObject::Two,
    ///     None, // Fixed PDO
    ///     OperatingCurrentSelection::Maximum
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Power Data Object Types
    ///
    /// - **Fixed PDO**: Provides constant voltage (e.g., 5V, 9V, 12V)
    /// - **Variable PDO**: Allows voltage adjustment within a range
    /// - **Battery PDO**: Specifies power range for battery charging
    /// - **PPS PDO**: Programmable Power Supply (3.3V-21V adjustable)
    /// - **AVS PDO**: Adjustable Voltage Supply (15V-28V adjustable)
    ///
    /// # Usage in Power Negotiation
    ///
    /// The returned PDO information is used with:
    /// - [`negotiate_power_delivery`] for high-level negotiation
    /// - [`send_power_delivery_request`] for low-level control (advanced mode)
    ///
    /// # See Also
    ///
    /// - [`negotiate_power_delivery`] to negotiate power with these capabilities
    /// - [`get_power_delivery_configuration`] for current PD mode settings
    /// - [Understanding USB-C PD Specification](https://github.com/ScottGibb/AP33772S-rs/blob/main/docs/understanding-the-usb-c-pd-specification.md)
    ///
    /// [`AllSourceDataPowerDataObject`]: crate::types::command_structures::AllSourceDataPowerDataObject
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
    /// [`send_power_delivery_request`]: crate::Ap33772s::send_power_delivery_request
    /// [`get_power_delivery_configuration`]: Self::get_power_delivery_configuration
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
