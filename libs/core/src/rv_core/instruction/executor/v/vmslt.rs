use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { dest, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let vreg = izip!(v.get(vs2).iter_mask(), v.get(vs1).iter_mask(),)
        .masked_map(v.default_mask(vm), v.get(dest).iter_eew(), |(vs2, vs1)| {
            if (vs2 as i64) < (vs1 as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vx(Opivx { dest, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_mask()
        .masked_map(v.default_mask(vm), v.get(dest).iter_eew(), |vs2| {
            if (vs2 as i64) < (x[rs1] as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}
