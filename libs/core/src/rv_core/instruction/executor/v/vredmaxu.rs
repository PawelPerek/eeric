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
        |max_val, (vs2, mask)| {
            if mask == 1 && vs2 > max_val {
                vs2
            } else {
                max_val
            }
        },
    );

    let mut vd_data = v.get(vd).iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data.into_iter().collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
