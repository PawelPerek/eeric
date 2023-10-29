use crate::rv_core::instruction::executor::prelude::*;

pub fn mulhu(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let full_product = (x[rs1] as u128).wrapping_mul(x[rs2] as u128);
    x[rd] = (full_product >> 64) as u64;
}
