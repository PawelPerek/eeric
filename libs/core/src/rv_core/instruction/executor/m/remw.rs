use crate::rv_core::instruction::executor::prelude::*;

pub fn remw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as i32;
    let divisor = x[rs2] as i32;

    let remainder = if divisor == 0 {
        dividend
    } else if dividend == i32::MIN && divisor == -1 {
        0
    } else {
        dividend.wrapping_rem(divisor)
    };

    x[rd] = remainder as i64 as u64;
}
