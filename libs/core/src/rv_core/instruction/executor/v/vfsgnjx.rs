use num_traits::Float;

use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = izip!(v.get(vs2).iter_fp()?, v.get(vs1).iter_fp()?)
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_fp()?,
            |(vs2, vs1)| match (vs2.is_sign_positive(), vs1.is_sign_positive()) {
                (true, true) => vs2,
                (true, false) => -vs2,
                (false, true) => -vs2,
                (false, false) => vs2,
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |vs2| {
            match (vs2.is_sign_positive(), f[rs1].is_sign_positive()) {
                (true, true) => vs2,
                (true, false) => -vs2,
                (false, true) => -vs2,
                (false, false) => vs2,
            }
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
