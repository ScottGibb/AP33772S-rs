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

From my understanding the Power Data Object (PDO) is essentially a struct that contains the key information of the power delivery mode. This defines its capablities that each PDO can do and if it is `detected` on the sink device. The Sink device being the USB Type C Power Supplier.

## Standard Power Range

## Extended Power Range

## Adjustable Voltahe Supply and Programmable Power Supply

## What is the DeRating Feature ?
