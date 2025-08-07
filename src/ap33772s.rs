use super::hal::*;
use crate::Ap33772sError;
use crate::commands::configuration::system_control::SystemControl;
use crate::commands::power_delivery::power_delivery_command_message::PowerDeliveryCommandMessage;

pub struct Ap33772s<I2C: I2c> {
    pub(crate) i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    /// The I2C address of the AP33772S device.
    /// This address is used for communication with the device over I2C.
    /// The address is defined in the AP33772S datasheet.
    pub const ADDRESS: SevenBitAddress = 0x52;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Result<Self, Ap33772sError> {
        let mut device = Self::new(i2c);
        device.is_device_present().await?;
        Ok(device)
    }

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

    #[maybe_async::maybe_async]
    pub async fn hard_reset(&mut self) -> Result<(), Ap33772sError> {
        let power_delivery_command_message = PowerDeliveryCommandMessage::builder()
            .with_HardResetEnable(true)
            .build();
        self.write_one_byte_command(power_delivery_command_message)
            .await?;
        Ok(())
    }
}
