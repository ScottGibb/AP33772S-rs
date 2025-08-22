use ap33772s_rs::Ap33772s;
use ap33772s_rs::types::command_structures::{
    OperatingCurrentSelection, PowerDataObject, StandardPowerRangeDataObject,
};
use ap33772s_rs::units::*;
use utils::setup_delay;

const PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT: PowerDataObject =
    PowerDataObject::StandardPowerRange6;

// This example shows how to cycle through the Adjustable Voltage Supply (AVS).
// It is inspired by the Centy Labs example for the AP33772S.
// [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/PPScycle/PPScycle.ino)
fn main() {
    let i2c = utils::setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");
    println!("AP33772S instance created successfully");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Retrieving power data objects...");
    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");
    println!("Power Data Objects: {power_data_objects}");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Beginning voltage negotiation...");
    for voltage in (5100..=StandardPowerRangeDataObject::ABSOLUTE_MAXIMUM_VOLTAGE)
        .step_by(StandardPowerRangeDataObject::VOLTAGE_RESOLUTION as usize)
    {
        // Set the Power Data Object
        let response = ap33772s
            .negotiate_power_delivery(
                PROGRAMMABLE_POWER_SUPPLY_POWER_DATA_OBJECT,
                Some(ElectricPotential::new::<millivolt>(voltage)),
                OperatingCurrentSelection::_1A,
                &power_data_objects,
            )
            .expect("Failed to send power delivery request");
        println!("Power Delivery request Response: {response:?} with Voltage: {voltage} mV");

        // Wait for a while to observe the change
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
