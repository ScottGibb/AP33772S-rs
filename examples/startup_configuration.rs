#[cfg_attr(docsrs, doc(cfg(feature = "advanced")))]
#[cfg(feature = "advanced")]
mod imports {
    pub use ap33772s_rs::ap33772s::Ap33772s;
    pub use ap33772s_rs::commands::{
        configuration::power_delivery_configuration::PowerDeliveryConfiguration,
        thresholds::{
            de_rating_threshold::DeRatingThreshold,
            over_current_protection_threshold::OverCurrentProtectionThreshold,
            over_temperature_protection_threshold::OverTemperatureProtectionThreshold,
            over_voltage_protection_threshold::OverVoltageProtectionThreshold,
            under_voltage_protection_threshold::UnderVoltageProtectionThreshold,
        },
    };
    pub use ap33772s_rs::commands::{
        configuration::{
            interrupt_enable::InterruptEnable,
            protection_mode_configuration::ProtectionModeConfiguration,
        },
        statistics::minimum_selection_voltage::MinimumSelectionVoltage,
        thresholds::vdc_threshold::VDCTHR,
    };

    pub use utils::setup_i2c;
}
#[cfg(not(feature = "advanced"))]
mod imports {}

#[allow(unused_imports)]
// Added here as Advanced feature is optional, Clippy complains otherwise
use imports::*;

/// This example demonstrates how to configure the AP33772S chip
/// for startup settings, including enabling interrupts and setting thresholds. This is done using the `advanced` feature in which you can read and write from the
/// registers directly as such in order to gain access to the advanced features you need to enable the `advanced` feature in your `Cargo.toml` file
#[cfg(feature = "advanced")]
fn main() {
    use utils::setup_delay;

    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new(i2c, delay);

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    if status.started() && status.i2c_ready() {
        println!(" AP33772S Is up and running!")
    } else {
        println!("Status: {}", status);
        println!("AP33772S is not ready..., but I2C communication is working");
    }

    println!("Apply Startup Settings:");

    // Enable Interrupt
    let interrupts = InterruptEnable::new_with_raw_value(0x0F);
    println!("Enabling Interrupts: {}", interrupts);
    ap33772s
        .write_one_byte_command(interrupts)
        .expect("Should not fail");

    // System Configuration
    let protection_mode_configuration = ProtectionModeConfiguration::new_with_raw_value(0xFC);
    println!(
        "Setting Protection Mode Configuration: {}",
        protection_mode_configuration
    );
    ap33772s
        .write_one_byte_command(protection_mode_configuration)
        .expect("Should not fail");

    // Power Delivery Configuration
    let power_delivery_configuration = PowerDeliveryConfiguration::new_with_raw_value(0x03);
    println!(
        "Setting Power Delivery Configuration: {}",
        power_delivery_configuration
    );
    ap33772s
        .write_one_byte_command(power_delivery_configuration)
        .expect("Should not fail");

    // Minimum Selection Voltage
    let minimum_selection_voltage = MinimumSelectionVoltage::new_with_raw_value(0x16);
    println!(
        "Setting Minimum Selection Voltage: {:?}",
        minimum_selection_voltage
    );
    // Under Voltage Protection Threshold
    ap33772s
        .write_one_byte_command(minimum_selection_voltage)
        .expect("This should not fail");
    let under_voltage_protection_threshold =
        UnderVoltageProtectionThreshold::new_with_raw_value(0x01);
    println!(
        "Setting Under Voltage Protection Threshold: {:?}",
        under_voltage_protection_threshold
    );
    ap33772s
        .write_one_byte_command(under_voltage_protection_threshold)
        .expect("This should not fail");
    // Over Voltage Protection Threshold
    let over_voltage_protection_threshold =
        OverVoltageProtectionThreshold::new_with_raw_value(0x19);
    println!(
        "Setting Over Voltage Protection Threshold: {:?}",
        over_voltage_protection_threshold
    );
    ap33772s
        .write_one_byte_command(over_voltage_protection_threshold)
        .expect("This should not fail");
    // Over current Protection Threshold
    let over_current_protection_threshold =
        OverCurrentProtectionThreshold::new_with_raw_value(0x00);
    println!(
        "Setting Over Current Protection Threshold: {:?}",
        over_current_protection_threshold
    );
    ap33772s
        .write_one_byte_command(over_current_protection_threshold)
        .expect("This should not fail");

    // Over temperature Protection Threshold
    let over_temperature_protection_threshold =
        OverTemperatureProtectionThreshold::new_with_raw_value(0x70);
    println!(
        "Setting Over Temperature Protection Threshold: {:?}",
        over_temperature_protection_threshold
    );
    ap33772s
        .write_one_byte_command(over_temperature_protection_threshold)
        .expect("This should not fail");

    // Derating Threshold
    let de_rating_threshold = DeRatingThreshold::new_with_raw_value(0x70);
    println!("Setting De-Rating Threshold: {:?}", de_rating_threshold);
    ap33772s
        .write_one_byte_command(de_rating_threshold)
        .expect("This should not fail");

    // VDC Threshold
    let vdc_threshold = VDCTHR::new_with_raw_value(0x00);
    println!("Setting VDC Threshold: {:?}", vdc_threshold);
    ap33772s
        .write_one_byte_command(vdc_threshold)
        .expect("This should not fail");

    // Read Status

    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status after configuration: {}", status);
}

#[cfg(not(feature = "advanced"))]
fn main() {
    println!("This example requires the 'advanced' feature to be enabled.");
}
