use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vmunary0 {
        dest: vd,
        vs2: _,
        vm,
        ..
    }: Vmunary0,
    v: &mut VectorContext<'_>,
) {
    let vreg = v
        .get(vd)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(i, _)| i as u64)
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
