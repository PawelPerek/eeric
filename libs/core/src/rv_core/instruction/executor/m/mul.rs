use crate::rv_core::instruction::executor::prelude::*;

pub fn mul(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1].wrapping_mul(x[rs2]);
}
