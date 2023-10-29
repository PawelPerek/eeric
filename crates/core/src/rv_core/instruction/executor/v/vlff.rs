use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vl { vd, rs1, vm }: Vl,
    eew: BaseSew,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) -> Result<(), String> {
    let addr = x[rs1] as usize;

    let element_amount = v.vlmax();

    let mut store = Vec::<u64>::with_capacity(element_amount);

    for offset in 0..element_amount {
        let address = addr.wrapping_add(offset.wrapping_mul(eew.byte_length()));

        let result = match eew {
            BaseSew::E8 => mem
                .fallible_get(address)
                .map(u8::from_le_bytes)
                .map(Into::into),
            BaseSew::E16 => mem
                .fallible_get(address)
                .map(u16::from_le_bytes)
                .map(Into::into),
            BaseSew::E32 => mem
                .fallible_get(address)
                .map(u32::from_le_bytes)
                .map(Into::into),
            BaseSew::E64 => mem.fallible_get(address).map(u64::from_le_bytes),
        };

        match result {
            Some(element) => store.push(element),
            None => {
                if offset == 0 {
                    return Err(String::from("Fault-Only-First Load trap"));
                } else {
                    v.csr[VL].write(offset as u64)?;
                }
            }
        };
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
    Ok(())
}
