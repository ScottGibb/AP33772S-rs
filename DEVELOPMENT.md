# Development Guide

When first diving into the project, I would advise having a quick look at the [Understanding the USB C Spec](./docs/understanding-the-usb-c-pd-specification.md). Its a short markdown file that contains some useful information for getting started with this device.

## Continuous Integration and GitHub workflows

At the moment there is [Continuous Build](./.github/workflows/continuous-build.yaml) workflow that aims to ensure the code can be compiled in both sync and async mode. This ensures that every iteration of the library can be built. Future plans are to incorporate some continuous Hardware in the Loop testing ensuring the driver works across multiple platforms. When adding extra [examples](./examples/). The developer should ensure these are being targeted in the [Continuous Build](./.github/workflows/continuous-build.yaml) workflow file. This ensures the example files are always up to date with the source code. We also include a [Continuous test](./.github/workflows/continuous-test.yaml) workflow for verifying the drivers functionality works.

[Dependabot](./.github/dependabot.yaml) is also used to check when dependencies need updated and ensure we are always up to date with the latest crates.

[MegaLinter](./.mega-linter.yaml) is also used throughout the project and can be used locally (see below) but also runs on GitHub to ensure we catch inconsistencies.

Finally upon release, we use [Release Plz](https://release-plz.dev/docs/release-plz-in-the-wild) to create releases for both the [Github Releases](https://github.com/ScottGibb/AP33772S-rs/releases) and <!-- markdown-link-check-disable -->[Crates.io release](https://crates.io/crates/ap33772s-rs)<!-- markdown-link-check-enable -->. The configuration of which can be found in the following [workflow](./.github/workflows/release-plz.yaml)

### Project Linting

The project is linted using [MegaLinter](https://megalinter.io/latest/) which provides a variety of linters that will not only check rust code but documentation and other types of files as well. Ensuring a consistent standard. Developers can run this locally as well via the following command:

```bash
npx mega-linter-runner --fix
```

However you will need Docker and NodeJS installed to run the linter.

### Visual Studio Code Setup

VSCode is used for this project, it is not required to contribute but it does come in handy for setting the project up and using the linters. You can see the recommended plugins in [.extensions.json](.vscode/extensions.json). The project is also setup to format on save as well.

## Running Examples

This crate comes with examples ready to go, some of the examples are inspired by the [Arduino library](https://github.com/CentyLab/AP33772S-CentyLab/tree/main), others are inspired by the vendor support examples shown in this [guide](./docs/AP33772S-Sink-Controller-EVB-User-Guide.pdf).

The examples are designed to be ran on your developer machine such as Mac or Linux with the help of the [FT232H Breakout Board](https://www.adafruit.com/product/2264?srsltid=AfmBOopHJEgnh4a6cuJ9i4CleCgHksuY1m4pmx0XMOKLCBGbsPyLs8iE) to provide USB to I2C bridge. More on this can be found in the Examples [README](./examples/README.md)
