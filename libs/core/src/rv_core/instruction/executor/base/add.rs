use crate::rv_core::instruction::executor::prelude::*;

pub fn add(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1].wrapping_add(x[rs2]);
}
