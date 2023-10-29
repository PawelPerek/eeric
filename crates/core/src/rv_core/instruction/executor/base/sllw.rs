use crate::rv_core::instruction::executor::prelude::*;

pub fn sllw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let rs1 = x[rs1];
    let shamt = x[rs2] & 0b11111;
    x[rd] = (rs1 << shamt) as i32 as u64;
}
