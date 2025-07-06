use ap33772s_rs::{
    ap33772s::{Ap33772s, CurrentSelection, PowerDataObject},
    commands::configuration::system_control::VoltageOutputControl,
};
use uom::si::{electric_potential::millivolt, f32::ElectricPotential};

fn main() {
    let adjustable_voltage_supply = ElectricPotential::new::<millivolt>(16000.0);
    let i2c = utils::setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).expect("Failed to create AP33772S instance");

    // Set the MMOS Switch
    ap33772s
        .set_voltage_out_override(VoltageOutputControl::ForceOn)
        .expect("Failed to set MMOS Switch");

    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");

    loop {
        ap33772s
            .send_power_delivery_request(
                PowerDataObject::ExtendedPowerRange8,
                Some(adjustable_voltage_supply),
                CurrentSelection::_3A,
                &power_data_objects,
            )
            .expect("Failed to send power delivery request");

        std::thread::sleep(std::time::Duration::from_secs(1)); // Call AVS every second to stop the charger from disconnecting if no sink
    }
}
