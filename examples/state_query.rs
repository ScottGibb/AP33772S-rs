use ap33772s_rs::ap33772s::Ap33772s;
use utils::setup_i2c;

fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new(i2c);

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    // Read the State of the Device
    let stats = ap33772s.get_statistics().expect("Should not fail");
    println!("State {:?}", stats);
}
