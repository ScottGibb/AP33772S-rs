use crate::Ap33772sError;

use crate::commands::requested::current_requested::CurrentRequested;
use crate::commands::requested::voltage_requested::VoltageRequested;
use crate::commands::statistics::current::Current;
use crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::statistics::temperature::Temperature;
use crate::commands::statistics::voltage::Voltage;

use uom::si::f32::ElectricCurrent;
use uom::si::f32::ElectricPotential;
use uom::si::f32::ThermodynamicTemperature;

use super::hal::*;

pub struct Ap33772s<I2C: I2c> {
    pub(crate) i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    pub const ADDRESS: u8 = 0x52;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Self {
        Self::new(i2c)
    }

    #[maybe_async::maybe_async]
    pub async fn get_current(mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let current = self.read_one_byte_command::<Current>()?;
        Ok(current.current())
    }

    #[maybe_async::maybe_async]
    pub async fn get_voltage(mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage = self.read_one_byte_command::<Voltage>()?;
        Ok(voltage.voltage())
    }

    #[maybe_async::maybe_async]
    pub async fn get_temperature(mut self) -> Result<ThermodynamicTemperature, Ap33772sError> {
        let temperature = self.read_one_byte_command::<Temperature>()?;
        Ok(temperature.temperature())
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_voltage(mut self) -> Result<ElectricPotential, Ap33772sError> {
        let requested_voltage = self.read_two_byte_command::<VoltageRequested>()?;
        Ok(requested_voltage.voltage())
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_current(mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let requested_current = self.read_two_byte_command::<CurrentRequested>()?;
        Ok(requested_current.current())
    }
    #[maybe_async::maybe_async]
    pub async fn get_minimum_selection_voltage(
        mut self,
    ) -> Result<ElectricPotential, Ap33772sError> {
        let voltage_selection = self.read_one_byte_command::<MinimumSelectionVoltage>()?;
        Ok(voltage_selection.voltage())
    }
}
