use crate::rv_core::instruction::executor::prelude::*;

pub fn xori(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] ^ imm12 as u64;
}
