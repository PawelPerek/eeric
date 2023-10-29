use crate::rv_core::instruction::executor::prelude::*;

pub fn divw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as i32;
    let divisor = x[rs2] as i32;

    let quotient = if divisor == 0 {
        -1
    } else if dividend == i32::MIN && divisor == -1 {
        i32::MIN
    } else {
        dividend.wrapping_div(divisor)
    };

    x[rd] = quotient as i64 as u64;
}
