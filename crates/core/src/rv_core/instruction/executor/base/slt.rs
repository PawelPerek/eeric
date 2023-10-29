use crate::rv_core::instruction::executor::prelude::*;

pub fn slt(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let rs1 = x[rs1] as i64;
    let rs2 = x[rs2] as i64;
    x[rd] = if rs1 < rs2 { 1 } else { 0 };
}
