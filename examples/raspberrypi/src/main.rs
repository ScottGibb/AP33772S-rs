/// This is a simple example for the Raspberry Pi using the AP33772S driver.
/// It sets up the I2C communication and sends a power delivery request to the AP33772S device.
/// The example is designed to run on a Raspberry Pi with the AP33772S
use ap33772s_rs::ap33772s::Ap33772s;
use rppal::i2c::I2c;
fn main() {
    let i2c = I2c::new().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).expect("Failed to create AP33772S instance");

    let status = ap33772s.get_status().expect("Failed to get status");
    println!("AP33772S Status: {:?}", status);

    let power_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get power capabilities");
    println!("AP33772S Power Capabilities: {:?}", power_capabilities);
}
