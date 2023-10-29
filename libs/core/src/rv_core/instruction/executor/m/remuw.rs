use crate::rv_core::instruction::executor::prelude::*;

pub fn remuw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let dividend = x[rs1] as u32;
    let divisor = x[rs2] as u32;

    let remainder = if divisor == 0 {
        dividend
    } else {
        dividend.wrapping_rem(divisor)
    };

    x[rd] = remainder as i32 as u64;
}
