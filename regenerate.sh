#!/bin/bash

svd2rust -i EFM32PG1B.svd -c svd2rust.toml
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt