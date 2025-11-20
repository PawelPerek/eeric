use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opfvv {
        dest,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vs2 = v.get(vs2);
    let vs1 = v.get(vs1);
    let vd  = v.get_wide(dest)?;

    let vreg = izip!(
        vs2.iter_fp()?,
        vs1.iter_fp()?,
        vd.iter_fp()?
    )
    .masked_map(
        v.default_mask(vm),
        vd.iter_fp()?,
        |(vs2, vs1, vd)| -(vs2.double_precision() * vs1.double_precision()) + vd,
    )
    .collect_fp();

    v.apply(dest, vreg);

    Ok(())
}

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let vreg = izip!(v.get(vs2).iter_fp()?, v.get(vd).iter_fp()?)
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_fp()?,
            |(vs2, vd)| {
                let vs2 = vs2.double_precision();
                -(vs2 * ArbitraryFloat::copy_type(&vs2, f[rs1])) + vd
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
