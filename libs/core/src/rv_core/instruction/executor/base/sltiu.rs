use crate::rv_core::instruction::executor::prelude::*;

pub fn sltiu(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    let rs = x[rs1] as i64;
    x[rd] = if rs < imm12 as i64 { 1 } else { 0 };
}
