use command_map::Command;

pub mod command_map;
pub mod interrupt_enable;
pub mod operation_mode;
pub mod power_delivery_configuration;
pub mod power_delivery_request_message;
pub mod protection_mode_configuration;
pub mod status;
pub mod source_power_data_object;
pub trait OneByteCommand {
    fn raw_value(&self) -> u8;
    fn new_with_raw_value(raw_value: u8) -> Self;
    fn get_command(&self) -> Command;
    fn command() -> Command;
}

pub trait TwoByteCommand {
    fn raw_value(&self) -> u16;
    fn new_with_raw_value(raw_value: u16) -> Self;
    fn get_command(&self) -> Command;
    fn command() -> Command;
}

#[macro_export]
macro_rules! impl_register {
    ($type:ty, $address:expr) => {
        impl $crate::commands::OneByteCommand for $type {
            fn raw_value(&self) -> u8 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u8) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_command(&self) -> Command {
                $address
            }
            fn command() -> Command {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dual_register {
    ($type:ty, $address:expr) => {
        impl $crate::commands::TwoByteCommand for $type {
            fn raw_value(&self) -> u16 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u16) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_command(&self) -> Command {
                $address
            }
             fn command() -> Command {
                $address
            }
        }
    };
}
