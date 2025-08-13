use ap33772s_rs::{
    ap33772s::Ap33772s,
    types::{
        api_commands::{CurrentSelection, PowerDataObject, PowerType, VoltageOutputControl},
        units::*,
    },
};
/// The following example shows how to cycle through the Adjustable Voltage Supply (AVS). It is inspired
/// by the Centy Labs example for the AP33772S. [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/AVScycle/AVScycle.ino)
use utils::{setup_delay, setup_i2c};

/// Choose your PPS compatible Power Data Object
const PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT: PowerDataObject =
    PowerDataObject::ExtendedPowerRange8;

fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new_default(i2c, delay).expect("Failed to initialise device");

    let operation_capability = ap33772s
        .get_power_delivery_configuration()
        .expect("Failed to get power delivery configuration");

    match operation_capability.programmable_power_supply_adjustable_voltage_supply_enabled {
        true => {
            println!("Programmable Power Supply (PPS) Adjustable Voltage Supply (AVS) is enabled");
        }
        false => {
            println!("Programmable Power Supply (PPS) Adjustable Voltage Supply (AVS) is disabled");
            panic!(
                "AVS is not enabled on this device, please enable it in the Power Delivery Configuration or find a compatible device"
            );
        }
    }
    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");

    // Check if PPS is supported on the chosen Power Data Object
    let power_type = power_data_objects
        .get_power_data_object(PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT)
        .source_power_type();
    match power_type {
        PowerType::Adjustable => {
            println!("The Power Data Object is Adjustable, proceeding with AVS cycle");
        }
        PowerType::Fixed => {
            println!("The Power Data Object is Fixed, cannot proceed with AVS cycle");
            panic!("AVS is not supported on this Power Data Object");
        }
    }

    loop {
        // Set the MMOS Switch
        ap33772s
            .override_output_voltage(VoltageOutputControl::ForceOn)
            .expect("Failed to set MMOS Switch to Force On");

        // Increase the voltage using the first extended power range data object
        for voltage in (15000..=30000).step_by(1000) {
            // Send a power delivery request with the current voltage
            let response = ap33772s
                .negotiate_power_delivery(
                    PowerDataObject::ExtendedPowerRange8,
                    Some(ElectricPotential::new::<millivolt>(voltage as f32)),
                    CurrentSelection::_3A,
                    &power_data_objects,
                )
                .expect("Failed to send power delivery request");
            println!("Power Delivery request Response: {response:?}");

            // Wait for a while to observe the change
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
