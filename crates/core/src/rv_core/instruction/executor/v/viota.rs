use crate::rv_core::instruction::executor::prelude::*;

pub fn m(
    Vmunary0 {
        dest: vd, vs2, vm, ..
    }: Vmunary0,
    v: &mut VectorContext<'_>,
) {
    let mut sum = 0u64;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let sum_snapshot = sum;
            if vs2 != 0 {
                sum = sum.wrapping_add(1);
            }
            sum_snapshot
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
