#!/usr/bin/env bash


cargo build --release --target=armv7-unknown-linux-gnueabihf
scp target/armv7-unknown-linux-gnueabihf/release/bela-i2c root@bela.local:~