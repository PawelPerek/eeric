use crate::rv_core::instruction::executor::prelude::*;

pub fn auipc(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: u64) {
    x[rd] = pc.wrapping_add((imm20 << 12) as u64);
}
