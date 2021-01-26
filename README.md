# `bela-i2c`

i2c communication to Eurorack (eg. ER301, Teletype, etc) for [Bela](https://bela.io/) written in Rust, inspired by [hans](https://llllllll.co/t/hans/36455/14), currently in progress...


# Quick Start (re-write later)
```
# run bela-osc
cd ~/bela-i2c
make -C ~/Bela EXAMPLE=Communication/OSC run CL="-p16"OA

# run rust-osc.
cargo run
```

------------------------------------
below is deprecated need to rewrite.
# Setup

Install the toolchain (the Beaglebone black):

```sh
rustup target add armv7-unknown-linux-gnueabihf
rustup toolchain install stable-armv7-unknown-linux-gnueabihf
```

for cross-compiling (OSX -> Debian) we need to get right linker
with a bela board plugged in and accessible at `bela.local`, run:

```sh
./bela_setup_local.sh
```

This will do the jobs, by pulling in some required files from the board,
and sets up the `$PATH` environment variable and setup `.cargo/config`. This MUST be called in each
terminal session that will be used to call `cargo`, but will only download the
files once.

# Building

## Quick building

```
sh ./build.sh

# on the bela board
./bela-i2c

# on another terminal window.
make -C ~/Bela EXAMPLE=Communication/OSC run CL="-p16"
# or compiled by Bela IDE.
```

## Manual

```
cargo build --release --target=armv7-unknown-linux-gnueabihf
scp target/armv7-unknown-linux-gnueabihf/release/bela-i2c root@bela.local:~
ssh root@bela.local
# ...
# on the bela board
./bela-i2c

# on another terminal window.
make -C ~/Bela EXAMPLE=Communication/OSC run CL="-p16"
# or compiled by Bela IDE.
```

# [Optional] send data via WIFI ( eg. use TouchOSC (Mobile) -> Bela )

<img src="images/bela-setup-ip.png?sanitize=true">

- setup WIFI on Bela first, details [here](https://learn.bela.io/using-bela/bela-techniques/connecting-to-wifi/).
- make sure you're in same WIFI as Bela.
- on Bela
  - get wlan IP by `ip a` on Bela board.
- on TouchOSC:
  - set HOST to IP obtained by previous command (eg. `192.168.1.115` as shown above).
  - set Outgoing to `7562` (default Bela's listening port).

# Notes

Bela i2c pin requires 3.3V, for safety we might need [ logic level converter ](https://shopee.co.th/search?keyword=logic%20level%20converter%20%E0%B9%80%E0%B8%84%E0%B8%A3%E0%B8%B7%E0%B9%88%E0%B8%AD%E0%B8%87%E0%B8%A1%E0%B8%B7%E0%B8%AD%E0%B9%84%E0%B8%9F%E0%B8%9F%E0%B9%89%E0%B8%B2%E0%B9%81%E0%B8%A5%E0%B8%B0%E0%B9%80%E0%B8%84%E0%B8%A3%E0%B8%B7%E0%B9%88%E0%B8%AD%E0%B8%87%E0%B8%A1%E0%B8%B7%E0%B8%AD%E0%B8%8A%E0%B9%88%E0%B8%B2%E0%B8%87&showItems=true) to use with Eurorack level (5V).

# Disclaimer

this have been successfully tested on
Bela image: v0.3.2, (released 13 March 2018).
OSX: 10.13.6 (High Sierra)
MacBook Pro (Retina, 13-inch, Early 2013)

# Licence

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Resources

- [bela-sys](https://github.com/padenot/bela-sys)
- [hans](https://github.com/nordseele/hans_rust)
