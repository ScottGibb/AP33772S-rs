use ap33772s_rs::types::units::*;
use ap33772s_rs::{ap33772s::Ap33772s, types::VoltageOutputControl};
use utils::setup_delay;
use utils::setup_i2c;
const COMMANDS: [&str; 6] = ["Profile", "On", "Off", "Temperature", "exit", "quit"];

/// This example shows how to toggle the Serial Profile On and Off using the AP33772S device.
/// It is inspired by the Centy Labs example for the AP33772S.
/// [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/SerialProfileOnOff/SerialProfileOnOff.ino)
fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");

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
                let temperature: ThermodynamicTemperature = ap33772s
                    .get_temperature()
                    .expect("Failed to get temperature");
                println!("Current Temperature: {temperature:?}");
            }
            _ => {
                println!("Invalid command");
                continue;
            }
        }
        println!("Command executed successfully");
    }
}
