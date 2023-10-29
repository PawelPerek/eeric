use crate::rv_core::instruction::executor::prelude::*;

pub fn s(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (fs2, _) = decompose(f[rs2]);

    x[rd] = if fs1 < fs2 { 1 } else { 0 };
}
