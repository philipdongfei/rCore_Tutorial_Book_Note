#!/bin/bash
cargo build --release
make target/riscv64gc-unknown-none-elf/release/os.bin
make gdbserver

