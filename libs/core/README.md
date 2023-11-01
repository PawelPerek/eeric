
# eeric
An Easily Embeddable RISC-V Core

# Overview
eeric is a RV64I core supporting Zicsr, M, F, D, and V extensions, designed primarily for WASM compilation, although it works with any cdylib target. It acts as an abstract back-end machine requiring a front-end compiler or interpreter (see libs/eeric-interpreter).

# Example
The vectorized memcpy algorithm from RISCV Vector Spec examples is represented in `eeric_core` as shown below:

```rust
use eeric_core::prelude::*;

fn main() {
    let mut core = RvCore::with_instructions(vec![
        I::Vsetvli(F::Vsetvli {
            rd: T0,
            rs1: A2,
            vtypei: 0b_1_1_000_011,
        }),
        I::Vlv {
            eew: 8,
            data: F::Vl {
                vd: 0,
                rs1: A1,
                vm: false,
            },
        },
        I::Add(F::R {
            rd: A1,
            rs1: A1,
            rs2: T0,
        }),
        I::Sub(F::R {
            rd: A2,
            rs1: A2,
            rs2: T0,
        }),
        I::Vsv {
            eew: 8,
            data: F::Vs {
                vs3: 0,
                rs1: A3,
                vm: false,
            },
        },
        I::Add(F::R {
            rd: A3,
            rs1: A3,
            rs2: T0,
        }),
        I::Bne(F::S {
            rs1: A2,
            rs2: ZERO,
            imm12: -24,
        }),
        I::Jalr(F::I {
            rd: ZERO,
            rs1: RA,
            imm12: 0,
        }),
    ]);

    for machine_state in core.run() {
        println!("{:?}", machine_state);
    }
}
```