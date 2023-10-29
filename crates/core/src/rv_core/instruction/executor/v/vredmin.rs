use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let initial_value = v.get(vs1).iter_eew().next().unwrap();
    let sum = izip!(v.get(vs2).iter_eew(), v.default_mask(vm)).fold(
        initial_value,
        |min_val, (vs2, mask)| {
            if mask == 1 && (vs2 as i64) < (min_val as i64) {
                vs2
            } else {
                min_val
            }
        },
    );

    let mut vd_data = v.get(vd).iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data.into_iter().collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
