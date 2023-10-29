use eeric_core::prelude::*;

use format as F;
use Instruction as I;

// Example:
// loop:
//    vsetvli t0, a2, e8, m8, ta, ma   # Vectors of 8b
//    vle8.v v0, (a1)               # Load bytes
//      add a1, a1, t0              # Bump pointer
//      sub a2, a2, t0              # Decrement count
//    vse8.v v0, (a3)               # Store bytes
//      add a3, a3, t0              # Bump pointer
//      bnez a2, loop               # Any more?
//      ret

fn main() {
    // Important note: eeric as low-level back-end abstraction layer does not support pseudo-instructions
    // Burden of decoding pseudo-instructions is on the front-end layer
    // E.G: ret == I::Jalr (F::I { rd: ZERO, rs1: RA, imm: 0 }),

    use alias::*;

    let mut core = RvCoreBuilder::default()
        .instructions(vec![
            I::Vsetvli(F::Vsetvli {
                rd: T0,
                rs1: A2,
                vtypei: 0b_1_1_000_011,
            }),
            I::Vlv {
                eew: BaseSew::E8,
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
                eew: BaseSew::E8,
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
        ])
        .build();

    for machine_state in core.run() {
        println!("{:?}", machine_state);
    }
}
