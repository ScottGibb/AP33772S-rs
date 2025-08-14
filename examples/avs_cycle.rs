use std::time::Duration;

use ap33772s_rs::{
    ap33772s::Ap33772s,
    types::{
        ap33772s_structures::{
            OperatingCurrentSelection, PowerDataObject, PowerType, VoltageOutputControl,
        },
        units::*,
    },
};
use utils::{setup_delay, setup_i2c};

/// The Power Data Object that supports AVS (Chosen due to Anker Power Bank supporting it on that configuration)
const PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT: PowerDataObject =
    PowerDataObject::StandardPowerRange6;

/// The following example shows how to cycle through the Adjustable Voltage Supply (AVS). It is inspired
/// by the Centy Labs example for the AP33772S. [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/AVScycle/AVScycle.ino)
/// The example steps up in 100mV increments and is designed to work with the [Anker Prime 27,650mAh Power Bank (250W)](https://www.amazon.co.uk/dp/B0BYP2F3SG?ref_=ppx_hzsearch_conn_dt_b_fed_asin_title_1&th=1)
fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new_default(i2c, delay).expect("Failed to initialise device");

    // Check if power Source supports AVS
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
    std::thread::sleep(Duration::from_secs(1));
    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");
    println!("Supported Power Data Objects: {power_data_objects}");
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
    std::thread::sleep(Duration::from_secs(1));

    println!("Turning MMOS Switch On");
    ap33772s
        .override_output_voltage(VoltageOutputControl::ForceOn)
        .expect("Failed to set MMOS Switch to Force On");

    loop {
        // Increase the voltage using the first extended power range data object
        for voltage in (6000..=28000).step_by(100) {
            // Send a power delivery request with the current voltage
            let response = ap33772s
                .negotiate_power_delivery(
                    PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT,
                    Some(ElectricPotential::new::<millivolt>(voltage as f32)),
                    OperatingCurrentSelection::Maximum,
                    &power_data_objects,
                )
                .expect("Failed to send power delivery request");
            println!("Power Delivery request Response: {response:?}");
            let stats = ap33772s.get_statistics().expect("Failed to get statistics");
            println!("Power Delivery Statistics: {stats}");

            // Wait for a while to observe the change
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
