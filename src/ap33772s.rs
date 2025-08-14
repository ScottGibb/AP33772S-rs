//! This module outlines the AP33772S device. Specifically the top level methods and structure of the device.
//! The device can operate in multiple modes specifically in respect to using `interrupts` or using a delay
//! based approach. The default is to use a delay based approach which can be bypassed using the `advanced` feature
//! which allows for the entire commands to be exposed to the end user.
//!
#[cfg(not(feature = "interrupts"))]
use core::time::Duration;

use super::hal::*;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;

// Public API Types
use crate::types::command_structures::*;
use crate::types::units::*;
use crate::types::*;

/// Represents the AP33772S device.
/// It provides methods for interacting with the device over I2C.
/// See The [GitHub Repo](https://github.com/ScottGibb/AP33772S-rs) for examples on how to use the API.
pub struct Ap33772s<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> {
    pub(crate) i2c: I2C,
    /// The underlying delay mechanism required for the USB C Power Delivery negotiation
    #[cfg(not(feature = "interrupts"))]
    pub(crate) delay: D,
    /// The InputPin assigned for the Interrupt signal. This pin will go high when the AP33772S is ready for communication
    #[cfg(feature = "interrupts")]
    pub(crate) interrupt_pin: P,
}

/// This impl block represents the the initialisation methods for when no interrupts are used. This approach uses a
/// delay approach which is dependent on the users HAL
#[cfg(not(feature = "interrupts"))]
impl<I2C: I2c, D: DelayNs> Ap33772s<I2C, D> {
    const NEGOTIATE_TIMING_DELAY: Duration = Duration::from_millis(100);
    const BOOT_UP_DELAY: Duration = Duration::from_millis(100);
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    /// Creates a new instance of the AP33772S device. This Instance has no initialisation with the I2C bus.
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self { i2c, delay }
    }
    /// Creates a new instance of the AP33772S device, checks if the device is present on the bus and is in the correct state for first boot initialisation.
    /// Then sets the threshold values for the device using the default values
    ///
    /// IMPORTANT: This function must be called after the device is powered on, if it is called after the device is powered on, it will fail to initialise.
    /// The device may still work subsequently
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C, delay: D) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c, delay);
        device.is_device_present().await?;

        let device_status = device.get_status()?;
        if device_status.i2c_ready()
            && device_status.started()
            && device_status.new_power_data_object()
        {
            Self::initialise(&mut device).await?;
        } else {
            // Device May already be initialised, to do a fresh install, the user must fully power cycle the device
            device.hard_reset().await?; // This does not fully power cycle the RotoPD board due to the device being powered by the STEMMA connector
            Self::initialise(&mut device).await?;
            return Err(Ap33772sError::InitialisationFailure);
        }
        Ok(device)
    }
    #[maybe_async::maybe_async]
    async fn initialise(device: &mut Self) -> Result<(), Ap33772sError> {
        device.delay.delay_ms(
            u32::try_from(Self::BOOT_UP_DELAY.as_millis())
                .expect("This should not fail, HAL Duration Type Conversions"),
        ); // Initial delay to allow the device to power up
        device
            .set_thermal_resistances(ThermalResistances::default())?
            .await;
        device.set_thresholds(Thresholds::default()).await
    }

    /// This function negotiates power delivery with the connected device.
    /// It does include a delay in which the result will be read from the device. The delay is handled
    /// by the hal provided. If the user wishes to ignore this delay, they should use the
    /// driver in `advanced` mode by enabled the `advanced` feature.
    #[maybe_async::maybe_async]
    pub async fn negotiate_power_delivery(
        &mut self,
        power_data_object_index: PowerDataObject,
        voltage_selection: Option<ElectricPotential>,
        current_selection: OperatingCurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        // Check to see if PDO requested is available on the Source, return early if not
        if !data_objects
            .get_power_data_object(power_data_object_index)
            .is_detected()
        {
            return Err(Ap33772sError::PowerDataObjectNotDetected(
                power_data_object_index,
            ));
        }

        self.send_power_delivery_request(
            power_data_object_index,
            voltage_selection,
            current_selection,
            data_objects,
        )
        .await?;
        self.delay.delay_ms(
            u32::try_from(Self::NEGOTIATE_TIMING_DELAY.as_millis())
                .expect("This should not fail, HAL Duration Type Conversions"),
        );
        self.get_power_delivery_request_result().await
    }

    /// Performs a negotiation with the AP33772S device to change its current state to the configuration provided. Uses the `self.delay` to
    /// wait for the response. Wether the delay is blocking or not is dependent on HAL thats implements the `Delay` trait.
    #[maybe_async::maybe_async]
    pub async fn negotiate_maximum_power_delivery(
        &mut self,
        power_data_object_index: PowerDataObject,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        self.send_maximum_power_delivery_request(power_data_object_index)
            .await?;
        self.delay.delay_ms(
            u32::try_from(Self::NEGOTIATE_TIMING_DELAY.as_millis())
                .expect("This should not fail, HAL Duration Type Conversions"),
        );
        self.get_power_delivery_request_result().await
    }
}

#[cfg(feature = "interrupts")]
impl<I2C: I2c, D: DelayNs, P: InputPin> Ap33772s<I2C, D, P> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    /// Creates a new instance of the AP33772S device. This Instance has no initialisation with the I2C bus.
    pub fn new(i2c: I2C, delay: D, interrupt_pin: P) -> Self {
        todo!("Not implemented Yet");
        Self {
            i2c,
            delay,
            interrupt_pin,
        }
    }
    /// Creates a new instance of the AP33772S device and checks if the device is present on the bus.
    /// TODO: Integrate Setting of Thermal Resistance and Thresholds matching RotoPD Board. This also handles the timings required for initialisation by using the provided hals delay method
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C, delay: D, interrupt_pin: P) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c, delay, interrupt_pin);
        device.is_device_present().await?;
        // TODO: Initialize Thermal Resistances and Thresholds
        todo!("Not implemented Yet");
        Ok(device)
    }
}

impl<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> Ap33772s<I2C, D> {
    pub const ADDRESS: SevenBitAddress = 0x52;
    /// Checks if the device is present on the I2C bus. It checks an command register of the device and matches with the expected value.
    #[maybe_async::maybe_async]
    pub async fn is_device_present(&mut self) -> Result<(), Ap33772sError> {
        let system_control = self.read_one_byte_command::<SystemControl>().await?;
        system_control
            .command_version()
            .map_err(|raw_command_version| {
                Ap33772sError::WrongCommandVersion(raw_command_version)
            })?;
        Ok(())
    }

    /// Performs a hard reset on the device. This will completely reset the device and put it in a fresh state
    /// and require the I2C connection to be severed on the RotoPD board.
    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await
    }
}
