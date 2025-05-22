use crate::{ap33772s::Ap33772s, commands::{ CommandRegister}, Ap33772sError};
use super::hal::*;
impl<I2C: I2c> Ap33772s<I2C> {

    #[maybe_async::maybe_async]
    pub async fn set_config_register(&mut self, register: impl CommandRegister) -> Result<(), Ap33772sError>
    {
        let register_address = u8::from(register.get_command());
        let data = register.raw_value();
        self.i2c.write(Self::ADDRESS, &[register_address, data])?.await;
        Ok(())
    }


}