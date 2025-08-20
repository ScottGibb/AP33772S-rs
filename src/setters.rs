//! # Setter Methods for AP33772S Device
//!
//! This module provides comprehensive methods to configure and control the AP33772S 
//! USB-C Power Delivery controller. These functions allow you to modify device 
//! behavior, request power delivery changes, and configure protection parameters.
//!
//! ## Categories of Setters
//!
//! ### Power Delivery Control
//! - [`set_power_delivery_mode`] - Configure PD modes (EPR, PPS/AVS support)
//! - [`send_power_delivery_request`] - Send low-level PD requests (advanced mode)
//! - [`send_maximum_power_delivery_request`] - Request maximum power from a PDO
//! - [`set_minimum_selection_voltage`] - Set minimum acceptable voltage
//!
//! ### Output Control
//! - [`override_output_voltage`] - Control the output voltage switch (VOUT)
//!
//! ### Protection and Thermal Configuration  
//! - [`set_thermal_resistances`] - Configure thermal resistance values
//! - [`set_thresholds`] - Set protection thresholds (OVP, OCP, OTP, UVP, derating)
//!
//! ## Power Delivery Request Flow
//!
//! For advanced power delivery control (when `advanced` feature is enabled):
//!
//! 1. Get available capabilities with [`get_all_source_power_capabilities`]
//! 2. Send request with [`send_power_delivery_request`] 
//! 3. Wait for processing (typically 100ms)
//! 4. Check result with [`get_power_delivery_request_result`]
//!
//! For simplified control, use the high-level [`negotiate_power_delivery`] function
//! which handles the entire flow automatically.
//!
//! ## Safety Considerations
//!
//! - **Protection thresholds**: Set appropriate values based on your hardware design
//! - **Thermal resistances**: Must match your PCB and thermal design
//! - **Power requests**: Ensure requested power doesn't exceed system capabilities
//! - **Voltage control**: Use caution when overriding automatic voltage control
//!
//! ## Usage Examples
//!
//! ```rust,no_run
//! # use ap33772s_rs::{Ap33772s, types::*};
//! # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
//! // Configure thermal protection for safe operation
//! let thresholds = Thresholds {
//!     over_voltage: ElectricPotential::new::<volt>(21.0),
//!     over_current: ElectricCurrent::new::<ampere>(3.25),
//!     over_temperature: ThermodynamicTemperature::new::<degree_celsius>(85.0),
//!     under_voltage: ElectricPotential::new::<volt>(4.5),
//!     derating: ThermodynamicTemperature::new::<degree_celsius>(75.0),
//! };
//! device.set_thresholds(thresholds).await?;
//!
//! // Enable extended power range mode for higher voltages
//! let pd_mode = PowerDeliveryMode {
//!     extended_power_range_mode_enabled: true,
//!     programmable_power_supply_adjustable_voltage_supply_enabled: true,
//! };
//! device.set_power_delivery_mode(pd_mode).await?;
//!
//! // Set minimum acceptable voltage to 9V
//! device.set_minimum_selection_voltage(
//!     ElectricPotential::new::<volt>(9.0)
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! [`set_power_delivery_mode`]: crate::setters::Ap33772s::set_power_delivery_mode
//! [`send_power_delivery_request`]: crate::setters::Ap33772s::send_power_delivery_request
//! [`send_maximum_power_delivery_request`]: crate::setters::Ap33772s::send_maximum_power_delivery_request
//! [`set_minimum_selection_voltage`]: crate::setters::Ap33772s::set_minimum_selection_voltage
//! [`override_output_voltage`]: crate::setters::Ap33772s::override_output_voltage
//! [`set_thermal_resistances`]: crate::setters::Ap33772s::set_thermal_resistances
//! [`set_thresholds`]: crate::setters::Ap33772s::set_thresholds
//! [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
//! [`get_power_delivery_request_result`]: crate::getters::Ap33772s::get_power_delivery_request_result
//! [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
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
use crate::errors::Ap33772sError;

