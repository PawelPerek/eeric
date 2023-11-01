# RISC-V Simulator WebApp
A reference eeric front-end app

# Overview
This webapp is a RISC-V simulator web application built with Rust and Leptos, compiled into WebAssembly (WASM). Utilizing eeric-core and eeric-interpreter, it provides an intuitive interface for simulating RISC-V algorithms right in your browser.

# Features
 - RISC-V Simulation: Accurately simulate RISC-V operations with the power of eeric-core.
 - Easy-to-Use Interface: A user-friendly interface for interaction and simulation control.
 - Vector instructions support

# Build
Webapp requires Typescript bridge to be built before WASM step. To build bridge run
`npm run build:ts:release`, and then `npm run build:rs:watch`.

Note: Trunk.rs which is used for bundling WASM app works much better on Linux and MacOS systems (WSL included) than Windows.
