use crate::rv_core::instruction::executor::prelude::*;

pub fn bgeu(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, pc: &mut u64) {
    if x[rs1] >= x[rs2] {
        *pc = pc.wrapping_add(imm12 as u64).wrapping_sub(4);
    }
}
