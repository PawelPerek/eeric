use crate::rv_core::instruction::executor::prelude::*;

pub fn addw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = (x[rs1] as i32).wrapping_add(x[rs2] as i32) as u64;
}
