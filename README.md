# RISC-V Simulation Suite
A comprehensive suite for RISC-V simulation, encompassing core, interpreter, and web application components. This monorepo hosts three primary projects: eeric-core, eeric-interpreter, and a reference webapp using those libraries.

# Projects
## libs/eeric-core
An embeddable RISC-V core designed with WASM compilation in mind, supporting IMFDV extensions. It acts as an abstract back-end machine for RISC-V simulation.

## libs/eeric-interpreter
A library that bridges textual input to eeric-core abstract types, easing the integration of user input into the RISC-V simulation in browser.

## apps/webapp
A Rust-Leptos web application compiled into WebAssembly, leveraging eeric-core and eeric-interpreter to provide an intuitive RISC-V simulator in a browser.

# Building
Both core and interpreter libraries can be built just by running `cargo build`. Webapp requires additional TypeScript bridge build, refer to it's README or .github/workflows file for instructions.

# Hosting

Thanks to back-endless nature of this project, it's possible to serve it on the static file server. Currently it's hosted right here: https://pawelperek.github.io/eeric/

# Future Work
Refer to the project's roadmap for upcoming features and enhancements.