use crate::rv_core::instruction::executor::prelude::*;

pub fn lwu(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, mem: &Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let int = u32::from_le_bytes(mem.get(addr as usize));

    x[rd] = int as u64;
}
