use ap33772s_rs::Ap33772s;
use utils::{setup_delay, setup_i2c};

/// The following example shows how to query the AP33772S device for its status and power source delivery capabilities.
/// It can be run on a host machine using the FT232H Breakout Board. This example is based on the Vendor supplied Arduino examples
fn main() {
    let i2c = setup_i2c(1_000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s =
        Ap33772s::new_default(i2c, delay).expect("Failed to create AP33772S instance");

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {status}");
    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {power_delivery_capabilities}");
}
