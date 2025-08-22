use crate::commands::data_objects::source_power_range_data_object::{
    PeakCurrent, SourceMaximumCurrent,
};
use crate::units::*;
use crate::{
    commands::data_objects::source_power_range_data_object::PowerType, errors::Ap33772sError,
};
use arbitrary_int::u2;
use bitbybit::bitfield;

/// Represents the standard power range for the AP33772S.
/// Many Power supplies will support various `objects` that implement the StandardPowerRange.
/// This contains all the necessary information to select or query what the
/// power range capabilities are.
#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct StandardPowerRangeDataObject {
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

/// Maximum Voltage of 21 (21000 mV) with 100mV resolution
/// U16 can hold values up to 65535
/// 0 = 0mV
/// 1 = 100mV
/// 2 = 200mV
/// ...
/// 210 = 21000mV (21V)
/// This means the maximum raw value is 210
/// 210 * 100 = 21000mV
/// Therefore the voltage should be checked multiplied and stored in a U16
impl StandardPowerRangeDataObject {
    /// The Voltage Resolution defined in mV per LSB
    pub const VOLTAGE_RESOLUTION: u16 = 100;
    /// The Maximum Voltage that can be requested using this data object, this is not the `max_voltage` that the data object can provide,
    /// but rather the Absolute Maximum Voltage that the Standard rage profile can support.
    /// Use [max_voltage function](crate::commands::data_objects::standard_power_range_data_object::StandardPowerRangeDataObject::max_voltage)
    /// to find out what value is supported.
    /// It is is defined in mV
    pub const ABSOLUTE_MAXIMUM_VOLTAGE: u16 = 21000; // mV
    /// Returns the maximum voltage that can be requested using this data object.
    pub fn max_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        let scaled_voltage = u16::from(self.raw_max_voltage())
            .checked_mul(Self::VOLTAGE_RESOLUTION)
            .ok_or(Ap33772sError::ConversionFailed)?;
        Ok(ElectricPotential::new::<millivolt>(scaled_voltage))
    }

    /// Returns the peak current that can be requested using this data object.
    pub fn peak_current(&self) -> Option<PeakCurrent> {
        match self.source_power_type() {
            PowerType::Fixed => Some(PeakCurrent::from(self.minimum_voltage_or_peak_current())),
            PowerType::Adjustable => None,
        }
    }

    /// Returns the minimum voltage that can be requested using this data object.
    pub fn minimum_voltage(&self) -> Option<MinimumVoltage> {
        match self.source_power_type() {
            PowerType::Fixed => None,
            PowerType::Adjustable => {
                Some(MinimumVoltage::from(self.minimum_voltage_or_peak_current()))
            }
        }
    }
}

/// The supported minimum voltages for the Standard Power Range Data Object when working in Programmable Power Supply mode
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinimumVoltage {
    Reserved = 0,
    _3_3 = 1,
    _3_3To5 = 2,
    Others = 3,
}

impl From<u2> for MinimumVoltage {
    fn from(value: u2) -> Self {
        match value.value() {
            0 => MinimumVoltage::Reserved,
            1 => MinimumVoltage::_3_3,
            2 => MinimumVoltage::_3_3To5,
            3 => MinimumVoltage::Others,
            _ => unreachable!("This will never happen due to rust type safety"),
        }
    }
}

impl core::fmt::Display for StandardPowerRangeDataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "StandardPowerDataObject {{ max_voltage: {:?} V, minimum_voltage: {:?}, peak_current: {:?}, max_current: {:?} A, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .unwrap_or(ElectricPotential::new::<millivolt>(0))
                .get::<millivolt>(),
            self.minimum_voltage(),
            self.peak_current(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for StandardPowerRangeDataObject {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StandardPowerDataObject {{ max_voltage: {:?}, minimum_voltage: {:?}, max_current: {:?} A, source_power_type: {:?}, is_detected: {} }}",
            self.max_voltage()
                .unwrap_or(ElectricPotential::new::<millivolt>(0))
                .get::<millivolt>(),
            self.minimum_voltage(),
            self.max_current(),
            self.source_power_type(),
            self.is_detected()
        )
    }
}
