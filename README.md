![build](https://circleci.com/gh/modulus-os/kernel.svg?style=shield&circle-token=:circle-token) ![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)

# Modulus v0.1.9

Modulus is a 64 bit operating system written in Rust which aims to be flexible and modular. It is currently in a very early stage of development.

# Features

Here is a list of currently implemented features:

* 64-bit mode
* Display module
* Paging memory management system
* Exception handlers
* Keyboard support

# Building

Modulus uses Waf as its build system. To build for the first time, execute

`./waf configure build`

This will configure and build the project. After the initial configuration, you will just need to run `./waf build` after making changes.

To run in QEMU, execute `./waf build -q`

To run in Bochs, execute `./waf build -b`

![Screenshot](https://raw.githubusercontent.com/modulus-os/kernel/master/screenshot.png)
