# Zephyr OS

## Introduction

A basic operating system built using `rust`.

## Getting Started

### Prerequisites

- [rust](https://www.rust-lang.org/tools/install)
- [qemu](https://www.qemu.org/download/)

Make sure you have installed `rust` and `qemu` before proceeding.

You need `nightly` version of `rust` to build this project. You can install `nightly` version of `rust` using the following command:

```bash
rustup toolchain install nightly
```

### Building

To build the project, run the following command:

```bash
cargo build
```

### Running

To run the project, run the following command:

```bash
cargo run --bin qemu-bios  # For running in bios mode
cargo run --bin qemu-uefi  # For running in uefi mode
```

### Miscellaneous

```bash
cargo clean  # To clean the project
cargo fmt --all  # To format the project
cargo clippy --all -- -D warnings  # To check for warnings
pre-commit run --all-files  # To run pre-commit checks
```
