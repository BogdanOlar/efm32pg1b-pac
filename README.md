# EFM32PG1B PAC

Low-level register mappings for the [Silicon Labs EFM32PG1B] family of ARM Cortex-M4 microcontrollers, written in Rust.
The code is generated automatically from a vendor-supplied SVD file, using [svd2rust](https://docs.rs/svd2rust).

The purpose of this crate is to give embedded programs or libraries written Rust access to the complete functionality
of EFM32PG1B MCUs.

This PAC is closely based on Timo Kröger's [PAC for the efm32pg12](https://github.com/timokroeger/efm32pg12-pac) microcontrollers.

## Documentation

SVD files are available in the [EFM32PG1B CMSIS-Pack](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.EFM32PG1B_DFP.5.8.2.pack).


Additional vendor supplied documents:
- [Datasheet](https://www.silabs.com/documents/public/data-sheets/efm32pg1-datasheet.pdf)
- [Reference Manual](https://www.silabs.com/documents/public/reference-manuals/EFM32PG1-ReferenceManual.pdf)
- [Errata](https://www.silabs.com/documents/public/errata/efm32pg1-errata.pdf)

## License

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
