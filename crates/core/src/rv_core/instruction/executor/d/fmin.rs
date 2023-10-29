use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    f[rd] = if f[rs1] < f[rs2] { f[rs1] } else { f[rs2] };
}
