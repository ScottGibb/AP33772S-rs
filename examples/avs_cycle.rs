/// The following example shows how to cycle through the Adjustable Voltage Supply (AVS). It is inspired
/// by the Centy Labs example for the AP33772S. [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/AVScycle/AVScycle.ino)
use ap33772s_rs::{
    ap33772s::{Ap33772s, CurrentSelection, PowerDataObject},
    setters::VoltageOutputControl,
};
use uom::si::{electric_potential::millivolt, f32::ElectricPotential};
use utils::setup_i2c;
fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).unwrap();

    loop {
        // Set the MMOS Switch
        ap33772s
            .override_output_voltage(VoltageOutputControl::ForceOn)
            .unwrap();

        let power_data_objects = ap33772s
            .get_all_source_power_capabilities()
            .expect("Failed to get power data object indices");

        // Increase the voltage using the first extended power range data object
        for voltage in (15000..=30000).step_by(1000) {
            // Send a power delivery request with the current voltage
            ap33772s
                .send_power_delivery_request(
                    PowerDataObject::ExtendedPowerRange8,
                    Some(ElectricPotential::new::<millivolt>(voltage as f32)),
                    CurrentSelection::_3A,
                    &power_data_objects,
                )
                .expect("Failed to send power delivery request");

            // Wait for a while to observe the change
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
