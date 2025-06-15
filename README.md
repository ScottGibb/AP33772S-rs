# AP33772S Rust Driver

[![MegaLinter](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/mega-linter.yaml/badge.svg)](https://github.com/ScottGibb/AP33772S-rs/actions/workflows/mega-linter.yaml)

## Under Development

This crate is still under heavy development and thus can and will change. Plans are to stabilise the API, however this will take time.

## Summary

This is a no-std rust crate for the AP33772S USB C Power Delivery chip by Diodes Incorporated. It attempts to be a fully featured driver offering low level register access through the `advanced` feature flag. However high level APIs with getters and setters can also be used and are encouraged to do so. The driver is cross platform thanks to the great work from the embedded-hal crate.  Async rust can also be utilised, thus supporting both async and sync modes of operation.

### Features

- no-std compliant
- synchronous mode through embedded-hal
- aysnchronous mode through embedded-hal-async
- Advanced mode for low level register access.
- Examples using the FT232H Breakout Board and a Mac or Linux device.

## Development

In terms of development the driver aims to follow the latest rust standards and PRs are more than welcome to improve or extend existing functionality. The project also contains GitHub Workflows to try and automate updates and testing.

### Continous Integration and GitHub workflows

At the moment there is [Continuous Build](./.github/workflows/continuous-build.yaml) workflow that aims to ensure the code can be compiled in both sync and async mode. This ensures that every iteration of the library can be built. Future plans are to incorporate some continuous Hardware in the Loop testing ensuring the driver works across multiple platforms.

### Project Linting

The project is linted using [MegaLinter]() which provides a variety of linters that will access not only rust code but documentation and other types of files as well. Ensuring a consistent standard.

## Datasheets

- [AP33772S Datasheet](./docs/AP33772S.pdf)
- [AP33772S Evaluation Board User Guide](./docs/AP33772S-Sink-Controller-EVB-User-Guide.pdf)
