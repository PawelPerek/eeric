use crate::rv_core::instruction::executor::prelude::*;

pub fn fsd(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, f: &FloatRegisters, mem: &mut Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let bytes = f[rs2].to_le_bytes();

    mem.set(addr as usize, bytes);
}
