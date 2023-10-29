use crate::rv_core::instruction::executor::prelude::*;

pub fn addiw(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters) {
    x[rd] = (x[rs1] as i32).wrapping_add(imm12) as u64;
}
