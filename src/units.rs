//! # Physical Units and Measurements
//!
//! This module re-exports types and units from the [`uom`] (Units of Measurement) crate
//! that are used throughout the AP33772S driver. All measurements use strongly-typed
//! units to prevent unit conversion errors.
//!
//! ## Available Units
//!
//! ### Electrical Measurements
//! - [`ElectricCurrent`] - Current measurements (amperes, milliamperes)
//! - [`ElectricPotential`] - Voltage measurements (volts, millivolts)  
//! - [`ElectricalResistance`] - Resistance measurements (ohms, milliohms)
//! - [`Power`] - Power measurements (watts)
//!
//! ### Temperature Measurements  
//! - [`ThermodynamicTemperature`] - Temperature measurements (celsius)
//!
//! ## Unit Constants
//!
//! Each measurement type has associated unit constants for creating values:
//!
//! - **Current**: [`ampere`], [`milliampere`]
//! - **Voltage**: [`volt`], [`millivolt`]
//! - **Resistance**: [`ohm`], [`milliohm`]
//! - **Power**: [`watt`]
//! - **Temperature**: [`degree_celsius`]
//!
//! ## Usage Examples
//!
//! ```rust
//! use ap33772s_rs::units::*;
//!
//! // Create measurements using unit constants with integer values
//! let voltage = ElectricPotential::new::<volt>(5);      // 5 volts
//! let current = ElectricCurrent::new::<milliampere>(1500); // 1500 mA (1.5A)
//! let temp = ThermodynamicTemperature::new::<degree_celsius>(25); // 25°C
//!
//! // Convert between units
//! let voltage_mv = voltage.get::<millivolt>(); // 5000 (mV)
//! let current_a = current.get::<ampere>();     // 1 (A, result of integer division: 1500 mA / 1000 = 1, fractional part discarded)
//! ```
pub use uom::si::electric_current::ampere;
pub use uom::si::electric_current::milliampere;
pub use uom::si::electric_potential::millivolt;
pub use uom::si::electric_potential::volt;
pub use uom::si::electrical_resistance::milliohm;
pub use uom::si::electrical_resistance::ohm;
pub use uom::si::power::watt;
pub use uom::si::thermodynamic_temperature::degree_celsius;
pub use uom::si::u16::ElectricCurrent;
pub use uom::si::u16::ElectricPotential;
pub use uom::si::u16::ElectricalResistance;
pub use uom::si::u16::Power;
pub use uom::si::u16::ThermodynamicTemperature;
