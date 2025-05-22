// Hal imports
#[cfg(feature = "sync")]
mod hal {
    pub use embedded_hal::i2c::I2c;
}

#[cfg(feature = "async")]
mod hal {
    pub use embedded_hal_async::i2c::I2c;
}

use hal::*;


pub struct Ap33772s<I2C: I2c> {
    i2c: I2C,
}

impl<I2C: I2c> Ap33772s<I2C> {
    pub const ADDRESS: u8 = 0x52;

    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[maybe_async::maybe_async]
    pub async fn new_default(i2c: I2C) -> Self
    where
    {
        let driver = Self::new(i2c);
        driver
    }
}