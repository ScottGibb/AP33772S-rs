//! This module provides the core command-related functionality for the project.
//!
//! It organizes various submodules that handle specific aspects of command processing,
//! configuration, data management, power delivery, statistics, and more.
//!
pub mod command_map;
pub mod configuration;
pub mod data_objects;
pub mod misc;
pub mod power_delivery;
pub mod requested;
pub mod statistics;
pub mod thermal_resistances;
pub mod thresholds;
// Keep macros private - they should not be exposed to end users
mod macros;
