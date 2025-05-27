use command_map::Command;

pub mod command_map;
pub mod current;
pub mod interrupt_enable;
pub mod minimum_selection_voltage;
pub mod operation_mode;
pub mod power_delivery;
pub mod data_objects;
pub mod protection_mode_configuration;
pub mod status;
pub mod system_control;
pub mod temperature;
pub mod thermal_resistances;
pub mod voltage;
pub mod requested;
pub mod thresholds;
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
macro_rules! impl_two_byte_write_command {
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
