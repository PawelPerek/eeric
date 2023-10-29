use crate::rv_core::instruction::executor::prelude::*;

pub fn and(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    x[rd] = x[rs1] & x[rs2];
}
