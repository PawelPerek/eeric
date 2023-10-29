use crate::rv_core::instruction::executor::prelude::*;

pub fn xd(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = f[rs1].to_bits();
}

pub fn dx(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    f[rd] = f64::from_bits(x[rs1]);
}
