use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let vlmax = v.vlmax();

    let vs2_state = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vs1)
        .iter_custom_eew(BaseSew::E16)
        .take(vlmax)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vindex| {
            if vindex as usize >= vlmax {
                0
            } else {
                vs2_state[vindex as usize]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
