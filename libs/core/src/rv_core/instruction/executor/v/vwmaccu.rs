use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vs1 = v.get(vs1);
    let vs2 = v.get(vs2);
    let vd = v.get_wide(dest)?;
    
    let vreg = izip!(
        vs1.iter_eew(),
        vs2.iter_eew(),
        vd.iter_eew()
    )
    .masked_map(
        v.default_mask(vm),
        vd.iter_eew(),
        |(vs1, vs2, vd)| (vs2 as u128).wrapping_mul(vs1 as u128).wrapping_add(vd),
    )
    .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(dest, vreg);

    Ok(())
}

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) -> Result<(), String> {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get_wide(vd)?.iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_eew(),
            |(vs2, vd)| (vs2 as u128).wrapping_mul(x[rs1] as u128).wrapping_add(vd),
        )
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
