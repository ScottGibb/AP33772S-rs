use ap33772s_rs::{
    ap33772s::Ap33772s,
    types::api_commands::{OperatingCurrentSelection, PowerDataObject},
};
use utils::{setup_delay, setup_i2c};

/// This example is Designed to work with the Macbook Pro M4 Charger - it may also work on others
fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {status}");

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {power_delivery_capabilities}");

    let index = PowerDataObject::StandardPowerRange2;
    let current_selection = OperatingCurrentSelection::_3A;
    println!("Switching to {index} with current selection {current_selection}");
    let response = ap33772s
        .negotiate_power_delivery(index, None, current_selection, &power_delivery_capabilities)
        .expect("Failed to send power delivery request");
    println!("Power Delivery request Response: {response:?}");
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {stats}");

    // Pause Between Switching
    std::thread::sleep(std::time::Duration::from_secs(2));

    let index = PowerDataObject::StandardPowerRange4;
    let current_selection = OperatingCurrentSelection::Maximum;
    let voltage_selection = None; // No voltage selection needed for fixed PDOs
    println!("Switching to {index} with current selection {current_selection}");
    let response = ap33772s
        .negotiate_power_delivery(
            index,
            voltage_selection,
            current_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send power delivery request");
    println!("Power Delivery request Response: {response:?}");
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {stats}");
}
