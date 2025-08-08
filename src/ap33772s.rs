//! This module outlines the AP33772S device. Specifically the top level methods and structure of the device.
//!
use super::hal::*;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;
use crate::error::Ap33772sError;
use crate::types::units::*;
use crate::types::{
    AllSourceDataPowerDataObject, CurrentSelection, PowerDataObject, PowerDeliveryResponse,
    ThermalResistances, Thresholds,
};

/// Represents the AP33772S device.
/// It provides methods for interacting with the device over I2C.
/// See The [GitHub Repo](https://github.com/ScottGibb/AP33772S-rs) for examples on how to use the API.
pub struct Ap33772s<I2C: I2c, D: DelayNs, #[cfg(feature = "interrupts")] P: InputPin> {
    pub(crate) i2c: I2C,
    pub(crate) delay: D,
    #[cfg(feature = "interrupts")]
    pub(crate) interrupt_pin: P,
}

/// This impl block represents the the initialisation methods for when no interrupts are used. This approach uses a
/// delay approach which is dependent on the users HAL
#[cfg(not(feature = "interrupts"))]
impl<I2C: I2c, D: DelayNs> Ap33772s<I2C, D> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    /// Creates a new instance of the AP33772S device. This Instance has no initialisation with the I2C bus.
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self { i2c, delay }
    }
    /// Creates a new instance of the AP33772S device and checks if the device is present on the bus.
    /// TODO: Integrate Setting of Thermal Resistance and Thresholds matching RotoPD Board. This also handles the timings required for initialisation by using the provided hals delay method
    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C, delay: D) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c, delay);
        device.is_device_present().await?;

        // Check if device has started
        if device.get_status()?.started() {
            device.delay.delay_ms(100); // Initial delay to allow the device to power up
            device
                .set_thermal_resistances(ThermalResistances::default())
                .await?;
            device.set_thresholds(Thresholds::default()).await?;
        } else {
            return Err(Ap33772sError::InitialisationFailure);
        }
        Ok(device)
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

    /// Performs a hard reset on the device.
    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await
    }

    /// This function negotiates power delivery with the connected device.
    /// It does include a delay in which the result will be read from the device. The dalay is handled
    /// by the hal provided. If the user wishes to ignore this delay, they should use the
    /// driver in `advanced` mode by enabled the `advanced` feature.
    #[maybe_async::maybe_async]
    pub async fn negotiate_power_delivery(
        &mut self,
        power_data_object_index: PowerDataObject,
        voltage_selection: Option<ElectricPotential>,
        current_selection: CurrentSelection,
        data_objects: &AllSourceDataPowerDataObject,
    ) -> Result<PowerDeliveryResponse, Ap33772sError> {
        self.send_power_delivery_request(
            power_data_object_index,
            voltage_selection,
            current_selection,
            data_objects,
        )
        .await?;
        self.delay.delay_ms(3); // Value chosen from the [Datasheet](../docs/AP33772S-Raspberry-Pi-I2C-User-Guide.pdf) 
        self.get_power_delivery_request_result().await
    }
}
