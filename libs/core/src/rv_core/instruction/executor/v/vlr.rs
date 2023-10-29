use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlr { vd, rs1 }: Vlr,
    nf: usize,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let elements_amount = v.vec_engine.vlen.byte_length();

    for segment in 0..nf {
        let vreg = (0..elements_amount)
            .map(|offset| addr.wrapping_add(offset.wrapping_mul(nf).wrapping_add(segment)))
            .map(|address| mem.get(address))
            .map(u8::from_le_bytes)
            .collect();

        v.apply(vd + segment, vreg);
    }
}
