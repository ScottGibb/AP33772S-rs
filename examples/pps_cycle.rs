use ap33772s_rs::types::command_structures::{OperatingCurrentSelection, PowerDataObject};
use ap33772s_rs::types::units::*;
use ap33772s_rs::{ap33772s::Ap33772s, types::command_structures::VoltageOutputControl};
use utils::setup_delay;
// This example shows how to cycle through the Adjustable Voltage Supply (AVS).
// It is inspired by the Centy Labs example for the AP33772S.
// [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/PPScycle/PPScycle.ino)
fn main() {
    let i2c = utils::setup_i2c(1_0000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");

    ap33772s
        .override_output_voltage(VoltageOutputControl::ForceOn)
        .expect("Failed to set MMOS Switch");

    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");
    for voltage in (3600..=20000).step_by(100) {
        // Set the Power Data Object
        let response = ap33772s
            .negotiate_power_delivery(
                PowerDataObject::StandardPowerRange1,
                Some(ElectricPotential::new::<millivolt>(voltage as f32)),
                OperatingCurrentSelection::_1A,
                &power_data_objects,
            )
            .expect("Failed to send power delivery request");
        println!("Power Delivery request Response: {response:?}");

        // Wait for a while to observe the change
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
