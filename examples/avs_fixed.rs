use ap33772s_rs::types::units::*;
use ap33772s_rs::{
    ap33772s::Ap33772s,
    types::api_commands::{OperatingCurrentSelection, PowerDataObject, VoltageOutputControl},
};
use utils::setup_delay;
// This another example inspired by the Centy Labs example for the AP33772S.
// [Centy Labs Example](https://github.com/CentyLab/AP33772S-CentyLab/blob/main/examples/AVSfixed/AVSfixed.ino)
// This example is designed to work with the [Anker Prime 27,650mAh Power Bank (250W)](https://www.amazon.co.uk/dp/B0BYP2F3SG?ref_=ppx_hzsearch_conn_dt_b_fed_asin_title_1&th=1)
fn main() {
    let delay = setup_delay();
    let i2c = utils::setup_i2c(1_000).expect("Failed to set up I2C");
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");

    let adjustable_voltage_supply = ElectricPotential::new::<millivolt>(16000.0);
    // Set the MMOS Switch
    ap33772s
        .override_output_voltage(VoltageOutputControl::ForceOn)
        .expect("Failed to set MMOS Switch");

    let power_data_objects = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power data object indices");

    loop {
        let response = ap33772s
            .negotiate_power_delivery(
                PowerDataObject::StandardPowerRange6,
                Some(adjustable_voltage_supply),
                OperatingCurrentSelection::_3A,
                &power_data_objects,
            )
            .expect("Failed to send power delivery request");
        println!("Power Delivery request Response: {response:?}");

        std::thread::sleep(std::time::Duration::from_secs(1)); // Call AVS every second to stop the charger from disconnecting if no sink
    }
}
