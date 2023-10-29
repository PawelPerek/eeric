use crate::rv_core::instruction::executor::prelude::*;

pub fn div(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as i64;
    let divisor = x[rs2] as i64;

    x[rd] = if divisor == 0 {
        u64::MAX // All bits set
    } else if dividend == i64::MIN && divisor == -1 {
        i64::MIN as u64 // Overflow
    } else {
        dividend.wrapping_div(divisor) as u64
    };
}
