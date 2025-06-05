use num_enum::{IntoPrimitive, TryFromPrimitive};

/// This module defines the command map for the AP33772S device, including various commands
/// for configuration, power delivery, and status management.
/// The `Command` enum represents the different commands that can be sent to the device,
/// each associated with a specific functionality or register.
///
/// To use this enum, you can simple do the following:
/// ```
/// // In the case of reading Registers, you can use the `read_one_byte_command` method
/// let voltage = self.read_one_byte_command::<Voltage>()?;
/// // // In the case of writing to Registers, you can use the `write_one_byte_command` method
/// //TODO FIll this in
/// ```
#[derive(Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Command {
    /// Command Associated with the STATUS Register
    /// See [Status](crate::commands::configuration::status::Status) for more details
    ///
    /// DataSheet Name: STATUS
    Status = 0x01,
    /// The AP33772S supports a level-triggered interrupt signal through the INT pin to the host MCU.
    /// The [Interrupt Enable](crate::commands::configuration::interrupt_enable::InterruptEnable)
    /// register defines the enable and disable of ON and OFF for various
    /// [Status](crate::commands::configuration::status::Status)-defined events
    ///
    /// Datasheet Name: MASK
    InterruptEnableMask = 0x02,
    /// The Operation Mode register provides the current operation mode of the AP33772S.
    /// See [OperationMode](crate::commands::configuration::operation_mode::OperationMode) for more details
    ///
    /// Datasheet Name: OPMODE
    OperationMode = 0x03,
    /// The System Configuration register is defined as the system configuration options that enable specific modules
    /// See [ProtectionModeConfiguration](crate::commands::configuration::protection_mode_configuration::ProtectionModeConfiguration) for more details
    ///
    /// Datasheet Name: CONFIG
    SystemConfiguration = 0x04,
    /// Power Delivery Configuration register is defined as the Power Delivery mode configuration options that enable specific modules
    /// See [PowerDeliveryConfiguration](crate::commands::configuration::power_delivery_configuration::PowerDeliveryConfiguration) for more details
    ///
    /// Datasheet Name: PDCONFIG
    PowerDeliveryConfiguration = 0x05,
    /// The SystemControl register is defined as the system information and control options that request specific functions.
    ///  See [SystemControl](crate::commands::configuration::system_control::SystemControl) for more details
    ///  By default, the VOUT MOS switches are controlled by the AP33772S.
    ///  Writing the [VOutControl](crate::commands::configuration::system_control::VOutControl) parameter can force the VOUT MOS switches to turn OFF/ON.
    ///
    /// Datasheet Name: SYSTEM
    SystemControl = 0x06,
    /// The ThermalResistance25 register is defined as the thermal resistance of the AP33772S at 25°C.
    /// See [ThermalResistance25](crate::commands::thermal_resistances::thermal_resistance_25::ThermalResistance25) for more details
    ///
    /// Datasheet Name: TR25
    ThermalResistance25 = 0x0C,
    /// The ThermalResistance50 register is defined as the thermal resistance of the AP33772S at 50°C.
    /// See [ThermalResistance50](crate::commands::thermal_resistances::thermal_resistance_50::ThermalResistance50) for more details
    ///
    /// Datasheet Name: TR50
    ThermalResistance50 = 0x0D,
    /// The ThermalResistance75 register is defined as the thermal resistance of the AP33772S at 75°C.
    /// See [ThermalResistance75](crate::commands::thermal_resistances::thermal_resistance_75::ThermalResistance75) for more details
    ///
    /// Datasheet Name: TR75
    ThermalResistance75 = 0x0E,
    /// The ThermalResistance100 register is defined as the thermal resistance of the AP33772S at 100°C.
    /// See [ThermalResistance100](crate::commands::thermal_resistances::thermal_resistance_100::ThermalResistance100) for more details
    ///
    /// Datasheet Name: TR100
    ThermalResistance100 = 0x0F,
    /// Voltage Register is defined as the output voltage of the AP33772S.
    /// See [Voltage](crate::commands::statistics::voltage::Voltage) for more details
    ///
    /// Datasheet Name: Voltage
    Voltage = 0x11,
    /// Current Register is defined as the output current of the AP33772S.
    /// See [Current](crate::commands::statistics::current::Current) for more details
    ///
    /// Datasheet Name: Current
    Current = 0x12,
    /// Temperature Register is defined as the temperature of the AP33772S.
    /// See [Temperature](crate::commands::statistics::temperature::Temperature) for more details
    ///
    /// Datasheet Name: TEMP
    Temperature = 0x13,
    /// The VoltageRequested register is defined as the requested output voltage of the AP33772S.
    /// See [VoltageRequested](crate::commands::requested::voltage_requested::VoltageRequested) for more details
    ///
    /// Datasheet Name: VREQ
    VoltageRequested = 0x14,
    /// The CurrentRequested register is defined as the requested output current of the AP33772S.
    /// See [CurrentRequested](crate::commands::requested::current_requested::CurrentRequested) for more details
    ///
    /// Datasheet Name: IREQ
    CurrentRequested = 0x15,
    /// The Minimum Selection Voltage os defined as the minimum selection voltage of the AP33772S.
    /// See [MinimumSelectionVoltage](crate::commands::statistics::minimum_selection_voltage::MinimumSelectionVoltage) for more details
    ///
    /// Command: VSELMIN
    MinimumSelectionVoltage = 0x16,
    /// The Under Voltage Protection Threshold is defined as the under voltage protection threshold of the AP33772S.
    /// See [UnderVoltageProtectionThreshold](crate::commands::thresholds::under_voltage_protection_threshold::UnderVoltageProtectionThreshold) for more details
    ///
    /// Datasheet Name: UVPTHR
    UnderVoltageProtectionThreshold = 0x17,
    /// The Over Voltage Protection Threshold is defined as the over voltage protection threshold of the AP33772S.
    /// See [OverVoltageProtectionThreshold](crate::commands::thresholds::over_voltage_protection_threshold::OverVoltageProtectionThreshold) for more details
    ///
    /// Datasheet Name: OVPTHR
    OverVoltageProtectionThreshold = 0x18,
    /// The Over Current Protection Threshold is defined as the over current protection threshold of the AP33772S.
    /// See [OverCurrentProtectionThreshold](crate::commands::thresholds::over_current_protection_threshold::OverCurrentProtectionThreshold) for more details
    ///
    /// Datasheet Name: OCPTHR
    OverCurrentProtectionThreshold = 0x19,
    /// The Over Temperature Protection Threshold is defined as the over temperature protection threshold of the AP33772S.
    /// See [OverTemperatureProtectionThreshold](crate::commands::thresholds::over_temperature_protection_threshold::OverTemperatureProtectionThreshold) for more details
    ///
    /// Datasheet Name: OTPTHR
    OverTemperatureProtectionThreshold = 0x1A,
    DeRatingThreshold = 0x1B,
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
