use crate::rv_core::instruction::executor::prelude::*;

pub fn ld(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, mem: &Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let int = u64::from_le_bytes(mem.get(addr as usize));

    x[rd] = int;
}
