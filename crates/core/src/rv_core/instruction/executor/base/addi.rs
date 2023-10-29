use crate::rv_core::instruction::executor::prelude::*;

pub fn addi(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = x[rs1].wrapping_add(imm12 as u64);
}
