use crate::rv_core::instruction::executor::prelude::*;

pub fn slliw(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    let rs1 = x[rs1];
    let shamt = imm12 & 0b11111;
    x[rd] = (rs1 << shamt) as i32 as u64;
}
