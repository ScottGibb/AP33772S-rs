use super::hal::*;
use crate::{
    Ap33772sError,
    ap33772s::Ap33772s,
    commands::traits::{ReadOneByteCommand, WriteOneByteCommand},
};
impl<I2C: I2c> Ap33772s<I2C> {
    #[maybe_async::maybe_async]
    pub(crate) async fn write_one_byte_command(
        &mut self,
        command: impl WriteOneByteCommand,
    ) -> Result<(), Ap33772sError> {
        let command_address = u8::from(command.get_command());
        let data = command.raw_value();
        self.i2c
            .write(Self::ADDRESS, &[command_address, data])?
            .await;
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub(crate) async fn read_one_byte_command<CommandRegister>(
        &mut self,
    ) -> Result<CommandRegister, Ap33772sError>
    where
        CommandRegister: ReadOneByteCommand,
    {
        let mut data: [u8; 1] = [0x00];
        let command_address = u8::from(CommandRegister::command());
        self.i2c
            .write_read(Self::ADDRESS, &[command_address], &mut data)?
            .await;
        Ok(CommandRegister::new_with_raw_value(data[0]))
    }
}
