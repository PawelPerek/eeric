use crate::rv_core::instruction::executor::prelude::*;

pub fn vs(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let initial_value = v.get_wide(vs1)?.iter_fp()?.next().unwrap();
    let sum =
        izip!(v.get(vs2).iter_fp()?, v.default_mask(vm)).fold(initial_value, |acc, (vs2, mask)| {
            if mask == 1 {
                acc + vs2.double_precision()
            } else {
                acc
            }
        });

    let mut vd_snapshot = v.get(vd).iter_fp()?.collect_vec();
    vd_snapshot[0] = sum;

    let vreg = vd_snapshot.into_iter().collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
