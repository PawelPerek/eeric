use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlx { vd, rs1, vs2, vm }: Vlx,
    eew: BaseSew,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let addr = x[rs1] as usize;
    let vs2 = v.get(vs2).iter_custom_eew(eew).collect_vec();

    let element_amount = v.vlmax();

    let mut store = Vec::with_capacity(element_amount);

    for offset in vs2.iter().take(element_amount) {
        let address = addr.wrapping_add(*offset as usize);

        let element: u64 = match v.vec_engine.sew {
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
