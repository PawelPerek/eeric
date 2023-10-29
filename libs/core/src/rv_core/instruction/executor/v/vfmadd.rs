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
    let vreg = izip!(
        v.get(vs2).iter_fp()?,
        v.get(vs1).iter_fp()?,
        v.get(vd).iter_fp()?
    )
    .masked_map(
        v.default_mask(vm),
        v.get(vd).iter_fp()?,
        |(vs2, vs1, vd)| (vd * vs1) + vs2,
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
    let vreg = izip!(v.get(vs2).iter_fp()?, v.get(vd).iter_fp()?)
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |(vs2, vd)| {
            (vd * ArbitraryFloat::copy_type(&vs2, f[rs1])) + vs2
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
