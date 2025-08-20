use ap33772s_rs::Ap33772s;
use utils::setup_delay;
use utils::setup_i2c;
fn main() {
    let i2c = setup_i2c(1_0000).expect("Failed to set up I2C");
    let delay = setup_delay();
    let mut ap33772s = match Ap33772s::new_default(i2c, delay) {
        Ok(device) => device,
        Err(e) => panic!("Failed to create AP33772S device: {e}"),
    };

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {status}");

    // Read the State of the Device
    let stats = ap33772s.get_statistics().expect("Should not fail");
    println!("State {stats}");
}
