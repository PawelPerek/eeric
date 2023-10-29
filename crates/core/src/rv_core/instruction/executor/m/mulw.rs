use crate::rv_core::instruction::executor::prelude::*;

pub fn mulw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let result = (x[rs1] as u32).wrapping_mul(x[rs2] as u32);
    x[rd] = result as u64;
}
