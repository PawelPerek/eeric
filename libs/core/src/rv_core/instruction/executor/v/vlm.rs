use crate::rv_core::instruction::executor::prelude::*;

pub fn v(Vl { vd, rs1, vm: _ }: Vl, v: &mut VectorContext<'_>, x: &IntegerRegisters, mem: &Memory) {
    let addr = x[rs1] as usize;
    let element_amount = v.vec_engine.vlen.byte_length();

    let vreg = (0..element_amount)
        .map(|offset| addr.wrapping_add(offset))
        .map(|address| mem.get(address))
        .map(u8::from_le_bytes)
        .collect();

    v.apply(vd, vreg);
}
