use ap33772s_rs::ap33772s::Ap33772s;
use ap33772s_rs::types::{CURRENT_SELECTIONS, CurrentSelection, PowerDataObject, PowerType};
use ap33772s_rs::types::{Status, units::*};
use utils::{setup_delay, setup_i2c};

fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = Ap33772s::new_default(i2c, delay).expect("Failed to setup AP33772S");

    let status = ap33772s.get_status().expect("Failed to get status");
    assert_eq!(status, Status::RESET); // This should be equal as Status Register should of been read and the device is waiting for the next PD configuration

    loop {
        // Get the Power Source Delivery Capabilities
        let power_delivery_capabilities = ap33772s
            .get_all_source_power_capabilities()
            .expect("Failed to get Power Source Delivery Capabilities");
        println!("Capabilities: {power_delivery_capabilities}");

        // Ask the User for the PDO Index they want to select
        let power_data_object_index = get_power_data_object_index_from_user();
        let power_data_object =
            power_delivery_capabilities.get_power_data_object(&power_data_object_index);
        // Check of the Power Data Object Index is not a fixed type
        let power_type = power_data_object.get_power_type();

        // If the Power Data Object is Adjustable, get the chosen voltage
        let voltage = if power_type == PowerType::Adjustable {
            Some(get_chosen_voltage_from_user())
        } else {
            None
        };

        // Get Current Selection from the user
        let current_selection = get_current_selection_from_user();
        println!("Selected PDO: {power_data_object_index}, Voltage: {voltage:?}, Current: {current_selection} with
        Power Data Object: {power_data_object}
        ");
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

        // Read the System Statistics
        let system_statistics = ap33772s
            .get_statistics()
            .expect("Failed to get System Statistics");
        println!("System Statistics: {system_statistics}");
    }
}

fn get_power_data_object_index_from_user() -> PowerDataObject {
    println!("Enter Power Object Index (0-12):");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let power_data_object_index: u8 = input.trim().parse().expect("Invalid input");
    if !(0..=12).contains(&power_data_object_index) {
        panic!("Power Delivery Index must be between 0 and 12");
    }

    let power_data_object_index = PowerDataObject::try_from(power_data_object_index)
        .expect("Invalid Power Data Object Index");
    println!(" Power Data Object Index: {power_data_object_index}");
    power_data_object_index
}

fn get_chosen_voltage_from_user() -> ElectricPotential {
    println!(
        "The power data object selected is Adjustable, please enter the voltage in milivolts (mV)"
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let raw_voltage: f32 = input.trim().parse().expect("Invalid input");
    ElectricPotential::new::<millivolt>(raw_voltage)
}

fn get_current_selection_from_user() -> CurrentSelection {
    let last_current_selections_index = CURRENT_SELECTIONS.len() - 1;
    println!("Enter the Current you wish to select");
    println!("{CURRENT_SELECTIONS:?}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let error_message = format!(
        "Invalid Current Selection, must be between 0 and {}",
        last_current_selections_index
    );
    let current_selection: u8 = input.trim().parse().expect(&error_message);
    let current_selection = CurrentSelection::try_from(current_selection).expect(&error_message);
    println!("Current Selected: {current_selection}");
    current_selection
}
