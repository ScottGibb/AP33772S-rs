use ap33772s_rs::ap33772s::{Ap33772s, CurrentSelection, PowerDataObject};
use utils::setup_i2c;

fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).unwrap();

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {:?}", power_delivery_capabilities);

    // Switch Between PDO Indexexes 1 and 2
    println!("Switching to Power Data Object Index 1 with 5A or more current selection...");
    let index = PowerDataObject::StandardPowerRange1;
    let current_selection = CurrentSelection::_5AOrMore;
    let voltage_selection = None; // No voltage selection needed for fixed PDOs
    ap33772s
        .send_power_delivery_request(
            index,
            current_selection,
            voltage_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send power delivery request");

    println!(
        "Switched to Power Data Object Index {:?} with current selection {:?}",
        index, current_selection
    );
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {:?}", stats);
    println!("Switching to Power Data Object Index 2 with 3A current selection...");
    let index = PowerDataObject::StandardPowerRange2;
    let current_selection = CurrentSelection::_3A;
    let voltage_selection = None; // No voltage selection needed for fixed PDOs
    ap33772s
        .send_power_delivery_request(
            index,
            current_selection,
            voltage_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send power delivery request");
    println!(
        "Switched to Power Data Object Index {:?} with current selection {:?}",
        index, current_selection
    );
    let stats = ap33772s.get_statistics().expect("Failed to get statistics");
    println!("Statistics: {:?}", stats);
}
