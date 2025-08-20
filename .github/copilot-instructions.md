# AP33772S-rs Driver Development Instructions

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Build
- **System Dependencies (Linux)**:
  - `sudo apt update && sudo apt install libftdi1 libftdi1-dev -y`
  - **CRITICAL**: Examples and tests WILL FAIL without these libraries
- **System Dependencies (macOS)**:
  - `brew install libftdi`
  - If installation fails: `brew unlink libftdi && brew link libftdi`
- **Clean Build**: `cargo clean` -- removes all build artifacts when needed
- **Build Commands**:
  - Basic sync build: `cargo build --no-default-features --features sync` -- takes 10-30 seconds first time, <1 second incremental
  - Async build: `cargo build --no-default-features --features async` -- takes 2-3 seconds first time
  - Advanced features: `cargo build --no-default-features --features "sync advanced"` -- takes <1 second incremental
  - Release build: `cargo build --release --no-default-features --features sync` -- takes 10 seconds. NEVER CANCEL.
  - Documentation: `cargo doc --no-default-features --features sync` -- takes 11 seconds. NEVER CANCEL.

### Testing and Validation
- **Run Tests**: `cargo test --no-default-features --features sync` -- takes 3 seconds. NEVER CANCEL.
- **Format Test with Output**: `cargo test --test test_standard_api_formatting --features "advanced" -- --nocapture` -- shows formatted output
- **Test Timeout**: Set timeout to 5+ minutes for initial builds, 2+ minutes for incremental builds and tests
- **CRITICAL**: All tests require libftdi1 libraries installed. Tests will link error without them.

### Linting and Code Quality
- **Format Check**: `cargo fmt --all -- --check` -- takes 1 second
- **Clippy**: `cargo clippy --no-default-features --features sync -- -D warnings` -- takes 10 seconds. NEVER CANCEL.
- **MegaLinter**: `npx mega-linter-runner --fix` -- requires Docker and NodeJS, may not work in all CI environments
- **ALWAYS run linting before committing** or CI (.github/workflows/mega-linter.yaml) will fail

### Examples and Hardware Requirements
- **Build Examples**: `cargo build --example power_data_object_query --no-default-features --features sync`
- **Run Examples**: `cargo run --example <name> --no-default-features --features sync`
- **HARDWARE REQUIRED**: Examples need FT232H Breakout Board + AP33772S breakout board
- **Expected Failure Without Hardware**: Examples will panic with "AllocationFailed" error - this is normal
  - Example: `cargo run --example power_data_object_query --no-default-features --features sync` will fail with "AllocationFailed"
- **Advanced Examples**: `cargo run --example startup_configuration --no-default-features --features "sync advanced"`
- **Available Examples**: power_data_object_query, power_data_object_send, power_data_object_switch, avs_cycle, avs_fixed, pps_cycle, serial_profile_on_off, state_query, startup_configuration

### Platform-Specific Examples
- **ESP32C3**: First install target: `rustup target add riscv32imac-unknown-none-elf`
  - Then: `cd examples/esp32c3 && cargo build` -- takes 52 seconds. NEVER CANCEL. Set timeout to 70+ minutes.
- **Raspberry Pi**: `cd examples/raspberrypi && cargo fmt --all -- --check`

## Validation Scenarios

**ALWAYS test these scenarios after making changes:**

1. **Basic Compilation Test**: Verify both sync and async modes compile
   - `cargo build --no-default-features --features sync`
   - `cargo build --no-default-features --features async`
2. **Feature Compatibility**: Test advanced feature combinations
   - `cargo build --no-default-features --features "sync advanced"`
   - `cargo build --no-default-features --features "async advanced"`
3. **Documentation Generation**: Ensure docs build without errors
   - `cargo doc --no-default-features --features sync`
4. **Test Suite Execution**: Run all tests to verify functionality
   - `cargo test --no-default-features --features sync`
5. **Example Compilation**: Verify examples build (hardware not required for build test)
   - `cargo build --example power_data_object_query --no-default-features --features sync`

## Feature Flags Understanding

- **sync** vs **async**: MUTUALLY EXCLUSIVE. Use sync for embedded-hal, async for embedded-hal-async
- **advanced**: Enables low-level register access and full device functionality
- **defmt**: Optional debugging feature for embedded systems
- **interrupts**: Enables interrupt pin support
- **Default**: sync feature is enabled by default

## Key Repository Structure

### Source Code Organization
- `src/lib.rs`: Main library entry point, feature flag handling
- `src/ap33772s.rs`: High-level driver API
- `src/commands/`: Low-level register commands (advanced feature)
- `src/communications/`: I2C communication layer
- `src/types.rs`: Type definitions and units

### Examples Directory
- `examples/*.rs`: Desktop sync examples using FT232H
- `examples/esp32c3/`: Async ESP32C3 example project
- `examples/raspberrypi/`: Raspberry Pi specific examples
- `examples/README.md`: Hardware setup instructions

### Testing and CI
- `tests/test_standard_api_formatting.rs`: Format display testing
- `.github/workflows/`: CI pipeline definitions
- `.mega-linter.yaml`: Linting configuration

## Common Commands Reference

### ls -la [repo-root]
```
.github/               # CI workflows and configurations
.vscode/               # VSCode settings and extensions
docs/                  # Datasheets and technical documentation
examples/              # Hardware examples and platform demos
src/                   # Main library source code
tests/                 # Integration and unit tests
utils/                 # Helper utilities for examples
Cargo.toml             # Main project configuration
DEVELOPMENT.md         # Development setup guide
README.md              # Project overview and quick start
```

### Cargo.toml Key Features
```toml
[features]
default = ["sync"]
sync = ["dep:embedded-hal", "maybe-async/is_sync"]
async = ["dep:embedded-hal-async"]
advanced = []  # Low-level register access
defmt = ["dep:defmt"]  # Embedded debugging
interrupts = []  # Interrupt pin support
```

## GitHub Actions Integration

- **continuous-build.yaml**: Tests compilation across platforms and features
- **continuous-test.yaml**: Runs cargo test suite
- **mega-linter.yaml**: Comprehensive code linting
- **release-plz.yaml**: Automated release management

## Hardware Setup Information

This driver is for the AP33772S USB-C Power Delivery chip. Development requires:
- **FT232H Breakout Board**: USB to I2C bridge for desktop development
- **AP33772S Breakout Board**: Available from CentyLab
- **USB-C PD Capable Charger**: For testing power delivery functionality

See `examples/README.md` and `docs/examples-dev-setup.drawio.png` for wiring diagrams.

## Troubleshooting

- **Link Error "cannot find -lftdi1"**: Install libftdi1 and libftdi1-dev
- **Examples panic with "AllocationFailed"**: Normal without hardware, indicates successful compilation
- **Async/Sync feature conflict**: Only enable one mode at a time
- **Documentation warnings about private links**: Expected behavior, use `--document-private-items` if needed
- **MegaLinter fails**: Requires Docker, may not work in all CI environments