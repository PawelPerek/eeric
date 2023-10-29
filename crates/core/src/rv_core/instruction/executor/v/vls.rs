use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vls { vd, rs1, rs2, vm }: Vls,
    eew: BaseSew,
    x: &IntegerRegisters,
    v: &mut VectorContext<'_>,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let stride = x[rs2] as usize;

    let element_amount = v.vlmax();

    let mut store = Vec::<u64>::with_capacity(element_amount);

    for offset in 0..element_amount {
        let address =
            addr.wrapping_add(offset.wrapping_mul(stride).wrapping_mul(eew.byte_length()));

        let element: u64 = match eew {
            BaseSew::E8 => u8::from_le_bytes(mem.get(address)) as u64,
            BaseSew::E16 => u16::from_le_bytes(mem.get(address)) as u64,
            BaseSew::E32 => u32::from_le_bytes(mem.get(address)) as u64,
            BaseSew::E64 => u64::from_le_bytes(mem.get(address)),
        };

        store.push(element);
    }

    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(index, _)| {
            store[index]
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
