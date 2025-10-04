Ferris Weather
==============

A demo app to try building a networked Rust application for PPC classic Mac OS.
This application fetches some JSON from my server, parses it with serde and
shows an alert with the temperature (the file on the server doesn't change).

![Screenshot of Ferris Weather](Ferris%20Weather.png)

Building
--------

**Note:** these steps have only been tested on Linux (Arch).

### Prerequisites

1. [My fork of the Retro68 with updated binutils][Retro68] (`binutils` branch)
   built and on `$PATH`. E.g. `export PATH=$PATH:/home/you/path/to/Retro68-build/toolchain/bin`
2. `rustup`
3. Docker (if not on Debian based system) or `binutils-powerpc-linux-gnu` on
   Debian based systems For non-Debian systems build the binutils docker image:
   `(cd powerpc-binutils && docker build -t binutils-objcopy .)`

### Prepare

```
mkdir build
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=path/to/Retro68-build/toolchain/powerpc-apple-macos/cmake/retroppc.toolchain.cmake
cd ..
```

### Build

Compile the Rust code with:

```
cargo build --release -Z build-std=core,alloc --target powerpc-apple-macos.json
```

Convert the static library to XCOFF, either via Docker or `powerpc-linux-gnu-objcopy`
directly:

```
docker run --rm -it -v $(pwd):/src binutils-objcopy -O aixcoff-rs6000 /src/target/powerpc-apple-macos/release/libclassic_weather.a /src/target/powerpc-apple-macos/release/libclassic_weather.obj
```

Build the C code, link in the Rust code, and produce the final binary:

```
cmake --build build --target FerrisWeather_APPL
```

### Running

`FerrisWeather.bin` is the MacBinary encoded application, you can copy this to a
machine or emulator to run it. Retro68 provides the `LaunchAAPL` tool, which
combined with its server counterpart makes this very easy. With the server
running on a machine or emulator you can launch the binary directly from you
host with:

```
LaunchAPPL -e tcp --tcp-address 127.0.0.1 build/FerrisWeather.bin
```

### Notes

The `rust-toolchain.toml` captures the nightly Rust version I used. More recent
changes to the Rust compiler mean versions after `nightly-2023-08-02` don't
work. See [this issue](https://github.com/wezm/ferris-weather/issues/1) for
more details.

[Retro68]: https://github.com/wezm/Retro68/tree/binutils
