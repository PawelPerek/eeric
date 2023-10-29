use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = izip!(
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew(),
        v.get_wide(vd)?.iter_eew()
    )
    .masked_map(
        v.default_mask(vm),
        v.get_wide(vd)?.iter_eew(),
        |(vs1, vs2, vd)| (vs2 as u128).wrapping_mul(vs1 as u128).wrapping_add(vd),
    )
    .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

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
