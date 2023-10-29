use crate::rv_core::instruction::executor::prelude::*;

pub fn divuw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as u32;
    let divisor = x[rs2] as u32;

    let quotient = if divisor == 0 {
        u32::MAX // Division by zero, all bits set
    } else {
        dividend.wrapping_div(divisor)
    };

    x[rd] = quotient as i32 as u64;
}
