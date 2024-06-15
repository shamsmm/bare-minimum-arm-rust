# Bare Minimum Rust Project To Get Started with ARM
## What is this
This repository contains the bare minimum to get started with the STM32F103XX microcontroller rust programming, which features a Cortex-M3 ARM processor.

This project relies on cargo, the rust package manager, to build the project. It was configured to cross compile the code to the ARM architecture, and contains a custom linker code.

## Motivation
Rust is a very promising programming language, and it is very well suited for embedded systems programming. This project aims to provide a simple way to get started with rust programming on ARM microcontrollers.

## Repo Branches
This repo may contain many branches as templates or branches that uses crates and so.

## How to build
In my other repos featuring `Embedded-C` programming language there was a `Makefile` to automate the build process. But in this repo, I am using `cargo` to build the project. So, to build the project, you need to run the following command:

```bash
cargo build
```

This command will build the project and generate the binary file in the `target/thumbv7m-none-eabi/debug/` directory.

## How to flash
To flash the binary file to the microcontroller, you can use the `openocd` tool. You can install it using the following command:

```bash
openocd -f stlink.cfg -c "program target/thumbv7m-none-eabi/debug/rust0 verify reset exit"
```