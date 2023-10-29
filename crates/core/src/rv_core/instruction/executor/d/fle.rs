use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = if f[rs1] <= f[rs2] { 1 } else { 0 };
}
