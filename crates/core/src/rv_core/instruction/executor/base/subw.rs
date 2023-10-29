use crate::rv_core::instruction::executor::prelude::*;

pub fn subw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = (x[rs1] as i32).wrapping_sub(x[rs2] as i32) as u64;
}
