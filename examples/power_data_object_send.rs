use ap33772s_rs::ap33772s::Ap33772s;
use ap33772s_rs::types::units::*;
use ap33772s_rs::types::{CURRENT_SELECTIONS, CurrentSelection, PowerDataObject, PowerType};
use utils::{setup_delay, setup_i2c};

fn main() {
    let i2c = setup_i2c(1_0000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new_default(i2c, delay).expect("Failed to setup AP33772S");

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
        .expect("Invalid Power Data Object Index");
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
    let response = ap33772s
        .negotiate_power_delivery(
            power_data_object_index,
            voltage,
            current_selection,
            &power_delivery_capabilities,
        )
        .expect("Failed to send Power Delivery Request");
    println!("Power Delivery request Response: {response:?}");

    // Read the Current Statistics
    let current_statistics = ap33772s
        .get_statistics()
        .expect("Failed to get Current Statistics");

    println!("Current Statistics: {current_statistics}");
}
