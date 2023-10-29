use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vsx { vs3, rs1, vs2, vm }: Vsx,
    eew: BaseSew,
    nf: usize,
    v: &VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;
    let vs2 = v.get(vs2).iter_custom_eew(eew).collect_vec();

    for segment in 0..nf {
        izip!(v.get(vs3 + segment).iter_eew(), v.default_mask(vm))
            .enumerate()
            .for_each(|(index, (vs3, mask))| {
                let offset = vs2[index] as usize;
                let address = addr
                    .wrapping_add(offset)
                    .wrapping_add(segment.wrapping_mul(v.vec_engine.sew.byte_length()));
                if mask == 1 {
                    match v.vec_engine.sew {
                        BaseSew::E8 => mem.set(address, (vs3 as u8).to_le_bytes()),
                        BaseSew::E16 => mem.set(address, (vs3 as u16).to_le_bytes()),
                        BaseSew::E32 => mem.set(address, (vs3 as u32).to_le_bytes()),
                        BaseSew::E64 => mem.set(address, vs3.to_le_bytes()),
                    };
                }
            });
    }
}
