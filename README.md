[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](MIT)
[![License: APACHE2.0](https://img.shields.io/badge/License-APACHE2.0-blue?style=flat-square)](APACHE2.0)

## N64 Project Template for Rust
This repo is intended to be a starting point for developing software for the Nintendo 64 console using Rust.

Only the bare minimum utilities are included at this time. A `no_std` allocator has been set up for you (which should
ideally be replaced in the future by an allocator that is more aware of the N64's memory map.)

The `n64-pac` crate is included as a dependency. It doesn't provide full coverage of all registers yet, but
has enough for the basics. Refer to [the crate's docs](https://docs.rs/n64-pac) for details.

An ISViewer implementation is available, with `print!` and `println!` macros. This allows you to print text to emulators
that support the ISViewer, such as [Ares](https://ares-emu.net/).

## Building
1. Install Rust: https://www.rust-lang.org/tools/install
2. Get the source: (e.g. using git, or downloading an archive manually)
```
git clone https://github.com/rust-n64/n64-project-template
cd n64-project-template
```
3. Install a cargo runner: `cargo install nust64`
4. Run `cargo run --release` to compile and build a ROM.

This template uses [libdragon's open-source IPL3](https://github.com/DragonMinded/libdragon/tree/unstable/boot) by default.
You do not need to supply your own.

## Cargo Runner Configuration
This project is configured to make use of [nust64](https://github.com/rust-n64/nust64), a program that creates the final
N64 ROM after cargo finishes compiling the project. `nust64` provides support for executing additional commands after
the ROM has been created.

Refer to `.cargo/config.toml` for examples.

## License
The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can chose either the MIT licence or the
Apache-2.0 licence when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific licence.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.