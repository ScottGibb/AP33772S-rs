use bitbybit::{bitenum, bitfield};

#[bitfield(u16, default = 0x00)]
#[derive(Debug, PartialEq)]
pub struct SourcePowerDataObject {
    #[bits(0..=7, r)]
    pub max_voltage: u8,
    #[bits(8..=9, r)]
    pub minimum_voltage: MinimumVoltage,
    #[bits(10..=13, r)]
    pub max_current: SourcePowerCurrent,
    #[bit(14, r)]
    pub source_power_type : SourcePowerType,
    #[bit(15, r)]
    pub is_detected: bool,
}

/// VOLTAGE_MIN
/// For AVS APDO (bit[14]=1)
#[bitenum(u2, exhaustive=true)]
pub enum MinimumVoltage {
    Reserved = 0,
    _3_3 = 1,
    _3_3To5 = 2,
    Others = 3,
}

#[bitenum(u3, exhaustive=true)]
#[derive(Debug, PartialEq)]
pub enum SourcePowerCurrent {
  
}

#[bitenum(u1, exhaustive=true)]
#[derive(Debug, PartialEq)]
pub enum SourcePowerType {
    FixedPowerDataObject = 0,
    ProgrammablePowerSupplyAdjustablePowerDataObject = 1,
}
