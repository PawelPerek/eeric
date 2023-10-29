use crate::rv_core::instruction::executor::prelude::*;

pub fn srai(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    let rs1 = x[rs1] as i64;
    let shamt = imm12 & 0b11111;
    x[rd] = (rs1 >> shamt) as u64;
}
