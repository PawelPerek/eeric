use crate::rv_core::instruction::executor::prelude::*;

pub fn rem(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as i64;
    let divisor = x[rs2] as i64;

    x[rd] = if divisor == 0 {
        x[rs1]
    } else if dividend == i64::MIN && divisor == -1 {
        0
    } else {
        dividend.wrapping_rem(divisor) as u64
    };
}
