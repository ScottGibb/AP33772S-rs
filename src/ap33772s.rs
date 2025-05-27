use crate::commands::minimum_selection_voltage::MinimumSelectionVoltage;
use crate::commands::temperature::{Temperature};
use crate::commands::voltage::Voltage;
use crate::commands::current::Current;
use crate::{commands::command_map::Command, Ap33772sError};
use uom::si::electric_current::milliampere;
use uom::si::electric_potential::millivolt;
use uom::si::f32::ElectricCurrent;
use uom::si::f32::ElectricPotential;
use uom::si::f32::ThermodynamicTemperature;

use super::hal::*;

mod resolutions {
    pub const REQUESTED_VOLTAGE_RESOLUTION:u16 = 50; //mV
    pub const REQUEUSTED_CURRENT_RESOLUTION: u16 = 10; //mA
}
pub struct Ap33772s<I2C: I2c> {
    pub (crate) i2c: I2C,
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
        let write = [u8::from(Command::VoltageRequested)];
        let mut buf: [u8; 2] = [0,0];
        self.i2c.write_read(Self::ADDRESS, &write, &mut buf)?.await;
        let voltage_raw = f32::from(u16::from(buf[0]) * resolutions::REQUESTED_VOLTAGE_RESOLUTION);
        Ok(ElectricPotential::new::<millivolt>(voltage_raw))
    }
    #[maybe_async::maybe_async]
    pub async fn get_requested_current(mut self) -> Result<ElectricCurrent, Ap33772sError> {
        let write = [u8::from(Command::CurrentRequested)];
        let mut buf: [u8; 2] = [0,0];
        self.i2c.write_read(Self::ADDRESS, &write, &mut buf)?.await;
        let current_raw = f32::from(u16::from(buf[0]) * resolutions::REQUEUSTED_CURRENT_RESOLUTION);
        Ok(ElectricCurrent::new::<milliampere>(current_raw))
    }

    #[maybe_async::maybe_async]
    pub async fn get_minimum_selection_voltage(mut self) -> Result<ElectricPotential, Ap33772sError> {
        let voltage_selection = self.read_one_byte_command::<MinimumSelectionVoltage>()?; 
        Ok(voltage_selection.voltage())
    }


   
}
