use crate::commands::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// The AP33772S supports a level-triggered interrupt signal through the INT pin to the host MCU.
/// The [Interrupt Enable](crate::commands::configuration::interrupt_enable::InterruptEnable)
/// register defines the enable and disable of ON and OFF for various
/// [Status](crate::commands::configuration::status::Status)-defined events
///
/// Datasheet Name: MASK
#[bitfield(u8, default = 0x03, defmt_bitfields(feature = "defmt"))]
#[derive(Debug, PartialEq)]
pub struct InterruptEnable {
    /// Started Status
    ///
    /// Datasheet Name: STARTED_MSK
    #[bit(0, rw)]
    pub started: bool,
    /// I2C Ready
    ///
    /// Datasheet Name: READY
    #[bit(1, rw)]
    pub i2c_ready: bool,
    /// New Source PDOs
    ///
    /// Datasheet Name: NEWPDO
    #[bit(2, rw)]
    pub new_power_data_object: bool,
    /// Under Voltage Protection
    ///
    /// Datasheet Name: UVP
    #[bit(3, rw)]
    pub under_voltage_protection: bool,
    /// Over Voltage Protection
    ///
    /// Datasheet Name: OVP
    #[bit(4, rw)]
    pub over_voltage_protection: bool,
    /// Over Current Protection
    ///
    /// Datasheet Name: OCP
    #[bit(5, rw)]
    pub over_current_protection: bool,
    /// Over Temperature Protection
    ///
    /// Datasheet Name: OTP
    #[bit(6, rw)]
    pub over_temperature_protection: bool,
    /// Reserved Bit
    #[bit(7, rw)]
    reserved: bool,
}

impl_one_byte_write_command!(InterruptEnable, Command::InterruptEnableMask);
impl_one_byte_read_command!(InterruptEnable, Command::InterruptEnableMask);

impl core::fmt::Display for InterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "InterruptEnable {{\n\
             started: {},\n\
             i2c_ready: {},\n\
             new_power_data_object: {},\n\
             under_voltage_protection: {},\n\
             over_voltage_protection: {},\n\
             over_current_protection: {},\n\
             over_temperature_protection: {}\n\
             }}",
            self.started(),
            self.i2c_ready(),
            self.new_power_data_object(),
            self.under_voltage_protection(),
            self.over_voltage_protection(),
            self.over_current_protection(),
            self.over_temperature_protection()
        )
    }
}
