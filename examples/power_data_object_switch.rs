use ap33772s_rs::ap33772s::Ap33772s;
use ap33772s_rs::types::{CurrentSelection, PowerDataObject};
use utils::{setup_delay, setup_i2c};

fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new_default(i2c, delay).unwrap();

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {status}");

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {power_delivery_capabilities}");

    // Switch Between PDO Indexexes 1 and 2
    println!("Switching to Power Data Object Index 1 with 5A or more current selection...");
    let index = PowerDataObject::StandardPowerRange1;
    let current_selection = CurrentSelection::_5AOrMore;
    let voltage_selection = None; // No voltage selection needed for fixed PDOs
    let response = ap33772s
        .negotiate_power_delivery(
            index,
            voltage_selection,
            current_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send power delivery request");
    println!("Power Delivery request Response: {response:?}");

    println!(
        "Switched to Power Data Object Index {index} with current selection {current_selection}"
    );
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {stats}");
    println!("Switching to Power Data Object Index 2 with 3A current selection...");
    let index = PowerDataObject::StandardPowerRange2;
    let current_selection = CurrentSelection::_3A;
    let voltage_selection = None; // No voltage selection needed for fixed PDOs
    let response = ap33772s
        .negotiate_power_delivery(
            index,
            voltage_selection,
            current_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send power delivery request");
    println!("Power Delivery request Response: {response:?}");

    println!(
        "Switched to Power Data Object Index {index} with current selection {current_selection}"
    );
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {stats}");
}
