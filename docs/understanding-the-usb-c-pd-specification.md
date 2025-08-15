# Understanding the USB C Power Delivery Specification

## Summary

This file contains my personalised notes for working with this device and hopefully provide some help to people new to using this hardware

## Acronyms

When working with USB C Power Delivery and this chip (AP33772S) in particular I found the following acronyms to be incredibly helpful to have a note of when reading the datasheet, as sometimes its not imediately clear what is going on.

### General

- **PD** : Power Delivery
- **TCD** : Type C Connected Device
- **NTC** : Negative Temperature Coefficeint Resister

### Power Data Objects

- **PDO** : Power Data Object
- **EPR** : Extended Power Range
- **SPR** : Standard Power Range
- **AVS** : Adjustable Voltage Supply
- **PPS** : Programmable Power Supply

### Protections

- **OVP** : Over Voltage Protection
- **UVP** : Under Voltage Protection
- **OCP** : Over Current Protection
- **OTP** : Over Temperature Protection

## Power Data Object

A PD source will provide it's capabilities which is a list of Power Data Objects (PDOs). A PDO is a struct that contains a power mode that is available from the source e.g. fixed 15V@1A (variable voltage and cell voltage are available too). The sink device can then select the most appropriate. 

## Standard Power Range

Standard Power Range (SPR) defines the original power range of USB-PD, supporting voltages from 5V, 9V, 12V, 15V and currents up to 3A (or 5A with a special cable). SPR is the baseline for USB-PD compatibility, ensuring devices can negotiate a suitable voltage and current level for safe and efficient power transfer within these defined limits.

## Extended Power Range

Extended Power Range (EPR) significantly expands the power capabilities of USB-PD, allowing for voltages up to 28V (in the case of the AP33772S) and currents up to 5A. This enables much higher power delivery, supporting devices with greater energy demands like laptops, power tools, and displays. EPR requires electronically marked cables rated for 5A and the higher voltage, ensuring safe operation at these elevated power levels.

## Adjustable Voltage Supply (AVS) versus Programmable Power Supply (PPS)

AVS: operates in the Extended Power Supply Range specifically 15V - 28V in the case of the AP33772S.

PPS: operates in the Standard Power Range specifically 3.3V to 21V

When working with the driver the user does not need to note the difference between PPS and AVS the API for accessing the `Adjustable` range of the power supply is the same

I noticed specifically when working PPS systems and my [Anker Power Bank](https://www.amazon.co.uk/dp/B0BYP2F3SG?ref_=ppx_hzsearch_conn_dt_b_fed_asin_title_1) that the PPS reported that it could do the minimum voltage of `3.3 to 5`. However in practice the device would not start to work until it hit 5.1V. The code for this was adjusted as such, see the below extract from the [Source Power Range Data Object](../src/commands/data_objects/source_power_range_data_object.rs)

```rust
    pub fn get_min_voltage(&self) -> Result<ElectricPotential, Ap33772sError> {
        match self {
            SourcePowerRangeDataObject::Standard(data_object) => {
                match data_object.minimum_voltage() {
                    Some(voltage) => match voltage {
                        StandardMinimumVoltage::_3_3 => {
                            Ok(ElectricPotential::new::<millivolt>(3300.0))
                        }
                        StandardMinimumVoltage::_3_3To5 => {
                            Ok(ElectricPotential::new::<millivolt>(5000.0)
                                + ElectricPotential::new::<millivolt>(
                                    StandardPowerRangeDataObject::VOLTAGE_RESOLUTION as f32,
                                ))
                        }
                        _ => Err(Ap33772sError::ConversionFailed),
                    },
                    None => Err(Ap33772sError::InvalidRequest(RequestError::MissingArgument)),
                }
            }
            SourcePowerRangeDataObject::Extended(data_object) => {
                match data_object.minimum_voltage() {
                    Some(voltage) => match voltage {
                        ExtendedMinimumVoltage::Fifteen => {
                            Ok(ElectricPotential::new::<millivolt>(15000.0))
                        }
                        ExtendedMinimumVoltage::FifteenLessThanVoltageMinimumLessThanTwenty => {
                            Ok(ElectricPotential::new::<millivolt>(20000.0)
                                + ElectricPotential::new::<millivolt>(
                                    ExtendedPowerRangeDataObject::VOLTAGE_RESOLUTION as f32,
                                ))
                        }
                        _ => Err(Ap33772sError::ConversionFailed),
                    },
                    None => Err(Ap33772sError::InvalidRequest(RequestError::MissingArgument)),
                }
            }
        }
    }
```