use crate::errors::RequestError;
// Public API Types
use crate::types::command_structures::*;
use crate::types::units::*;
use crate::types::*;

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    /// Controls the output voltage switch (VOUT) state.
    ///
    /// This function allows manual override of the automatic power delivery output control.
    /// It can force the output voltage switch on, off, or return it to automatic PD control.
    ///
    /// # Parameters
    ///
    /// - `voltage_output` - The desired output control state:
    ///   - [`VoltageOutputControl::Enabled`] - Force output ON regardless of PD state
    ///   - [`VoltageOutputControl::Disabled`] - Force output OFF regardless of PD state  
    ///   - [`VoltageOutputControl::Auto`] - Return to automatic PD control (default)
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Safety
    ///
    /// **Warning**: Forcing the output enabled bypasses normal Power Delivery safety mechanisms.
    /// Only use this for testing or emergency situations. The automatic mode should be used
    /// for normal operation to ensure safe power delivery negotiation.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::command_structures::VoltageOutputControl};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Temporarily disable output for safety
    /// device.override_output_voltage(VoltageOutputControl::Disabled).await?;
    /// 
    /// // Perform some configuration...
    /// 
    /// // Return to automatic control
    /// device.override_output_voltage(VoltageOutputControl::Auto).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_voltage_out_override`] to read current output control state
    /// - [`negotiate_power_delivery`] for proper PD-controlled power management
    ///
    /// [`VoltageOutputControl::Enabled`]: crate::types::command_structures::VoltageOutputControl::Enabled
    /// [`VoltageOutputControl::Disabled`]: crate::types::command_structures::VoltageOutputControl::Disabled
    /// [`VoltageOutputControl::Auto`]: crate::types::command_structures::VoltageOutputControl::Auto
    /// [`get_voltage_out_override`]: crate::getters::Ap33772s::get_voltage_out_override
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
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

    /// Sets the minimum voltage selection threshold for Power Delivery negotiation.
    ///
    /// This function configures the lowest voltage that the device will accept during
    /// Power Delivery negotiation. Any Power Data Objects (PDOs) offering voltages
    /// below this threshold will be rejected.
    ///
    /// # Parameters
    ///
    /// - `voltage` - The minimum acceptable voltage as an [`ElectricPotential`].
    ///   Typically specified in volts, e.g., `ElectricPotential::new::<volt>(9.0)`.
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::ConversionFailed`] if the voltage cannot be converted to device format
    ///
    /// # Voltage Range
    ///
    /// The acceptable range depends on the PDO types supported by the source:
    /// - **Standard Power Range**: Typically 5V minimum
    /// - **Extended Power Range**: Typically 15V minimum  
    /// - **PPS**: Usually supports down to 3.3V
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::units::*};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Only accept 9V or higher voltages
    /// device.set_minimum_selection_voltage(
    ///     ElectricPotential::new::<volt>(9.0)
    /// ).await?;
    /// 
    /// // This ensures 5V PDOs will be rejected, forcing negotiation to higher voltages
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Efficiency optimization**: Higher voltages often provide better efficiency
    /// - **Power requirements**: Some loads require minimum voltage to function
    /// - **Fast charging**: Higher voltages enable faster charging rates
    ///
    /// # See Also
    ///
    /// - [`get_minimum_selection_voltage`] to read current threshold
    /// - [`get_all_source_power_capabilities`] to see available voltages
    /// - [`negotiate_power_delivery`] which respects this threshold
    ///
    /// [`ElectricPotential`]: crate::types::units::ElectricPotential  
    /// [`get_minimum_selection_voltage`]: crate::getters::Ap33772s::get_minimum_selection_voltage
    /// [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
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

    /// Configures Power Delivery mode settings.
    ///
    /// This function controls advanced USB-C Power Delivery features, specifically
    /// whether Extended Power Range (EPR) and Programmable/Adjustable Power Supply
    /// modes are enabled and advertised to the source.
    ///
    /// # Parameters
    ///
    /// - `mode` - A [`PowerDeliveryMode`] struct containing:
    ///   - `extended_power_range_mode_enabled` - Enables EPR mode (up to 28V @ 5A)
    ///   - `programmable_power_supply_adjustable_voltage_supply_enabled` - Enables PPS/AVS support
    ///
    /// # Power Delivery Modes
    ///
    /// ## Extended Power Range (EPR)
    /// - Supports voltages from 15V to 28V (device dependent)
    /// - Requires 5A-rated electronically marked cables
    /// - Enables higher power applications (>100W)
    ///
    /// ## Programmable Power Supply (PPS) / Adjustable Voltage Supply (AVS)
    /// - **PPS**: Adjustable voltage in Standard Power Range (3.3V-21V)
    /// - **AVS**: Adjustable voltage in Extended Power Range (15V-28V)
    /// - Allows fine-grained voltage control for optimal efficiency
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::PowerDeliveryMode};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Enable all advanced PD features
    /// let advanced_mode = PowerDeliveryMode {
    ///     extended_power_range_mode_enabled: true,
    ///     programmable_power_supply_adjustable_voltage_supply_enabled: true,
    /// };
    /// device.set_power_delivery_mode(advanced_mode).await?;
    /// 
    /// // Standard mode (SPR only, fixed voltages)
    /// let standard_mode = PowerDeliveryMode {
    ///     extended_power_range_mode_enabled: false,
    ///     programmable_power_supply_adjustable_voltage_supply_enabled: false,
    /// };
    /// device.set_power_delivery_mode(standard_mode).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Hardware Requirements
    ///
    /// - **EPR mode**: Requires 5A-rated USB-C cable and connectors
    /// - **High power**: Ensure adequate thermal management for >100W
    /// - **Safety**: Configure appropriate protection thresholds
    ///
    /// # See Also
    ///
    /// - [`get_power_delivery_configuration`] to read current mode settings
    /// - [`set_thresholds`] to configure protection for high-power operation
    /// - [USB PD Specification Documentation](https://github.com/ScottGibb/AP33772S-rs/blob/main/docs/understanding-the-usb-c-pd-specification.md)
    ///
    /// [`PowerDeliveryMode`]: crate::types::PowerDeliveryMode
    /// [`get_power_delivery_configuration`]: crate::getters::Ap33772s::get_power_delivery_configuration
    /// [`set_thresholds`]: Self::set_thresholds
    #[maybe_async::maybe_async]
    pub async fn set_power_delivery_mode(
        &mut self,
        mode: PowerDeliveryMode,
    ) -> Result<(), Ap33772sError> {
        let command = PowerDeliveryConfiguration::builder()
            .with_extended_power_delivery_enabled(mode.extended_power_range_mode_enabled)
            .with_programmable_power_supply_and_adjustable_power_supply_enabled(
                mode.programmable_power_supply_adjustable_voltage_supply_enabled,
            )
            .build();
        self.write_one_byte_command(command).await
    }

    /// Sends a Power Delivery request to the connected source (Advanced Mode).
    ///
    /// This function sends a low-level USB-C Power Delivery request message to negotiate
    /// a specific Power Data Object (PDO) with custom voltage and current parameters.
    /// This provides fine-grained control over the power negotiation process.
    ///
    /// **Note**: This function is only publicly available when the `advanced` feature is enabled.
    /// For normal operation, use [`negotiate_power_delivery`] which provides a higher-level interface.
    ///
    /// # Parameters
    ///
    /// - `power_data_object_index` - Which PDO to request (PowerDataObject::One through PowerDataObject::Thirteen)
    /// - `voltage_selection` - Desired voltage for variable PDOs (None for fixed PDOs)  
    /// - `current_selection` - Operating current selection mode
    /// - `data_objects` - Available source capabilities from [`get_all_source_power_capabilities`]
    ///
    /// # Voltage Selection Behavior
    ///
    /// - **Fixed PDOs**: `voltage_selection` must be `None` (voltage is predetermined)
    /// - **Variable/PPS/AVS PDOs**: `voltage_selection` specifies the desired voltage within the PDO's range
    ///
    /// # Current Selection Options
    ///
    /// - [`OperatingCurrentSelection::Maximum`] - Request maximum available current
    /// - [`OperatingCurrentSelection::Minimum`] - Request minimum operating current
    /// - Custom current values (device dependent)
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if communication fails
    /// - [`Ap33772sError::InvalidRequest`] with specific error:
    ///   - [`RequestError::CurrentOutOfRange`] - Requested current exceeds PDO capability
    ///   - [`RequestError::VoltageOutOfRange`] - Requested voltage outside PDO range
    ///   - [`RequestError::MissingArgument`] - Missing voltage for variable PDO
    /// - [`Ap33772sError::ConversionFailed`] if parameters cannot be converted to device format
    ///
    /// # Timing and Response
    ///
    /// This function only **sends** the request. To check if it was accepted:
    /// 1. Wait approximately 100ms for processing
    /// 2. Call [`get_power_delivery_request_result`] to check the response
    ///
    /// # Example (Advanced Mode)
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "advanced")]
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::types::{command_structures::*, units::*};
    /// 
    /// // Get available power capabilities
    /// let capabilities = device.get_all_source_power_capabilities().await?;
    /// 
    /// // Request PDO #2 (often 9V) at maximum current  
    /// device.send_power_delivery_request(
    ///     PowerDataObject::Two,
    ///     None, // Fixed PDO - no voltage selection needed
    ///     OperatingCurrentSelection::Maximum,
    ///     &capabilities
    /// ).await?;
    /// 
    /// // Wait for processing
    /// device.delay.delay_ms(100).await;
    /// 
    /// // Check if request was accepted
    /// match device.get_power_delivery_request_result().await? {
    ///     PowerDeliveryResponse::Accepted => println!("Request accepted!"),
    ///     PowerDeliveryResponse::Rejected => println!("Request rejected"),
    ///     _ => println!("Unexpected response"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # PPS/AVS Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "advanced")]
    /// # async fn pps_example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// # use ap33772s_rs::types::{command_structures::*, units::*};
    /// # let capabilities = device.get_all_source_power_capabilities().await?;
    /// // Request adjustable voltage PDO at 12.5V
    /// device.send_power_delivery_request(
    ///     PowerDataObject::Seven, // Assuming this is a PPS PDO
    ///     Some(ElectricPotential::new::<volt>(12.5)), // Custom voltage
    ///     OperatingCurrentSelection::Maximum,
    ///     &capabilities
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Safety Considerations
    ///
    /// - Verify PDO capabilities before requesting
    /// - Ensure requested power doesn't exceed system limits
    /// - Monitor protection flags after successful negotiation
    ///
    /// # See Also
    ///
    /// - [`negotiate_power_delivery`] for high-level power negotiation
    /// - [`get_power_delivery_request_result`] to check request status
    /// - [`get_all_source_power_capabilities`] to get available PDOs
    /// - [`send_maximum_power_delivery_request`] for simplified maximum power requests
    ///
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
    /// [`get_all_source_power_capabilities`]: crate::getters::Ap33772s::get_all_source_power_capabilities
    /// [`get_power_delivery_request_result`]: crate::getters::Ap33772s::get_power_delivery_request_result
    /// [`send_maximum_power_delivery_request`]: Self::send_maximum_power_delivery_request
    /// [`OperatingCurrentSelection::Maximum`]: crate::types::command_structures::OperatingCurrentSelection::Maximum
    /// [`OperatingCurrentSelection::Minimum`]: crate::types::command_structures::OperatingCurrentSelection::Minimum
    /// [`RequestError::CurrentOutOfRange`]: crate::errors::RequestError::CurrentOutOfRange
    /// [`RequestError::VoltageOutOfRange`]: crate::errors::RequestError::VoltageOutOfRange
    /// [`RequestError::MissingArgument`]: crate::errors::RequestError::MissingArgument
    #[maybe_async::maybe_async]
    #[cfg_attr(feature = "advanced", visibility::make(pub))]
    pub(crate) async fn send_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
        // If the Power Data Object is in Fixed Mode, the voltage selection is not needed.
        voltage_selection: Option<ElectricPotential>,
        current_selection: OperatingCurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<(), Ap33772sError> {
        let data_object = data_objects.get_power_data_object(power_data_object_index);
        // Check if the device can support the current draw
        if data_object.get_max_current().max_range() < current_selection.current() {
            return Err(Ap33772sError::InvalidRequest(
                RequestError::CurrentOutOfRange,
            ));
        }
        let delivery_message = if data_object.source_power_type() == PowerType::Fixed {
            // If we are in fixed PDO Mode, the voltage selection is not needed.
            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(0)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        } else {
            let scaling_value = f32::from(data_object.voltage_resolution());
            let voltage_selection = voltage_selection
                .ok_or(Ap33772sError::InvalidRequest(RequestError::MissingArgument))?;
            let scaled_voltage = voltage_selection.get::<millivolt>() / scaling_value;
            // Check for overflow
            let scaled_voltage = if scaled_voltage > f32::from(u8::MAX) {
                Err(Ap33772sError::ConversionFailed)
            } else {
                Ok(scaled_voltage as u8)
            }?;

            if voltage_selection > data_object.get_max_voltage()? {
                return Err(Ap33772sError::InvalidRequest(
                    RequestError::VoltageOutOfRange,
                ));
            }
            if voltage_selection < data_object.get_min_voltage()? {
                return Err(Ap33772sError::InvalidRequest(
                    RequestError::VoltageOutOfRange,
                ));
            }

            PowerDeliveryRequestMessage::builder()
                .with_voltage_selection(scaled_voltage)
                .with_current_selection(current_selection)
                .with_power_data_object_index(power_data_object_index)
                .build()
        };
        self.write_two_byte_command(delivery_message).await
    }

    /// Requests maximum power from a specific Power Data Object (Advanced Mode).
    ///
    /// This is a simplified version of [`send_power_delivery_request`] that automatically
    /// requests the maximum available power from a given PDO. It uses a special message
    /// format defined in the AP33772S datasheet for this common use case.
    ///
    /// **Note**: This function is only publicly available when the `advanced` feature is enabled.
    ///
    /// # Parameters
    ///
    /// - `power_data_object_index` - Which PDO to request maximum power from
    ///
    /// # Behavior
    ///
    /// - Automatically sets voltage selection to 0xFF (no voltage selection for fixed PDOs)
    /// - Sets current selection to maximum available
    /// - Works with both fixed and variable PDO types
    ///
    /// # Errors
    ///
    /// Returns [`Ap33772sError::I2c`] if communication with the device fails.
    ///
    /// # Timing and Response
    ///
    /// Like [`send_power_delivery_request`], this function only sends the request.
    /// Use [`get_power_delivery_request_result`] to check if the request was accepted.
    ///
    /// # Example (Advanced Mode)
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "advanced")]
    /// # async fn example(mut device: ap33772s_rs::Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// use ap33772s_rs::types::command_structures::*;
    /// 
    /// // Request maximum power from PDO #3 (often 15V or 20V)
    /// device.send_maximum_power_delivery_request(PowerDataObject::Three).await?;
    /// 
    /// // Wait for processing
    /// device.delay.delay_ms(100).await;
    /// 
    /// // Check result
    /// let result = device.get_power_delivery_request_result().await?;
    /// println!("Maximum power request result: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Fast charging**: Request maximum available power for fastest charging
    /// - **High-power applications**: Get maximum power without specifying details
    /// - **Simplified negotiation**: Avoid complex PDO analysis
    ///
    /// # See Also
    ///
    /// - [`send_power_delivery_request`] for custom voltage/current requests
    /// - [`get_power_delivery_request_result`] to check request status
    /// - [`negotiate_power_delivery`] for high-level power negotiation
    ///
    /// [`send_power_delivery_request`]: Self::send_power_delivery_request
    /// [`get_power_delivery_request_result`]: crate::getters::Ap33772s::get_power_delivery_request_result
    /// [`negotiate_power_delivery`]: crate::ap33772s::Ap33772s::negotiate_power_delivery
    #[maybe_async::maybe_async]
    #[cfg_attr(feature = "advanced", visibility::make(pub))]
    pub(crate) async fn send_maximum_power_delivery_request(
        &mut self,
        power_data_object_index: PowerDataObject,
    ) -> Result<(), Ap33772sError> {
        // Special message outlined in the AP33772S Datasheet Page 22
        let delivery_message = PowerDeliveryRequestMessage::builder()
            .with_voltage_selection(0xFF) // No Voltage Selection in Fixed Mode
            .with_current_selection(OperatingCurrentSelection::Maximum)
            .with_power_data_object_index(power_data_object_index)
            .build();
        self.write_two_byte_command(delivery_message).await
    }
}

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    /// Configures thermal resistance values for accurate thermal modeling.
    ///
    /// Thermal resistances are critical parameters that define how heat flows from the
    /// AP33772S junction to the ambient environment. These values must be accurately
    /// configured based on your PCB design, thermal management, and operating conditions
    /// for proper thermal protection and derating behavior.
    ///
    /// # Parameters
    ///
    /// - `resistances` - A [`ThermalResistances`] struct containing resistance values at different temperatures:
    ///   - `_25` - Thermal resistance from junction to ambient at 25°C
    ///   - `_50` - Thermal resistance from junction to ambient at 50°C  
    ///   - `_75` - Thermal resistance from junction to ambient at 75°C
    ///   - `_100` - Thermal resistance from junction to ambient at 100°C
    ///
    /// All values should be specified as [`ElectricalResistance`] in ohms or milliohms.
    ///
    /// # Thermal Modeling Theory
    ///
    /// Thermal resistance represents the temperature rise per unit of power dissipated:
    /// ```text
    /// ΔT = P × R_th
    /// T_junction = T_ambient + (P_dissipated × R_thermal)
    /// ```
    ///
    /// The device uses these values to:
    /// - Predict junction temperature under different loads
    /// - Implement thermal derating (current reduction at high temperatures)
    /// - Provide accurate over-temperature protection
    ///
    /// # Determining Thermal Resistance Values
    ///
    /// These values depend on your specific hardware design:
    /// - **PCB copper area and thickness** - More copper = lower thermal resistance
    /// - **Via stitching to ground planes** - Improves heat spreading  
    /// - **Component placement and airflow** - Affects ambient temperature
    /// - **Thermal pads and heatsinks** - Dramatically improve thermal performance
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if any of the I2C communications fail
    /// - [`Ap33772sError::ConversionFailed`] if resistance values cannot be converted to device format
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::{ThermalResistances, units::*}};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Example values for a well-designed PCB with good thermal management
    /// let thermal_resistances = ThermalResistances {
    ///     _25: ElectricalResistance::new::<ohm>(15.0),   // Good thermal design
    ///     _50: ElectricalResistance::new::<ohm>(18.0),   // Slightly higher at elevated temp
    ///     _75: ElectricalResistance::new::<ohm>(22.0),   // Increasing with temperature
    ///     _100: ElectricalResistance::new::<ohm>(28.0),  // Highest resistance at max temp
    /// };
    /// 
    /// device.set_thermal_resistances(thermal_resistances).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # PCB Design Guidelines
    ///
    /// For optimal thermal performance:
    /// - Use large copper pours connected to the device's thermal pad
    /// - Implement via stitching to connect top and bottom copper layers
    /// - Consider thermal vias directly under the device
    /// - Ensure adequate airflow over the device area
    /// - Place temperature-sensitive components away from the AP33772S
    ///
    /// # Safety Considerations
    ///
    /// **Important**: Incorrect thermal resistance values can lead to:
    /// - Inadequate thermal protection (device overheating)
    /// - Premature derating (reduced performance)
    /// - Inaccurate temperature predictions
    ///
    /// Always measure or simulate your actual thermal performance!
    ///
    /// # See Also
    ///
    /// - [`get_thermal_resistances`] to read current values
    /// - [`set_thresholds`] to configure thermal protection temperatures
    /// - [`get_temperature`] to monitor junction temperature
    /// - [`get_operating_mode`] to check derating status
    ///
    /// [`ThermalResistances`]: crate::types::ThermalResistances
    /// [`ElectricalResistance`]: crate::types::units::ElectricalResistance
    /// [`get_thermal_resistances`]: crate::getters::Ap33772s::get_thermal_resistances
    /// [`set_thresholds`]: Self::set_thresholds
    /// [`get_temperature`]: crate::getters::Ap33772s::get_temperature
    /// [`get_operating_mode`]: crate::getters::Ap33772s::get_operating_mode
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

    /// Configures all protection threshold values for safe operation.
    ///
    /// Protection thresholds define the operating limits at which the AP33772S will take
    /// protective action to prevent damage from electrical or thermal stress. These must
    /// be carefully configured based on your system design and component ratings.
    ///
    /// # Parameters
    ///
    /// - `thresholds` - A [`Thresholds`] struct containing all protection limits:
    ///   - `over_voltage` - Maximum output voltage before shutdown [`ElectricPotential`]
    ///   - `over_current` - Maximum output current before shutdown [`ElectricCurrent`]
    ///   - `over_temperature` - Maximum junction temperature before shutdown [`ThermodynamicTemperature`]
    ///   - `under_voltage` - Minimum input/output voltage for operation [`ElectricPotential`]
    ///   - `derating` - Junction temperature at which current is reduced by 50% [`ThermodynamicTemperature`]
    ///
    /// # Protection Behaviors
    ///
    /// ## Over-Voltage Protection (OVP)
    /// - **Trigger**: Output voltage exceeds `over_voltage` threshold
    /// - **Action**: Immediate output shutdown via MOSFET switch
    /// - **Typical values**: 105-110% of maximum expected voltage
    ///
    /// ## Over-Current Protection (OCP)  
    /// - **Trigger**: Output current exceeds `over_current` threshold
    /// - **Action**: Immediate output shutdown
    /// - **Typical values**: 110-120% of maximum load current
    ///
    /// ## Over-Temperature Protection (OTP)
    /// - **Trigger**: Junction temperature exceeds `over_temperature` threshold  
    /// - **Action**: Immediate output shutdown
    /// - **Typical values**: 125-150°C (depends on thermal design)
    ///
    /// ## Under-Voltage Protection (UVP)
    /// - **Trigger**: Input or output voltage below `under_voltage` threshold
    /// - **Action**: Output shutdown or limited operation
    /// - **Typical values**: 85-90% of minimum operating voltage
    ///
    /// ## Thermal Derating
    /// - **Trigger**: Junction temperature exceeds `derating` threshold
    /// - **Action**: Current limit reduced by 50% (not shutdown)
    /// - **Typical values**: 75-85°C (well below OTP threshold)
    ///
    /// # Errors
    ///
    /// - [`Ap33772sError::I2c`] if any I2C communication fails
    /// - [`Ap33772sError::ConversionFailed`] if threshold values cannot be converted to device format
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ap33772s_rs::{Ap33772s, types::{Thresholds, units::*}};
    /// # async fn example(mut device: Ap33772s<impl embedded_hal::i2c::I2c, impl embedded_hal::delay::DelayNs>) -> Result<(), Box<dyn std::error::Error>> {
    /// // Conservative protection thresholds for a 20V/3A system
    /// let safety_thresholds = Thresholds {
    ///     over_voltage: ElectricPotential::new::<volt>(22.0),      // 110% of 20V max
    ///     over_current: ElectricCurrent::new::<ampere>(3.3),       // 110% of 3A max  
    ///     over_temperature: ThermodynamicTemperature::new::<degree_celsius>(125.0), // Conservative
    ///     under_voltage: ElectricPotential::new::<volt>(4.5),      // Below USB spec minimum
    ///     derating: ThermodynamicTemperature::new::<degree_celsius>(85.0),         // Start derating early
    /// };
    /// 
    /// device.set_thresholds(safety_thresholds).await?;
    /// 
    /// println!("Protection thresholds configured for safe operation");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Design Guidelines
    ///
    /// ## Voltage Thresholds
    /// - **OVP**: Set 5-10% above maximum expected voltage
    /// - **UVP**: Set 10-15% below minimum required voltage
    /// - Consider cable voltage drops and regulation tolerances
    ///
    /// ## Current Thresholds  
    /// - **OCP**: Set 10-20% above maximum load current
    /// - Account for inrush currents and transient loads
    /// - Consider connector and cable current ratings
    ///
    /// ## Temperature Thresholds
    /// - **OTP**: Typically 125-150°C for commercial operation
    /// - **Derating**: Set 20-40°C below OTP threshold
    /// - Account for thermal resistance and ambient temperature
    ///
    /// # Hardware Considerations
    ///
    /// Ensure your system design can handle the configured thresholds:
    /// - **PCB traces**: Rated for maximum current
    /// - **Connectors**: Rated for maximum voltage and current
    /// - **Capacitors**: Voltage rating above OVP threshold
    /// - **Thermal design**: Can dissipate heat at maximum power
    ///
    /// # Safety Warning
    ///
    /// **Critical**: Incorrect threshold settings can result in:
    /// - Component damage from overstress
    /// - Fire hazard from overheating
    /// - Unreliable operation from nuisance trips
    ///
    /// Always validate thresholds against your hardware specifications!
    ///
    /// # See Also
    ///
    /// - [`get_thresholds`] to read current threshold settings
    /// - [`set_thermal_resistances`] to configure thermal modeling
    /// - [`get_status`] to monitor protection events
    /// - [`get_temperature`] to monitor thermal conditions
    ///
    /// [`Thresholds`]: crate::types::Thresholds
    /// [`ElectricPotential`]: crate::types::units::ElectricPotential
    /// [`ElectricCurrent`]: crate::types::units::ElectricCurrent
    /// [`ThermodynamicTemperature`]: crate::types::units::ThermodynamicTemperature
    /// [`get_thresholds`]: crate::getters::Ap33772s::get_thresholds
    /// [`set_thermal_resistances`]: Self::set_thermal_resistances
    /// [`get_status`]: crate::getters::Ap33772s::get_status
    /// [`get_temperature`]: crate::getters::Ap33772s::get_temperature
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
