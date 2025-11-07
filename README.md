# MultiScope Firmware

## Compile
### For hardware

ADCs enabled, all peripherals expected etc
``` console
$ cargo build -r
```

### For emulation through Renode

Emulation feature disables the ADCs which hang on Renode 
``` console
$ cargo build -r --features emulation
```

## Emulation

See: [Renode Docs](https://renode.readthedocs.io/en/latest/introduction/using.html)

Use Renode to emulate the environment. The board/periphral layout is descried in ``boardDesc.repl``.
Run the script ``stm32f4_discovery.resc`` using the Renode Monitor console to initialize the simulation.
```console
$ include (path to project)/stm32f4_discovery.resc
```
This will create a predefined sim environment. Doing so will open a monitor of the UART4 bus, which is used for debugging / logging.
The simulation must then be started using ``start`` and paused with the ``pause`` commmand in the Renode Monitor.

The peripheral gpioPortB.UserButton can be used to interact with a GPIO pin. To press it, use the following command:
```console
$ gpioPortB.Userbutton Press
```
The ``Press`` token can be changed for ``Release``, to untoggle the button, or ``Pressed`` to show the state of the button.

### Behavior as of 11/7/25

When unpressed, the ``Uart4`` bus will echo any incoming bytes. When pressed, the ``Uart4`` bus will append a "hello world" line to every message.




# 'OLD:'

# `stm32-template`

> A template for building applications for STM32 microcontrollers

## Dependencies

To build embedded programs using this template you'll need:

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/cargo-generate/cargo-generate#installation).
``` console
$ cargo install cargo-generate
```

- Flash and run/debug tools:
``` console
$ cargo install probe-rs --features cli
```

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:
  
``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Instantiate the template.

1. Run and enter project name
``` console
$ cargo generate --git https://github.com/burrbull/stm32-template/
 Project Name: app
```

2. Specify **chip product name** and answer on several other guide questions.

3. Your program is ready to compile:
``` console
$ cargo build --release
```

## Flash and run/debug

You can flash your firmware using one of those tools:

- `cargo flash --release` — just flash
- `cargo run --release` — flash and run using `probe-rs run` runner or `probe-run` runner (deprecated) which you can set in `.cargo/config.toml`
- `cargo embed --release` — multifunctional tool for flash and debug

You also can debug your firmware on device from VS Code with [probe-rs](https://probe.rs/docs/tools/vscode/) extention or with `probe-rs gdb` command.
You will need SVD specification for your chip for this. You can load patched SVD files [here](https://stm32-rs.github.io/stm32-rs/).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
