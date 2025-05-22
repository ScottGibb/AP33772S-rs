use command_map::Command;
mod command_map;
mod interrupt_enable;
mod status;
mod operation_mode;
mod protection_mode_configuration;
mod power_delivery_configuration;
mod power_delivery_request_message;
pub trait CommandRegister {
    fn raw_value(&self) -> u8;
    fn new_with_raw_value(raw_value: u8) -> Self;
    fn get_command() -> Command;
}

pub trait TwoByteCommandRegister {
    fn raw_value(&self) -> u16;
    fn new_with_raw_value(raw_value: u16) -> Self;
    fn get_command() -> Command;
}

#[macro_export]
macro_rules! impl_register {
    ($type:ty, $address:expr) => {
        impl $crate::commands::CommandRegister for $type {
            fn raw_value(&self) -> u8 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u8) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_command() -> Command {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dual_register {
    ($type:ty, $address:expr) => {
        impl $crate::commands::TwoByteCommandRegister for $type {
            fn raw_value(&self) -> u16 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u16) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_command() -> Command {
                $address
            }
        }
    };
}