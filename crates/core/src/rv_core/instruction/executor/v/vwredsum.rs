use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) -> Result<(), String> {
    let initial_value = v.get_wide(vs1)?.iter_eew().next().unwrap();
    let sum = izip!(v.get(vs2).iter_eew(), v.default_mask(vm))
        .fold(initial_value, |acc, (vs2, mask)| {
            acc.wrapping_add(vs2 as i64 as u128 * mask as u128)
        });

    let mut vd_data = v.get_wide(vd)?.iter_eew().collect_vec();
    vd_data[0] = sum;

    let vreg = vd_data.into_iter().collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
