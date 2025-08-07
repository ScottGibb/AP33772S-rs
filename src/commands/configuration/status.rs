use crate::commands::command_map::Command;
use crate::impl_one_byte_read_command;
use bitbybit::bitfield;

/// The host MCU, working as an I2C master, can access the status of the AP33772S through the STATUS register.
///  The STATUS register will be reset to 0 after a read operation.
///
/// Datasheet Name: STATUS
#[bitfield(u8, default = 0x00)]
#[derive(Debug, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Status {
    /// Detect if the System has started.
    /// Allow [System Configuration Register](crate::commands::configuration::protection_mode_configuration::ProtectionModeConfiguration)
    /// to be updated within 100ms
    ///
    /// Datasheet Name: STARTED_MSK
    #[bit(0, r)]
    pub started: bool,
    /// Ready to receive I2C request/command
    ///
    /// Datasheet Name: READY
    #[bit(1, r)]
    pub i2c_ready: bool,
    /// New source PDO(Power Data Object)s received
    ///
    /// Datasheet Name: NEWPDO
    #[bit(2, r)]
    pub new_power_data_object: bool,
    /// Under Voltage Protection Status
    ///
    /// Datasheet Name: UVP
    #[bit(3, r)]
    pub under_voltage_protection: bool,
    /// Over Voltage Protection Status
    ///
    /// Datasheet Name: OVP
    #[bit(4, r)]
    pub over_voltage_protection: bool,
    /// Over Current Protection Status
    ///
    /// Datasheet Name: OCP
    #[bit(5, r)]
    pub over_current_protection: bool,
    /// Over Temperature Protection Status
    ///
    /// Datasheet Name: OTP
    #[bit(6, r)]
    pub over_temperature_protection: bool,
    // /// Reserved Bit
    // #[bit(7, r)]
    // reserved: bool,
}
impl_one_byte_read_command!(Status, Command::Status);

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Status {{\n started: {}, \n i2c_ready: {}, \n new_power_data_object: {}, \n under_voltage_protection: {}, \n over_voltage_protection: {}, \n over_current_protection: {}, \n over_temperature_protection: {}\n }}",
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
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{\n started: {}, \n i2c_ready: {}, \n new_power_data_object: {}, \n under_voltage_protection: {}, \n over_voltage_protection: {}, \n over_current_protection: {}, \n over_temperature_protection: {}\n }}",
            self.started(),
            self.i2c_ready(),
            self.new_power_data_object(),
            self.under_voltage_protection(),
            self.over_voltage_protection(),
            self.over_current_protection(),
            self.over_temperature_protection()
        );
    }
}
