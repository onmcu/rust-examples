# rust-examples

Get started with [OnMCU](https://onmcu.com) in minutes. These are minimal per-board Rust examples that count upwards and stream the count over **RTT** on real hardware in the cloud.

OnMCU can plug straight into the standard Rust toolchain as a Cargo *runner*, so flashing and running firmware on a physical MCU is just:

```sh
cd nucleo-h743zi # or any other folder
cargo run
```

That builds the firmware, uploads it, runs it on a real NUCLEO-H743ZI, streams the RTT log back to your terminal, and exits `0` on success, so it drops into CI and `cargo`-based workflows like any ordinary binary.

To keep things separated, there is one crate per board. Each crate is built and run from its own directory so its `.cargo/config.toml` selects the right target, linker arguments and `onmcu` board.

## Prerequisites

1. **Rust**: install via [rustup](https://rustup.rs).
2. **Toolchain & targets**: handled automatically: the workspace
   `rust-toolchain.toml` pins the `stable` channel and the two Cortex-M targets
   (`thumbv7em-none-eabihf`, `thumbv7em-none-eabi`), which rustup installs on
   your first `cargo` invocation here. Nothing to do by hand.
3. **OnMCU CLI**: install the `onmcu` binary, then authenticate once:

   ```sh
   onmcu login            # stores your API key in the OS keyring
   onmcu list-boards      # confirm which boards are available
   ```

   Create your API Key under [app.onmcu.com/settings](https://app.onmcu.com/settings). See [github.com/onmcu/onmcu-rs](https://github.com/onmcu/onmcu-rs) for more details.
4. **Happy testing!**

## Boards

| Crate | Board | MCU | Target | Approach |
|-------|-------|-----|--------|----------|
| `nucleo-h743zi` | NUCLEO-H743ZI | STM32H743ZIT6 | `thumbv7em-none-eabihf` | `embassy-stm32` |
| `nucleo-f401re` | NUCLEO-F401RE | STM32F401RET6 | `thumbv7em-none-eabihf` | `embassy-stm32` |
| `nucleo-wl55jc` | NUCLEO-WL55JC | STM32WL55JC (CM4) | `thumbv7em-none-eabi` | `embassy-stm32` |
| `nucleo-f439zi` | NUCLEO-F439ZI | STM32F439ZIT6 | `thumbv7em-none-eabihf` | `embassy-stm32` |
| `nucleo-wb55rg` | NUCLEO-WB55RG | STM32WB55RGV6 | `thumbv7em-none-eabihf` | `embassy-stm32` |
| `lp-cc1312r7` | LP-CC1312R7 | CC1312R7 | `thumbv7em-none-eabihf` | bare-metal PAC 


## Build and run

Always execute from inside a single crate directory, do **not** run `cargo build --workspace`, as the `embassy-stm32` chip-selection features are mutually exclusive and only resolve one crate at a time.

```sh
cd nucleo-h743zi
cargo run                # compile + flash + run on OnMCU, returns 0 on success
```

`cargo run` invokes the `runner` set in each crate's `.cargo/config.toml` (`onmcu run --board <BOARD> --file`); Cargo appends the freshly linked ELF path as the final argument, so it becomes the value of `--file`.

## Exit signalling

Each binary counts `0..=10` over RTT (`defmt`), then signals success to the host via an ARM semihosting `SYS_EXIT_EXTENDED` call so `onmcu run` terminates with exit code `0`. This is done by calling `semihosting::process::exit(0)`. The semihosting trap is serviced by the debugger that OnMCU attaches, so the run ends cleanly with code `0`. A non-zero code would signal failure.
