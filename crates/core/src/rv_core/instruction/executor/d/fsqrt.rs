use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2: _ }: R, f: &mut FloatRegisters) {
    f[rd] = f[rs1].sqrt();
}
