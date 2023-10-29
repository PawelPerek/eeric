use crate::rv_core::instruction::executor::prelude::*;

pub fn fld(I { rd, rs1, imm12 }: I, x: &IntegerRegisters, f: &mut FloatRegisters, mem: &Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let fp = f64::from_le_bytes(mem.get(addr as usize));

    f[rd] = fp;
}
