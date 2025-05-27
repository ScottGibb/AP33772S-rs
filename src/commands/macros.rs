#[macro_export]
macro_rules! impl_one_byte_write_command {
    ($type:ty, $address:expr) => {
        impl $crate::commands::traits::WriteOneByteCommand for $type {
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
        impl $crate::commands::traits::ReadOneByteCommand for $type {
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
        impl $crate::commands::traits::WriteTwoByteCommand for $type {
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
        impl $crate::commands::traits::ReadTwoByteCommand for $type {
            fn new_with_raw_value(raw_value: u16) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn command() -> Command {
                $address
            }
        }
    };
}
