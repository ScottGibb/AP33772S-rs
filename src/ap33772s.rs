use super::hal::*;
use uom::si::f32::Power;
use uom::si::f32::ThermodynamicTemperature;
use uom::si::f32::ElectricPotential;
use uom::si::f32::ElectricCurrent;


#[derive(Debug)]
pub struct AP33772SStatistics{
    pub current: ElectricCurrent,
    pub voltage: ElectricPotential,
    pub power: Power,
    pub temperature: ThermodynamicTemperature,

    pub requested_voltage: ElectricPotential,
    pub requested_current: ElectricCurrent,
    pub requested_power: Power,
}
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
}
