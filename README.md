# AP33772S Rust Driver

[![MegaLinter](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/mega-linter.yaml/badge.svg)](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/mega-linter.yaml)
[![Continuous Build](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/continuous-build.yaml/badge.svg)](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/continuous-build.yaml)
![Crates.io Version](https://img.shields.io/crates/v/AP33772S-rs?color=green)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/ap33772s-rs)

## Under Development

This crate is still under heavy development and thus can and will change. Plans are to stabilise the API, however this will take time.

## Summary

This is a no-std rust crate for the AP33772S USB C Power Delivery chip by Diodes Incorporated. It attempts to be a fully featured driver offering low level register access through the `advanced` feature flag. However high level APIs with getters and setters can also be used and are encouraged to do so. The driver is cross platform thanks to the great work from the embedded-hal crate.  Async rust can also be utilised, thus supporting both async and sync modes of operation.

### Features

- no-std compliant
- synchronous mode through embedded-hal
- asynchronous mode through embedded-hal-async
- Advanced mode for low level register access.
- Examples using the FT232H Breakout Board and a Mac or Linux device.

The chip itself is summarised below:
>The AP33772S is a highly integrated USB Type-C® PD3.1 sink controller to support Extended Power Range (EPR) / Adjustable Voltage Supply (AVS) up to 28V and Standard Power Range (SPR) / Programmable Power Supply (PPS) up to 21V. The device is targeted for DC power request and control for flexible Type-C Connector- equipped Devices (TCDs) with an embedded host MCU (Micro Controller Unit) and I2C interface pins (SCL, SDA). Through the I2C interface interaction and interrupt mechanism, the host MCU sets up proper desired power profile (programmable voltage and current) with relevant threshold (OTP, OCP, etc.) and delegates PD negotiation tasks to the AP33772S. The host MCU further inquiries about status of various I2C registers for intended applications. Based on high-voltage process, the AP33772S offers short protection between CC1/CC2 pins to adjacent high-voltage pin up to 34V. Smart built-in firmware of the AP33772S offers comprehensive safety protection scheme, including overvoltage protection (OVP), undervoltage protection (UVP), overcurrent protection (OCP), over temperature protection (OTP) and moisture detection of the Type-C connector. In addition, external OTP and thermal de-rating are supported by a NTC resistor. Meanwhile, the AP33772S provides a LED pin to indicate the different PD power negotiation results, and a FLIP pin to show the cable plug- in orientation.

It is also advised to have a glance through the [Understanding the USB C PD Specification](./docs/understanding-the-usb-c-pd-specification.md) Notes provided. As this will hoepfully provide further insight into how to use this chip and the driver provided.

### Quick Start

To run the examples quickly simply run the following commands:

```bash
cargo run --example <example script name> --no-default-features --features sync
```

`sync` feature is required so that the FT232H Breakout board can be used on your dev machine. See [DEVELOPMENT.md](./DEVELOPMENT.md) for more details.

## Development

In terms of development, the driver aims to follow the latest rust standards and PRs are more than welcome to improve or extend existing functionality. The project also contains GitHub Workflows to try and automate updates and testing. Please read [DEVELOPMENT.md](./DEVELOPMENT.md) for more details.

## Datasheets

The datasheets are also stored in this repository for convenience purposes. They are listed below:

- [AP33772S Datasheet](./docs/AP33772S.pdf)
- [AP33772S Evaluation Board User Guide](./docs/AP33772S-Sink-Controller-EVB-User-Guide.pdf)
