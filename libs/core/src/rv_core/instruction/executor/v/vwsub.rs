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
    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_eew(),
            |(vs2, vs1)| (vs2 as i64 as u128).wrapping_sub(vs1 as i64 as u128),
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
    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            (vs2 as i64 as u128).wrapping_sub(x[rs1] as i64 as u128)
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn wv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = izip!(v.get_wide(vs2)?.iter_eew(), v.get(vs1).iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_eew(),
            |(vs2, vs1)| vs2.wrapping_sub(vs1 as i64 as u128),
        )
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn wx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            vs2.wrapping_sub(x[rs1] as i64 as u128)
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
