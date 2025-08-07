#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::i2c::master::Config;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::{clock::CpuClock, i2c::master::I2c};

use ap33772s_rs::ap33772s::Ap33772s;
use defmt::error;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use panic_rtt_target as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // Create an I2C Bus for the AP33772S device
    let i2c = I2c::new(peripherals.I2C0, Config::default().with_frequency(Rate::from_hz(100_000)))
        .expect("Failed to Create I2C")
        .with_scl(peripherals.GPIO8)
        .with_sda(peripherals.GPIO9)
        .into_async();

    let mut ap33772s = Ap33772s::new(i2c); // Skip the initialization check for this example

    loop {
        info!("Checking if AP33772S is present...");
        match ap33772s.is_device_present().await {
            Ok(_) => {
                info!("AP33772S is present!");

                // Read the Status Register
                match ap33772s.get_status().await {
                    Ok(status) => {
                        info!("Status:");
                        info!("Started: {}", status.started());
                        info!("I2C Ready: {}", status.i2c_ready());
                        info!("New Power Data Object: {}", status.new_power_data_object());
                        info!(
                            "Under Voltage Protection: {}",
                            status.under_voltage_protection()
                        );
                        info!(
                            "Over Voltage Protection: {}",
                            status.over_voltage_protection()
                        );
                        info!(
                            "Over Current Protection: {}",
                            status.over_current_protection()
                        );
                        info!(
                            "Over Temperature Protection: {}",
                            status.over_temperature_protection()
                        );
                    }
                    Err(e) => error!("Failed to read status: {:?}", e),
                }

                // // Read the State of the Device
                // match ap33772s.get_statistics().await {
                //     Ok(stats) => info!("State: {}", stats.current),
                //     Err(e) => error!("Failed to read statistics: {:?}", e),
                // }

                Timer::after(Duration::from_secs(1)).await;
            }
            Err(e) => {
                info!("AP33772S is not present: {:?}", e);
                Timer::after(Duration::from_secs(2)).await;
            }
        }
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
