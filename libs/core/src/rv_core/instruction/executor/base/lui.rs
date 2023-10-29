use crate::rv_core::instruction::executor::prelude::*;

pub fn lui(U { rd, imm20 }: U, x: &mut IntegerRegisters) {
    x[rd] = (imm20 << 12) as u64;
}
