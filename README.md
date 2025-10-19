# STM32F103C8T6 (Blue Pill) Embedded Rust Template

> Quickly set up a [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project for STM32F103C8T6 (Blue Pill) with LED blink example.

[`probe-rs`]: https://crates.io/crates/probe-rs  
[`defmt`]: https://github.com/knurling-rs/defmt  
[`flip-link`]: https://github.com/knurling-rs/flip-link  

## Features
- No_std with Cortex-M runtime
- Defmt logging via RTT
- Panic probe for diagnostics
- Pre-configured for STM32F1xx HAL (stm32f103 feature)
- Simple LED blink in `src/main.rs` (PC13 onboard LED)
- VSCode launch/tasks for probe-rs debugging
- Memory layout for Blue Pill (64KB Flash, 20KB RAM)

## Dependencies
1. **flip-link**: `cargo install flip-link`
2. **probe-rs**: Install via https://probe.rs/docs/getting-started/installation/
3. **cargo-generate** (for using this template): `cargo install cargo-generate`

## Setup
### 1. Generate the Project
```bash
cargo generate \
    --git https://github.com/your-repo/app-template \
    --branch main \
    --name my-stm32-app
```
This replaces placeholders like `{{project-name}}` (becomes "my-stm32-app") and `{{crate_name}}` (becomes "my_stm32_app").

### 2. Post-Generation Steps
After generation, check the TODOs in files:

- **Cargo.toml**: Update `authors`. The HAL is pre-set for STM32F103; adjust if needed (e.g., features = ["stm32f103"]).
- **src/lib.rs**: Uncomment and adjust HAL import: `use stm32f1xx_hal as _;`
- **.cargo/config.toml**: Already set for STM32F103C8Tx chip and thumbv7m-none-eabi target. Run `rustup target add thumbv7m-none-eabi`.
- **memory.x**: Pre-configured for Blue Pill.

If changing MCU:
- Update chip in config.toml (use `probe-rs chip list`).
- Swap HAL in Cargo.toml and lib.rs.
- Adjust memory.x regions.

### 3. Build and Run
- `cargo build` (builds "main" binary from src/main.rs).
- `cargo run --bin main` (flashes via probe-rs and runs LED blink).
- In VSCode: Use "Build main (debug)" task, then "Debug STM32F103C8T6 (main)" launch.

Connect Blue Pill via ST-Link V2 (or USB if bootloader-enabled). Logs via defmt show in probe-rs or RTT viewer (e.g., `probe-rs tool --chip STM32F103C8Tx rtt`).

### 4. Example Binaries
src/bin/ contains examples like hello.rs (prints "Hello, world!" via defmt). Build with `cargo build --bin hello`.

### 5. Testing
- Host: `cargo test`
- Target: `cargo test --target thumbv7m-none-eabi`
Unit tests in src/lib.rs; integration in tests/integration.rs.

If flash/RTT issues: Set `DEFMT_RTT_BUFFER_SIZE=512 cargo run` to reduce buffer.

## Running the LED Blink Example
src/main.rs blinks PC13 LED (active low) every ~1s at 8MHz default clock. Outputs via defmt: "LED ON/OFF".

## Support
Based on Knurling-rs app-template, customized for STM32 Blue Pill. For issues, see [Rust Embedded](https://docs.rust-embedded.org/book/).

## License
Apache-2.0 or MIT (see LICENSE-* files).
