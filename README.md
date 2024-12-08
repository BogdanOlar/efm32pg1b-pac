# EFM32PG1B Peripheral Access Crate (PAC)

[![crates.io](https://img.shields.io/crates/v/efm32pg1b-pac)](https://crates.io/crates/efm32pg1b-pac)
[![docs.rs](https://docs.rs/efm32pg1b-pac/badge.svg)](https://docs.rs/efm32pg1b-pac)

Low-level register mappings for the [Silicon Labs EFM32PG1B](https://www.silabs.com/mcu/32-bit/efm32-pearl-gecko/device.EFM32PG1B200F256GM48) family of ARM Cortex-M4 microcontrollers, written in Rust.
The code is generated automatically from a vendor-supplied SVD file, using [svd2rust](https://docs.rs/svd2rust) v0.33.0:

```sh
svd2rust -i EFM32PG1B.svd -c svd2rust.toml
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo +nightly fmt
```

The purpose of this crate is to give embedded programs or libraries written Rust access to the complete functionality
of EFM32PG1B MCUs.

The SVD file used is based on all the files in the [EFM32PG1B CMSIS-Pack](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.EFM32PG1B_DFP.5.8.2.pack). The only differences between them are the `<name>`, `<description>` and `<flashSize>` field values. Here are the flash sizes for each MCU:

| EFM32PG1B   | flashSize |
| ----------- | :-------: |
| `100F128GM32` | 0x20000  |
| `100F128IM32` | 0x20000  |
| `100F256GM32` | 0x40000  |
| `100F256IM32` | 0x40000  |
| `200F128GM32` | 0x20000  |
| `200F128GM48` | 0x20000  |
| `200F128IM32` | 0x20000  |
| `200F256GM32` | 0x40000  |
| `200F256GM48` | 0x40000  |
| `200F256IM32` | 0x40000  |
| `200F256IM48` | 0x40000  |

The `flashSize` value used in the source `EFM32PG1B.svd` file was set to the lowest value (`0x20000`).

The `ASYNC` (Asynchronous Reflex) field for all Channel Control Registers of the Clock Management Unit has been renamed to `ASYNCREFL` because `svd2rust` converted it to `async`, which is a rust keyword, and caused compilation errors.

## Documentation

SVD files are available in the [EFM32PG1B CMSIS-Pack](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.EFM32PG1B_DFP.5.8.2.pack).


Additional vendor supplied documents:
- [Datasheet](https://www.silabs.com/documents/public/data-sheets/efm32pg1-datasheet.pdf)
- [Reference Manual](https://www.silabs.com/documents/public/reference-manuals/EFM32PG1-ReferenceManual.pdf)
- [Errata](https://www.silabs.com/documents/public/errata/efm32pg1-errata.pdf)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
