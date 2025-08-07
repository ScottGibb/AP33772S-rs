use ap33772s_rs::ap33772s::Ap33772s;
use ap33772s_rs::types::units::*;
use ap33772s_rs::types::{CURRENT_SELECTIONS, CurrentSelection, PowerDataObject, PowerType};
use utils::setup_i2c;

fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).unwrap();

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {status}");

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {power_delivery_capabilities}");

    // Select the Power Data Object Index
    println!("Enter Power Object Index (1-13):");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let power_data_object_index: u8 = input.trim().parse().expect("Invalid input");

    if !(1..=13).contains(&power_data_object_index) {
        panic!("Power Delivery Index must be between 1 and 13");
    }
    let power_data_object_index = PowerDataObject::try_from(power_data_object_index - 1)
        .expect("The Power Data Object Index must be between 1 and 13");
    println!(" Power Data Object Index: {power_data_object_index}");

    // Check of the Power Data Object Index is not a fixed type //TODO: Replace this
    let fixed = if power_data_object_index >= PowerDataObject::ExtendedPowerRange9 {
        let index = usize::from(u8::from(
            power_data_object_index.raw_value() - PowerDataObject::ExtendedPowerRange9.raw_value(),
        ));
        power_delivery_capabilities.extended_power[index].source_power_type() == PowerType::Fixed
    } else {
        let index: usize = usize::from(u8::from(power_data_object_index.raw_value()));
        power_delivery_capabilities.source_power[index].source_power_type() == PowerType::Fixed
    };

    let voltage = if !fixed {
        println!(
            "The power data object is not a fixed type, please enter the voltage in milivolts (mV)"
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let raw_voltage: f32 = input.trim().parse().expect("Invalid input");
        Some(ElectricPotential::new::<millivolt>(raw_voltage))
    } else {
        None
    };

    println!("Enter the Current you wish to select");
    println!("{CURRENT_SELECTIONS:?}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let current_selection: u8 = input.trim().parse().expect("Invalid input");
    let current_selection = CurrentSelection::try_from(current_selection)
        .expect("Invalid Current Selection, must be between 0 and 7");

    println!("Current Selected: {current_selection}");

    // Request the Power Delivery
    ap33772s
        .send_power_delivery_request(
            power_data_object_index,
            voltage,
            current_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send Power Delivery Request");

    // Read the Current Statistics
    let current_statistics = ap33772s
        .get_statistics()
        .expect("Failed to get Current Statistics");

    println!("Current Statistics: {current_statistics}");
}
