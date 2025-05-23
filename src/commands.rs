use command_map::Command;

pub mod command_map;
pub mod interrupt_enable;
pub mod operation_mode;
pub mod power_delivery_configuration;
pub mod power_delivery_request_message;
pub mod protection_mode_configuration;
pub mod all_source_power_data_object;
pub mod status;
pub mod system_control;
pub mod source_power_data_object;
pub mod extended_power_range_data_object;
pub mod power_delivery_command_message;
pub mod power_delivery_message_result;
pub mod minimum_selection_voltage;
pub mod voltage;
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
#[macro_export]
macro_rules! impl_one_byte_write_command {
    ($type:ty, $address:expr) => {
        impl $crate::commands::WriteOneByteCommand for $type {
            fn raw_value(&self) -> u8 {
                self.raw_value()
            }

            fn get_command(&self) -> Command {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_one_byte_read_command {
    ($type:ty, $address:expr) => {
        impl $crate::commands::ReadOneByteCommand for $type {
            fn new_with_raw_value(raw_value: u8) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn command() -> Command {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_write_two_byte_command {
    ($type:ty, $address:expr) => {
        impl $crate::commands::WriteTwoByteCommand for $type {
            fn raw_value(&self) -> u16 {
                self.raw_value()
            }
            fn get_command(&self) -> Command {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_two_byte_read_command {
    ($type:ty, $address:expr) => {
        impl $crate::commands::ReadTwoByteCommand for $type {
            fn new_with_raw_value(raw_value: u16) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn command() -> Command {
                $address
            }
        }
    };
}
