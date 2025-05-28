use crate::commands::command_map::Command;

pub trait WriteOneByteCommand {
    fn raw_value(&self) -> u8;
    fn get_command(&self) -> Command;
}

pub trait ReadOneByteCommand {
    fn new_with_raw_value(raw_value: u8) -> Self;
    fn command() -> Command;
}
pub trait WriteTwoByteCommand {
    fn raw_value(&self) -> u16;
    fn get_command(&self) -> Command;
}

pub trait ReadTwoByteCommand {
    fn new_with_raw_value(raw_value: u16) -> Self;
    fn command() -> Command;
}
