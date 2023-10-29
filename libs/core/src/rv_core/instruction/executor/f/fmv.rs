use crate::rv_core::instruction::executor::prelude::*;

pub fn xw(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    x[rd] = fs1.to_bits() as i32 as u64;
}

pub fn wx(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(f32::from_bits(x[rs1] as u32), rest);
}
