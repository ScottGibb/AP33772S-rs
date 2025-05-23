use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]

pub enum Command {
    Status = 0x01,
    InterruptEnableMask = 0x02,
    OperationMode = 0x03,
    SystemConfiguration = 0x04,
    PowerDeliveryConfiguration = 0x05,
    SystemControl = 0x06,
    ThermalResistance25 = 0x0C,
    ThermalResistance50 = 0x0D,
    ThermalResistance75 = 0x0E,
    ThermalResistance100 = 0x0F,
    Voltage = 0x11,
    Current = 0x12,
    Temperature = 0x13,
    RequestedVoltage = 0x14,
    RequestedCurrent = 0x15,
    MinimumSelectionVoltage = 0x16,
    UnderVoltageProtectionThreshold = 0x17,
    OverVoltageProtectionThreshold = 0x18,
    OverCurrentProtectionThreshold = 0x19,
    OverTemperatureProtectionThreshold = 0x1A,
    DeratingThreshold = 0x1B,
    /// Get All PD Source Power Capabilities (PDO1 to PDO13)
    /// Command: SRCPDO 
    AllSourcesPowerDataObject = 0x20,
    /// Source Standard Power Range  Power Data Object 1
    /// Command: SRC_SPR_PDO1
    SourceStandardPowerRange1 = 0x21,
    /// Source Standard Power Range  Power Data Object 2
    /// Command: SRC_SPR_PDO2
    SourceStandardPowerRange2 = 0x22,
    /// Source Standard Power Range  Power Data Object 3
    /// Command: SRC_SPR_PDO3
    SourceStandardPowerRange3 = 0x23,
    /// Source Standard Power Range  Power Data Object 4
    /// Command: SRC_SPR_PDO4
    SourceStandardPowerRange4 = 0x24,
    /// Source Standard Power Range  Power Data Object 5
    /// Command: SRC_SPR_PDO5
    SourceStandardPowerRange5 = 0x25,
    /// Source Standard Power Range  Power Data Object 6
    /// Command: SRC_SPR_PDO6
    SourceStandardPowerRange6 = 0x26,
    /// Source Standard Power Range  Power Data Object 7
    /// Command: SRC_SPR_PDO7
    SourceStandardPowerRange7 = 0x27,
    /// Source Extended Power Range  Power Data Object 8
    /// Command: SRC_EPR_PDO8
    SourceExtendedPowerRange8 = 0x28,
    /// Source Extended Power Range  Power Data Object 9
    /// Command: SRC_EPR_PDO9
    SourceExtendedPowerRange9 = 0x29,
    /// Source Extended Power Range  Power Data Object 10
    /// Command: SRC_EPR_PDO10
    SourceExtendedPowerRange10 = 0x2A,
    /// Source Extended Power Range  Power Data Object 11
    /// Command: SRC_EPR_PDO11
    SourceExtendedPowerRange11 = 0x2B,
    /// Source Extended Power Range  Power Data Object 12
    /// Command: SRC_EPR_PDO12
    SourceExtendedPowerRange12 = 0x2C,
    /// Source Extended Power Range  Power Data Object 13
    /// Command: SRC_EPR_PDO13 
    SourceExtendedPowerRange13 = 0x2D,
    /// Send request message with selected voltage, current and Power Data Object(PDO) index 
    /// Command: PD_REQMSG
    PowerDeliveryRequestMessage = 0x31,
    /// Send request message with selected voltage, current and PDO index
    /// Command: PD_CMDMSG 
    PowerDeliveryCommandMessage = 0x32,
    /// Result and status of PD request or command message
    /// Command: PD_MSGRLT 
    PowerDeliveryMessageResult = 0x33,
}