use super::hal::*;
use crate::{
    ap33772s::Ap33772s, commands::{OneByteCommand, TwoByteCommand}, Ap33772sError
};
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub async fn set_command(&mut self, command: impl OneByteCommand) -> Result<(), Ap33772sError> {
        let command_address = u8::from(command.get_command());
        let data = command.raw_value();
        self.i2c
            .write(Self::ADDRESS, &[command_address, data])?
            .await;
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub async fn get_command<CommandRegister>(&mut self) -> Result<CommandRegister, Ap33772sError>
    where
        CommandRegister: OneByteCommand,
    {
        let mut data: [u8; 1] = [0x00];
        let command_address = u8::from(CommandRegister::command());
        self.i2c
            .write_read(Self::ADDRESS, &[command_address], &mut data)?
            .await;
        Ok(CommandRegister::new_with_raw_value(data[0]))
    }

     #[maybe_async::maybe_async]
    pub async fn get_dual_command<CommandRegister>(&mut self) -> Result<CommandRegister, Ap33772sError>
    where
        CommandRegister: TwoByteCommand,
    {
        let mut data: [u8; 2] = [0x00, 0x00];
        let command_address = u8::from(CommandRegister::command());
        self.i2c
            .write_read(Self::ADDRESS, &[command_address], &mut data)?
            .await;
        Ok(CommandRegister::new_with_raw_value(u16::from_le_bytes(data)))
    }
}
