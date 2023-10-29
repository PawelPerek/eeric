use crate::rv_core::instruction::executor::prelude::*;

pub fn m(
    Vmunary0 {
        dest: vd, vs2, vm, ..
    }: Vmunary0,
    v: &mut VectorContext<'_>,
) {
    let mut found_mask_bit = false;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            if !found_mask_bit && vs2 != 0 {
                found_mask_bit = true;
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
