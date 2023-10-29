use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vsr { vs3, rs1 }: Vsr,
    nf: usize,
    v: &VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;

    for segment in 0..nf {
        v.get_single(vs3 + segment)
            .iter_byte()
            .enumerate()
            .for_each(|(offset, vs3)| {
                let address = addr.wrapping_add(offset.wrapping_mul(nf).wrapping_add(segment));
                mem.set(address, vs3.to_le_bytes());
            });
    }
}
