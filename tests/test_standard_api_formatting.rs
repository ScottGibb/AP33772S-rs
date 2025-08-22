/// This module contains integration tests for verifying the formatting implementations
/// of various data structures in the `ap33772s_rs` crate. It includes tests for:
///
/// - `Statistics`: Checks the formatting of statistical data such as current, voltage, power, and temperature.
/// - `ThermalResistances`: Validates the formatting of thermal resistance values at different temperature points.
/// - `Thresholds`: Ensures correct formatting for various threshold values like over voltage, under voltage, over current, and over temperature.
///
/// Each test case prints the data structure using both the `Display` and `Debug` traits to compare the outputs.
/// To run these tests and see the formatted outputs in the console, execute the following command in your terminal:
///
/// ```bash
/// cargo test --test test_formatting --features "advanced" -- --nocapture
/// ```
///
/// Explanation:
///   - `cargo test`:  Runs all the tests in the project.
///   - `--test test_formatting`: Specifies that only the tests in the `test_formatting.rs` file should be run.
///   - `--features "advanced"`: Enables the "advanced" feature, which is required for some of the data structures used in the tests.
///   - `--`:  Separates the cargo test options from the options that are passed to the test binaries.
///   - `--nocapture`: Prevents `cargo test` from capturing the standard output. This ensures that `println!` macros in the tests are printed to the console.
///
/// The output will display the formatted data structures for `Statistics`, `ThermalResistances`, and `Thresholds` using both the `Display` and `Debug` traits.
mod standard_api_formatting_tests {
    use ap33772s_rs::types::Statistics;
    use ap33772s_rs::types::ThermalResistances;
    use ap33772s_rs::types::Thresholds;
    use ap33772s_rs::types::UnderVoltageThreshold;
    use ap33772s_rs::units::*;

    fn print_separator(title: &str) {
        println!("\n{}", "=".repeat(60));
        println!("Testing: {title}");
        println!("{}", "=".repeat(60));
    }

    #[test]
    fn test_statistics_formatting() {
        print_separator("Statistics");

        let statistics = Statistics {
            current: ElectricCurrent::new::<milliampere>(100), // 0.1A = 100mA
            voltage: ElectricPotential::new::<millivolt>(100),
            power: Power::new::<watt>(10),
            temperature: ThermodynamicTemperature::new::<degree_celsius>(25),
            requested_voltage: ElectricPotential::new::<millivolt>(120),
            requested_current: ElectricCurrent::new::<milliampere>(120), // 0.12A = 120mA
            requested_power: Power::new::<watt>(12),
        };

        println!("Statistics (Display): {statistics}");
        println!("Statistics (Debug): {statistics:?}");
    }

    #[test]
    fn test_thermal_resistances_formatting() {
        print_separator("ThermalResistances");

        let thermal_resistances = ThermalResistances {
            _25: ElectricalResistance::new::<milliohm>(250),
            _50: ElectricalResistance::new::<milliohm>(500),
            _75: ElectricalResistance::new::<milliohm>(750),
            _100: ElectricalResistance::new::<milliohm>(1000),
        };

        println!("ThermalResistances (Display): {thermal_resistances}");
        println!("ThermalResistances (Debug): {thermal_resistances:?}");
    }

    #[test]
    fn test_thresholds_formatting() {
        print_separator("Thresholds");

        let thresholds = Thresholds {
            over_voltage: ElectricPotential::new::<millivolt>(13000),
            under_voltage: UnderVoltageThreshold::EightyPercent,
            over_current: ElectricCurrent::new::<milliampere>(3500),
            over_temperature: ThermodynamicTemperature::new::<degree_celsius>(85),
            derating: ThermodynamicTemperature::new::<degree_celsius>(75),
        };

        println!("Thresholds (Display): {thresholds}");
        println!("Thresholds (Debug): {thresholds:?}");
    }
}
