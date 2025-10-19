
# Rust STM32 嵌入式模板 - Blue Pill (STM32F103C8T6)

此仓库作为一个 **模板**，用于 STM32 微控制器的 Rust 嵌入式开发，特别针对 STM32F103C8T6 (Blue Pill) 板进行了预配置。它提供了即用型设置，集成了 Knurling-rs 工具，包括用于调试的 `probe-rs`、用于日志记录的 `defmt`，以及用于内存优化的 `flip-link`。您可以通过从中生成新仓库来作为自己的 STM32 项目起点。

> 快速设置一个针对 STM32F103C8T6 (Blue Pill) 的 [`probe-rs`] + [`defmt`] + [`flip-link`] 嵌入式项目，包含 LED 闪烁示例。

[`probe-rs`]: https://crates.io/crates/probe-rs  
[`defmt`]: https://github.com/knurling-rs/defmt  
[`flip-link`]: https://github.com/knurling-rs/flip-link  

## 为什么使用这个模板？

- **模板特性**：这是一个与 `cargo-generate` 兼容的模板设计。您可以 fork 或克隆它，然后生成新项目来自定义占位符（例如项目名称、作者）。它包含示例二进制文件、测试，以及基本的 LED 闪烁 main.rs 以启动开发。
- **No_std 与 Cortex-M 运行时**
- **通过 RTT 的 Defmt 日志记录**
- **Panic 探针用于诊断**
- **预配置 STM32F1xx HAL (stm32f103 特性)**
- **`src/main.rs` 中的简单 LED 闪烁 (PC13 板载 LED)**
- **VSCode launch/tasks 用于 probe-rs 调试**
- **Blue Pill 的内存布局 (64KB Flash, 20KB RAM)**
- **`src/bin/` 中的多个示例二进制文件，用于常见嵌入式模式（例如位字段、panic 处理）**

## 依赖项

1. **flip-link**: `cargo install flip-link`
2. **probe-rs**: 通过 https://probe.rs/docs/getting-started/installation/ 安装
3. **cargo-generate** (用于此模板): `cargo install cargo-generate`

## 作为模板的设置

### 1. 作为模板源使用

要从此模板创建新项目：

```bash
cargo generate \
    --git https://github.com/your-username/app-template.git \
    --branch main \
    --name my-stm32-app
```

这将：
- 替换占位符，如 `{{project-name}}` (变为 "my-stm32-app") 和 `{{crate_name}}` (变为 "my_stm32_app")。
- 生成自定义的 Cargo.toml、README.md 和其他文件。

### 2. 生成后自定义 (TODOs)

生成或克隆后，处理这些模板 TODOs：

- **Cargo.toml**: 
  - 如果不使用 `cargo-generate`，更新 `authors` 和 `name`。
  - 如果更改 MCU，调整 HAL 依赖（例如，将 `stm32f1xx-hal` 替换为 F4 系列的 `stm32f4xx-hal`）。
- **src/lib.rs**: 取消注释并调整 HAL 导入：`use stm32f1xx_hal as _;`。对于其他 MCU：`use your-hal as _;`。
- **.cargo/config.toml**: 为您的芯片设置（使用 `probe-rs chip list`）。默认是 STM32F103C8Tx 和 thumbv7m-none-eabi 目标。运行 `rustup target add thumbv7m-none-eabi`。
- **memory.x**: 为您的板自定义 FLASH/RAM 区域。

如果切换 MCU：
- 在 config.toml 中更新芯片。
- 在 Cargo.toml 和 lib.rs 中交换 HAL。
- 调整 memory.x。

### 3. 构建和运行

- `cargo build` (从 src/main.rs 构建 "main" 二进制文件)。
- `cargo run --bin main` (通过 probe-rs 闪存并运行 LED 闪烁)。
- 在 VSCode 中：使用 "Build main (debug)" 任务，然后 "Debug STM32F103C8T6 (main)" 启动。

通过 ST-Link V2 (或如果启用了引导加载程序则通过 USB) 连接 Blue Pill。defmt 日志通过 probe-rs 或 RTT 查看器显示（例如，`probe-rs tool --chip STM32F103C8Tx rtt`）。

### 4. 示例二进制文件

`src/bin/` 包含教育性示例：
- `hello.rs`: 通过 defmt 打印 "Hello, world!"。
- `bitfield.rs`、`format.rs`、`levels.rs`、`overflow.rs`、`panic.rs`: 演示常见嵌入式 Rust 概念。

使用 `cargo build --bin <name>` 构建。

### 5. 测试

- 主机: `cargo test`
- 目标: `cargo test --target thumbv7m-none-eabi`

`src/lib.rs` 中的单元测试；`tests/integration.rs` 中的集成测试。

如果闪存/RTT 问题：设置 `DEFMT_RTT_BUFFER_SIZE=512 cargo run` 以减少缓冲区。

## 运行 LED 闪烁示例

`src/main.rs` 以 8MHz 默认时钟每 ~1s 闪烁 PC13 LED (低电平有效)。通过 defmt 输出: "LED ON/OFF"。

## 扩展模板

- 通过扩展 `main.rs` 添加中断、定时器或外设等功能。
- 对于高级项目，集成 Embassy (异步) 或通过 `stm32-usbd` 添加 USB 支持。
- 更多信息请参阅 [Rust Embedded Book](https://docs.rust-embedded.org/book/)。

## 支持

基于 Knurling-rs app-template，为 STM32 Blue Pill 自定义。对于问题，请检查 [Rust Embedded](https://docs.rust-embedded.org/book/)。

## 许可

Apache-2.0 或 MIT (请参阅 LICENSE-* 文件)。
