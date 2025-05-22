use ap33772s_rs::{ap33772s::Ap33772s, commands};
use ftdi::Device;
use ftdi_embedded_hal::{self as hal, I2c};
use std::{error::Error, process::Command};
fn main() {
 let i2c = setup_i2c().expect("Failed to set up I2C");
 let ap33772s = Ap33772s::new(i2c);

 // Set Some Registers with the feature register flag enabled
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