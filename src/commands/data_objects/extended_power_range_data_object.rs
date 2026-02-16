use crate::commands::data_objects::source_power_range_data_object::{
    PeakCurrent, PowerType, SourceMaximumCurrent,
};
use crate::errors::Ap33772sError;
use crate::units::*;
use arbitrary_int::u2;
use bitbybit::bitfield;

/// Represents the extended power range for the AP33772S.
/// This contains all the necessary information to select or query what the
/// power range capabilities are.
#[bitfield(u16, default = 0x00, defmt_bitfields(feature = "defmt"))]
#[derive(Debug, PartialEq)]
pub struct ExtendedPowerRangeDataObject {
    #[bits(0..=7, r)]
    pub raw_max_voltage: u8,
    #[bits(8..=9, r)]
    pub minimum_voltage_or_peak_current: u2,
    #[bits(10..=13, r)]
    pub max_current: SourceMaximumCurrent,
    #[bit(14, r)]
    pub source_power_type: PowerType,
    #[bit(15, r)]
    pub is_detected: bool,
}
/// Maximum Voltage of 28V (28000 mV) with 200mV resolution
/// U8 can hold values up to 255
/// 0 = 0mV
/// 1 = 200mV
/// 2 = 400mV
/// ...
/// 140 = 28000mV (28V)
/// This means the maximum raw value is 140
/// 140 * 200 = 28000mV U16 is required
/// Therefore the voltage should be checked multiplied
impl ExtendedPowerRangeDataObject {
    /// The Voltage Resolution defined in mV per LSB
    pub const VOLTAGE_RESOLUTION: u16 = 200;
    /// The Absolute Maximum Voltage that can be requested using this data object.
    /// The actual maximum voltage can be determined by using the [max_voltage function](crate::commands::data_objects::extended_power_range_data_object::ExtendedPowerRangeDataObject::max_voltage).
    /// This is defined in mV
    pub const ABSOLUTE_MAXIMUM_VOLTAGE: u16 = 28000;

    /// Returns the maximum voltage that can be requested using this data object.
    pub fn max_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        let scaled_voltage = u16::from(self.raw_max_voltage())
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)?;
        Ok(ElectricPotential::new::<millivolt>(f32::from(
            scaled_voltage,
        )))
    }
    /// Returns the peak current that can be requested using this data object.
    pub fn peak_current(&self) -> Option<PeakCurrent> {
        match self.source_power_type() {
            PowerType::Fixed => Some(PeakCurrent::from(self.minimum_voltage_or_peak_current())),
            PowerType::Adjustable => None,
        }
    }
    /// Returns the minimum voltage that can be requested using this data object.
    /// This is only applicable for Adjustable Power Supplies using the Adjustable Voltage Supply (AVS) feature
    pub fn minimum_voltage(&self) -> Option<MinimumVoltage> {
        match self.source_power_type() {
            PowerType::Fixed => None,
            PowerType::Adjustable => {
                Some(MinimumVoltage::from(self.minimum_voltage_or_peak_current()))
            }
        }
    }
}

/// The minimum voltage the Extended Power Range Data Object can provide
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinimumVoltage {
    Reserved = 0,
    Fifteen = 1,
    FifteenLessThanVoltageMinimumLessThanTwenty = 2,
    Others = 3,
}

impl From<u2> for MinimumVoltage {
    fn from(value: u2) -> Self {
        match value.value() {
            0 => MinimumVoltage::Reserved,
            1 => MinimumVoltage::Fifteen,
            2 => MinimumVoltage::FifteenLessThanVoltageMinimumLessThanTwenty,
            3 => MinimumVoltage::Others,
            _ => unreachable!("This will never happen due to rust type safety"),
        }
    }
}

impl core::fmt::Display for ExtendedPowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ExtendedPowerRangeDataObject {{ max_voltage: {:?}, minimum_voltage: {:?}, peak_current: {:?}, max_current: {:?}, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .unwrap_or(ElectricPotential::new::<millivolt>(f32::NEG_INFINITY))
                .get::<volt>(),
            self.minimum_voltage(),
            self.peak_current(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}
