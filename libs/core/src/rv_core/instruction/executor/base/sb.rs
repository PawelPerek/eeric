use crate::rv_core::instruction::executor::prelude::*;

pub fn sb(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, mem: &mut Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let bytes = (x[rs2] as u8).to_le_bytes();

    mem.set(addr as usize, bytes);
}
