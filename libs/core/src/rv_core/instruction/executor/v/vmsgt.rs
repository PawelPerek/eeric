use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let vreg = v
        .get(vs2)
        .iter_mask()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            if (vs2 as i64) > (x[rs1] as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) {
    let vreg = v
        .get(vs2)
        .iter_mask()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            if (vs2 as i64) > (imm5 as i64) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
