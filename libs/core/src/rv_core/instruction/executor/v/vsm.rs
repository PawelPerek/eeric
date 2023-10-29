use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vs { vs3, rs1, vm: _ }: Vs,
    v: &VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;

    v.get_single(vs3)
        .iter_byte()
        .enumerate()
        .for_each(|(index, vs3)| {
            let address = addr.wrapping_add(index);
            mem.set(address, vs3.to_le_bytes());
        });
}
