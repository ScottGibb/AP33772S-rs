use ap33772s_rs::ap33772s::Ap33772s;
use ap33772s_rs::types::PowerDeliveryMode;
use ap33772s_rs::types::ap33772s_structures::PowerDataObject;
use ap33772s_rs::types::ap33772s_structures::VoltageOutputControl;
use ap33772s_rs::types::units::*;
use utils::setup_delay;
use utils::setup_i2c;
const COMMANDS: [&str; 9] = [
    "Profile",
    "OpMode",
    "On",
    "Off",
    "Temperature",
    "Statistics",
    "Maximum",
    "exit",
    "quit",
];

/// This example shows how to toggle the Serial Profile On and Off using the AP33772S device.
/// It is inspired by the Centy Labs example for the AP33772S.
/// [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/SerialProfileOnOff/SerialProfileOnOff.ino)
fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");
    ap33772s
        .set_power_delivery_mode(PowerDeliveryMode {
            extended_power_range_mode_enabled: true,
            programmable_power_supply_adjustable_voltage_supply_enabled: true,
        })
        .expect("Failed to set Power Delivery Mode");
    loop {
        println!("Enter the Command:");
        println!("{COMMANDS:?}");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "Profile" => {
                let power_data_objects = ap33772s
                    .get_all_source_power_capabilities()
                    .expect("Failed to get power data object indices");
                println!("Available Power Data Objects: {power_data_objects}");
            }
            "OpMode" => {
                let operating_mode = ap33772s
                    .get_operating_mode()
                    .expect("Failed to get operating mode");
                println!("Current Operating Mode: {operating_mode}");
            }
            "On" => {
                ap33772s
                    .override_output_voltage(VoltageOutputControl::ForceOn)
                    .expect("Failed to set MMOS Switch");
                println!("MMOS Switch set to Force On");
            }
            "Off" => {
                ap33772s
                    .override_output_voltage(VoltageOutputControl::ForceOff)
                    .expect("Failed to set MMOS Switch");
                println!("MMOS Switch set to Force Off");
            }
            "exit" | "quit" => {
                println!("Exiting...");
                break;
            }
            "Temperature" => {
                let temperature = ap33772s
                    .get_temperature()
                    .expect("Failed to get temperature")
                    .get::<degree_celsius>();
                println!("Current Temperature: {temperature:.2}°C");
            }
            "Statistics" => {
                let statistics = ap33772s.get_statistics().expect("Failed to get statistics");
                println!("Current Statistics: {statistics}");
            }
            "Maximum" => {
                let power_data_object = PowerDataObject::StandardPowerRange4;
                println!("Requesting Maximum Power Delivery on {power_data_object}");
                let result = ap33772s
                    .negotiate_maximum_power_delivery(power_data_object)
                    .expect("Failed to negotiate maximum power delivery");
                println!("Maximum Power Delivery Response: {result:?}");
            }
            _ => {
                println!("Invalid command");
                continue;
            }
        }
        println!("Command executed successfully");
    }
}
