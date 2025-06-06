use ap33772s_rs::ap33772s::Ap33772s;
use ftdi::Device;
use ftdi_embedded_hal::{self as hal, I2c};
use std::error::Error;

/// The following example shows how to query the AP33772S device for its status and power source delivery capabilities.
/// It can be run on a host machine using the FT232H Breakout Board. This example is based on the Vendor supplied Arduino examples
fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new(i2c);

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    // Get the Power Source Delivery Capabilities
    let power_delivery_capabilities = ap33772s
        .get_all_source_power_capabilities()
        .expect("Failed to get Power Source Delivery Capabilities");
    println!("Capabilities: {:?}", power_delivery_capabilities);
}

pub fn setup_i2c() -> Result<I2c<Device>, Box<dyn Error>> {
    const BAUDRATE: u32 = 400_000;
    // Change these for your device
    const DEVICE_VID: u16 = 0x0403;
    const DEVICE_PID: u16 = 0x6014;

    let device = ftdi::find_by_vid_pid(DEVICE_VID, DEVICE_PID)
        .interface(ftdi::Interface::A)
        .open()?;
    // Next initialise the HAL with the device and the Baudrate
    let hal = match hal::FtHal::init_freq(device, BAUDRATE) {
        Ok(hal) => hal,
        Err(err) => {
            eprintln!("Failed to initialise HAL: {}", err);
            return Err(Box::new(err));
        }
    };
    // Finally initialise the I2C with the HAL
    let i2c = match hal.i2c() {
        Ok(i2c) => i2c,
        Err(err) => {
            eprintln!("Failed to initialise I2C: {}", err);
            return Err(Box::new(err));
        }
    };
    Ok(i2c)
}
