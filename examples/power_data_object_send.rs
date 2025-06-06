use ap33772s_rs::{
    ap33772s::Ap33772s,
    commands::{
        data_objects::{
            all_source_power_data_object::PowerType,
            extended_power_range_data_object::ExtendedPowerSourcePowerType,
            source_power_data_object::SourcePowerType,
        },
        power_delivery::power_delivery_request_message::PowerDataObject,
    },
};
use uom::si::power;
use utils::setup_i2c;

fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new(i2c);

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {:?}", power_delivery_capabilities);

    // Select the Power Data Object Index
    println!("Enter Power Object Index (1-13):");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let power_data_object_index: u8 = input.trim().parse().expect("Invalid input");
    if power_data_object_index < 1 || power_data_object_index > 13 {
        panic!("Power Delivery Index must be between 1 and 13");
    }
    let power_data_object_index =
        PowerDataObject::new_with_raw_value(power_data_object_index.into())
            .expect("The Power Data Object Index must be between 1 and 13");
    print!(" Power Data Object Index: {:?}\n", power_data_object_index);

    // Check of the Power Data Object Index is not a fixed type //TODO: Replace this
    if let fixed = power_data_object_index >= PowerDataObject::ExtendedPowerRange9 {
        let index = usize::from(u8::from(
            power_data_object_index.raw_value() - PowerDataObject::ExtendedPowerRange9.raw_value(),
        ));
        power_delivery_capabilities.extended_power[index].source_power_type() == PowerType::Fixed;
    } else {
        let index: usize = usize::from(u8::from(power_data_object_index.raw_value()));
        power_delivery_capabilities.source_power[index].source_power_type()
            == SourcePowerType::Fixed;
    }

    if !fixed {
        println!(
            "The power data object is not a fixed type, please enter the voltage in milivolts (mV)"
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let voltage: u16 = input.trim().parse().expect("Invalid input");
    }
}
