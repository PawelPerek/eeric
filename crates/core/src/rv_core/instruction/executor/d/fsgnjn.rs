use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    f[rd] = f[rs1].copysign(-f[rs2]);
}
