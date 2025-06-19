#[cfg(feature = "advanced")]
mod imports {
    use ap33772s_rs::ap33772s::{AP33772SThresholds, Ap33772s};
    use uom::si::{
        electric_current::{ElectricCurrent, milliampere},
        electric_potential::{ElectricPotential, millivolt},
        f32::ThermodynamicTemperature,
        thermodynamic_temperature::degree_celsius,
    };
    use utils::setup_i2c;
}
#[cfg(not(feature = "advanced"))]
mod imports {}

#[allow(unused_imports)]
// Added here as Advanced feature is optional, Clippy complains otherwise
use imports::*;

/// This example demonstrates how to configure the AP33772S chip
/// for startup settings, including enabling interrupts and setting thresholds. This is done using the `advanced` feature in which you can read and write from the
/// registers directly as such in order to gain access to the advanced features you need to enable the `advanced` feature in your `Cargo.toml` file
#[cfg(feature = "advanced")]
fn main() {
    let i2c = setup_i2c().expect("Failed to set up I2C");
    let mut ap33772s = Ap33772s::new_default(i2c).unwrap();

    // Read The Status Register
    let status = ap33772s.get_status().expect("Failed to get status");
    println!("Status: {:?}", status);

    if status.started() && status.i2c_ready() {
        println!(" AP33772S Is up and running!")
    } else {
        println!("Status {:?}", status);
        panic!("AP33772 not ready...")
    }

    println!("Apply Startup Settings:");

    //Enable Interrupt
    let interrupts = InterruptEnable::default().with_started(true);
    ap33772s
        .write_one_byte_command(interrupts)
        .expect("Should not fail");

    // Unsure? TODO: Investigate this
    let config = ProtectionModeConfiguration::new_with_raw_value(0x1); //TODO: Investigate this
    ap33772s
        .write_one_byte_command(config)
        .expect("This should not fail");

    // Minimum Selection Voltage
    let minimum_selection_voltage = MinimumSelectionVoltage::builder()
        .with_raw_voltage(
            MinimumSelectionVoltage::convert_voltage_to_raw_voltage(ElectricPotential::new::<
                millivolt,
            >(0.0))
            .unwrap(),
        )
        .build();
    ap33772s
        .write_one_byte_command(minimum_selection_voltage)
        .expect("This should not fail");
    // Set Thresholds
    let thresholds = AP33772SThresholds {
        over_voltage: ElectricPotential::new::<millivolt>(80.0),
        under_voltage: UnderVoltageThreshold::SeventyFivePercent,
        over_current: ElectricCurrent::new::<milliampere>(0.0),
        over_temperature: ThermodynamicTemperature::new::<degree_celsius>(0x78 as f32), //TODO: Fix this
        derating: ThermodynamicTemperature::new::<degree_celsius>(0x78 as f32), //TODO: Fix this
    };

    //TODO: Investigate this Mystery Registers
    let vdc_threshold = VDCTHR::builder().with_percentage(0x06).build();
    ap33772s
        .write_one_byte_command(vdc_threshold)
        .expect("This should not fail");

    // Setup complete?? TODO: Investigate this example?
}

#[cfg(not(feature = "advanced"))]
fn main() {
    println!("This example requires the 'advanced' feature to be enabled.");
}
