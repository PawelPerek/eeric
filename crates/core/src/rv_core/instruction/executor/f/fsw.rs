use crate::rv_core::instruction::executor::prelude::*;

pub fn fsw(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, f: &FloatRegisters, mem: &mut Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let (fs1, _) = decompose(f[rs2]);
    let bytes = fs1.to_le_bytes();

    mem.set(addr as usize, bytes);
}
