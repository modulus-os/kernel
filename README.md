# Modulus v0.1.9

64 bit operating system written in Rust

# Features

Here is a list of currently implemented features:

* 64-bit mode
* Display module
* Paging memory management system
* Exception handlers
* Keyboard support

# Building

## macOS

Install dependencies via Homebrew:

```
brew install nasm x86_64-elf-binutils i686-elf-grub xorriso qemu
```

You also need Rust nightly with the `x86_64-unknown-none` target:

```
rustup toolchain install nightly
rustup target add x86_64-unknown-none
```

## Build & Run

```
make          # build the kernel ELF binary
make iso      # build a GRUB-bootable ISO
make run      # build and launch in QEMU
make clean    # remove all build artifacts
```

![Screenshot](https://raw.githubusercontent.com/modulus-os/kernel/master/screenshot.png)
