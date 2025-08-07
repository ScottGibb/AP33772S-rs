use crate::commands::command_map::Command;
use crate::{impl_one_byte_read_command, impl_one_byte_write_command};
use bitbybit::bitfield;

/// The AP33772S supports a Protection Mode Configuration register that defines the
/// system configuration options that enable specific modules.
///
/// Datasheet Name: CONFIG
#[bitfield(u8, default = 0xF8)]
#[derive(Debug, PartialEq)]
pub struct ProtectionModeConfiguration {
    // /// Reserved
    // #[bits(0..=2, rw)]
    // reserved: u3,
    /// Under Voltage Protection Enabled
    ///
    /// Datasheet Name: UVP_EN
    #[bit(3, rw)]
    under_voltage_protection_enabled: bool,
    /// Over Voltage Protection Enabled
    ///
    /// Datasheet Name: OVP_EN
    #[bit(4, rw)]
    over_voltage_protection_enabled: bool,
    /// Over Current Protection Enabled
    ///
    /// Datasheet Name: OCP_EN
    #[bit(5, rw)]
    over_current_protection_enabled: bool,
    /// Over Temperature Protection Enabled
    ///
    /// Datasheet Name: OTP_EN
    #[bit(6, rw)]
    over_temperature_protection_enabled: bool,
    /// Derating Function Enabled
    ///
    /// Datasheet Name: DR_EN
    #[bit(7, rw)]
    derating_function_enabled: bool,
}

impl_one_byte_write_command!(ProtectionModeConfiguration, Command::SystemConfiguration);
impl_one_byte_read_command!(ProtectionModeConfiguration, Command::SystemConfiguration);

impl core::fmt::Display for ProtectionModeConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ProtectionModeConfiguration {{\n\
             under_voltage_protection_enabled: {},\n\
             over_voltage_protection_enabled: {},\n\
             over_current_protection_enabled: {},\n\
             over_temperature_protection_enabled: {},\n\
             derating_function_enabled: {}\n\
             }}",
            self.under_voltage_protection_enabled(),
            self.over_voltage_protection_enabled(),
            self.over_current_protection_enabled(),
            self.over_temperature_protection_enabled(),
            self.derating_function_enabled()
        )
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProtectionModeConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ProtectionModeConfiguration {{\n
             under_voltage_protection_enabled: {},\n\
             over_voltage_protection_enabled: {},\n\
             over_current_protection_enabled: {},\n\
             over_temperature_protection_enabled: {},\n\
             derating_function_enabled: {}\n\
             }}",
            self.under_voltage_protection_enabled(),
            self.over_voltage_protection_enabled(),
            self.over_current_protection_enabled(),
            self.over_temperature_protection_enabled(),
            self.derating_function_enabled()
        );
    }
}
