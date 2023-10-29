use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |(vs2, vs1)| match v.vec_engine.sew {
                BaseSew::E8 => (vs2 as u8).saturating_sub(vs1 as u8) as u64,
                BaseSew::E16 => (vs2 as u16).saturating_sub(vs1 as u16) as u64,
                BaseSew::E32 => (vs2 as u32).saturating_sub(vs1 as u32) as u64,
                BaseSew::E64 => vs2.saturating_sub(vs1),
            },
        )
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            match v.vec_engine.sew {
                BaseSew::E8 => (vs2 as u8).saturating_sub(x[rs1] as u8) as u64,
                BaseSew::E16 => (vs2 as u16).saturating_sub(x[rs1] as u16) as u64,
                BaseSew::E32 => (vs2 as u32).saturating_sub(x[rs1] as u32) as u64,
                BaseSew::E64 => vs2.saturating_sub(x[rs1]),
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
