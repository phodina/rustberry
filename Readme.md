# Rust kernel for Raspberry Pi 3

Evolving kernel for Raspberry Pi 3 written in Rust.

## Build

To build simply type:
```shell
$ cargo xbuild
```

## Run in QEMU

To run in QEMU type:
```shell
$ qemu-system-aarch64 -kernel target/aarch64-unknown-none/debug/rustberry -M raspi3 -serial null -serial mon:stdio
```
