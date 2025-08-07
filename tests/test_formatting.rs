//! # Formatting Tests
//!
//! This test file contains tests specifically for verifying the Display and Debug
//! formatting of various data structures in the AP33772S crate.
//!
//! ## Usage:
//!
//! Run individual formatting tests:
//! ```bash
//! cargo test test_statistics_formatting --features sync -- --nocapture
//! ```
//!
//! Run all formatting tests (requires advanced feature):
//! ```bash
//! cargo test formatting --features sync,advanced -- --nocapture
//! ```
//!
//! Run the comprehensive test (marked with #[ignore]):
//! ```bash
//! cargo test test_all_formatting --features sync,advanced -- --nocapture --ignored
//! ```
//!
//! Note: Use `--nocapture` to see the printed output in the console.

mod api_formatting_tests {
    use ap33772s_rs::types::Statistics;
    use ap33772s_rs::types::units::*;

    #[cfg(feature = "advanced")]
    use ap33772s_rs::commands::{
        configuration::{status::Status, system_control::SystemControl},
        data_objects::{
            all_source_power_data_object::AllSourceDataPowerDataObject,
            extended_power_range_data_object::ExtendedPowerRangeDataObject,
            source_power_data_object::SourcePowerDataObject,
        },
    };

    fn print_separator(title: &str) {
        println!("\n{}", "=".repeat(60));
        println!("Testing: {title}");
        println!("{}", "=".repeat(60));
    }

    #[test]
    fn test_statistics_formatting() {
        print_separator("Statistics");

        let statistics = Statistics {
            current: ElectricCurrent::new::<ampere>(0.1),
            voltage: ElectricPotential::new::<millivolt>(100.0),
            power: Power::new::<watt>(10.0),
            temperature: ThermodynamicTemperature::new::<degree_celsius>(25.0),
            requested_voltage: ElectricPotential::new::<millivolt>(120.0),
            requested_current: ElectricCurrent::new::<ampere>(0.12),
            requested_power: Power::new::<watt>(12.0),
        };

        println!("Statistics (Display): {statistics}");
        println!("Statistics (Debug): {statistics:?}");
    }

    #[test]
    #[cfg(feature = "advanced")]
    fn test_status_formatting() {
        print_separator("Status");

        let status = Status::default();

        println!("Status (Display): {}", status);
        println!("Status (Debug): {:?}", status);
        println!("Status raw_value: 0x{:02X}", status.raw_value());
    }

    #[test]
    #[cfg(feature = "advanced")]
    fn test_system_control_formatting() {
        print_separator("SystemControl");

        let system_control = SystemControl::default();

        println!("SystemControl (Display): {}", system_control);
        println!("SystemControl (Debug): {:?}", system_control);
        println!(
            "SystemControl raw_value: 0x{:02X}",
            system_control.raw_value()
        );
    }

    #[test]
    #[cfg(feature = "advanced")]
    fn test_source_power_data_object_formatting() {
        print_separator("SourcePowerDataObject");

        let power_obj = SourcePowerDataObject::default();

        println!("SourcePowerDataObject (Display): {}", power_obj);
        println!("SourcePowerDataObject (Debug): {:?}", power_obj);
        println!(
            "SourcePowerDataObject raw_value: 0x{:04X}",
            power_obj.raw_value()
        );
    }

    #[test]
    #[cfg(feature = "advanced")]
    fn test_extended_power_data_object_formatting() {
        print_separator("ExtendedPowerRangeDataObject");

        let extended_power_obj = ExtendedPowerRangeDataObject::default();

        println!(
            "ExtendedPowerRangeDataObject (Display): {}",
            extended_power_obj
        );
        println!(
            "ExtendedPowerRangeDataObject (Debug): {:?}",
            extended_power_obj
        );
        println!(
            "ExtendedPowerRangeDataObject raw_value: 0x{:04X}",
            extended_power_obj.raw_value()
        );
    }

    #[test]
    #[cfg(feature = "advanced")]
    fn test_all_source_power_data_object_formatting() {
        print_separator("AllSourceDataPowerDataObject");

        let all_power_obj = AllSourceDataPowerDataObject::default();

        println!("AllSourceDataPowerDataObject (Display):\n{}", all_power_obj);
        println!(
            "\nAllSourceDataPowerDataObject (Debug): {:?}",
            all_power_obj
        );
    }

    #[test]
    #[ignore] // Use this to run all formatting tests at once
    fn test_all_formatting() {
        test_statistics_formatting();

        #[cfg(feature = "advanced")]
        {
            test_status_formatting();
            test_system_control_formatting();
            test_source_power_data_object_formatting();
            test_extended_power_data_object_formatting();
            test_all_source_power_data_object_formatting();
        }
    }
}
