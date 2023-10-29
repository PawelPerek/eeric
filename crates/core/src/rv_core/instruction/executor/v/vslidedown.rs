use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let offset = x[rs1] as usize;

    let vs2_snapshot = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(index, _)| {
            vs2_snapshot.get(index + offset).copied().unwrap_or(0)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) {
    let offset = imm5 as usize;

    let vs2_snapshot = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(index, _)| {
            vs2_snapshot.get(index + offset).copied().unwrap_or(0)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
