# eeric

An Easily Embeddable RIsc-v Core

# Design

eeric is a RV64I core with support for Zicsr, M, F, D and V extensions. I designed it with following design goals in mind:

- It's designed with WASM compilation in mind (although any cdylib target should work as well)
- It doesn't support interrupts
- It's single threaded, hence no A extension support
- It's not designed to be most performant emulator, but it should be reasonably fast
- It's meant to be an abstract back-end machine, so it needs a front-end compiler or interpreter to work (see https://github.com/PawelPerek/eeric-interpreter)

# Example

Let's consider following RISC-V Vector Algorithm from [RISCV Vector Spec examples](https://github.com/riscv/riscv-v-spec):

```
loop:
   vsetvli t0, a2, e8, m8, ta, ma   # Vectors of 8b
   vle8.v v0, (a1)               # Load bytes
     add a1, a1, t0              # Bump pointer
     sub a2, a2, t0              # Decrement count
   vse8.v v0, (a3)               # Store bytes
     add a3, a3, t0              # Bump pointer
     bnez a2, loop               # Any more?
     ret
```

It can be expressed as following eeric core:
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

# Roadmap

See https://github.com/PawelPerek/eeric/issues
