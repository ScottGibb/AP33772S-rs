use crate::{
    commands::command_map::Command, impl_one_byte_read_command, impl_one_byte_write_command,
};
use bitbybit::bitfield;

/// The AP33772S supports a Power Delivery Configuration register that defines the
/// system configuration options that enable specific modules.
///
/// Datasheet Name: PDCONFIG
#[bitfield(u8, default = 0x03)]
#[derive(Debug, PartialEq)]
pub struct PowerDeliveryConfiguration {
    /// Extended Power Delivery Enabled
    ///
    /// Datasheet Name: EPR_MODE_EN
    #[bit(0, rw)]
    extended_power_delivery_enabled: bool,
    /// Programmable Power Delivery and Adjustable Power Supply Enabled
    ///
    /// Datasheet Name: PPS_AVS_EN
    #[bit(1, rw)]
    programmable_power_supply_and_adjustable_power_supply_enabled: bool,
    // /// Reserved bit
    // #[bit(2, rw)]
    // reserved: u1,
    // /// Reserved bit
    // #[bit(3, rw)]
    // reserved2: u1,
    // /// Reserved bits
    // #[bits(4..=7, rw)]
    // reserved3: u4,
}

impl_one_byte_write_command!(
    PowerDeliveryConfiguration,
    Command::PowerDeliveryConfiguration
);
impl_one_byte_read_command!(
    PowerDeliveryConfiguration,
    Command::PowerDeliveryConfiguration
);

impl core::fmt::Display for PowerDeliveryConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PowerDeliveryConfiguration {{\n\
             extended_power_delivery_enabled: {},\n\
             programmable_power_delivery_and_adjustable_power_supply_enabled: {}\n\
             }}",
            self.extended_power_delivery_enabled(),
            self.programmable_power_supply_and_adjustable_power_supply_enabled()
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PowerDeliveryConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PowerDeliveryConfiguration {{\n\
             extended_power_delivery_enabled: {},\n\
             programmable_power_delivery_and_adjustable_power_supply_enabled: {}\n\
             }}",
            self.extended_power_delivery_enabled(),
            self.programmable_power_delivery_and_adjustable_power_supply_enabled()
        );
    }
}
