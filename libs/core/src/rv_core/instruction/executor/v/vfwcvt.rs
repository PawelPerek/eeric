use num_traits::{Float, ToPrimitive};

use crate::rv_core::instruction::executor::prelude::*;

pub fn xufv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            vs2.round().to_u128().unwrap()
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn xfv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            vs2.round().to_i128().unwrap() as u128
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn fxuv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let sew = v.vec_engine.sew;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_fp()?,
            |vs2| match sew {
                BaseSew::E16 => ArbitraryFloat::F32(vs2 as u16 as f32),
                BaseSew::E32 => ArbitraryFloat::F64(vs2 as u32 as f64),
                _ => unreachable!(),
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn fxv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let sew = v.vec_engine.sew;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_fp()?,
            |vs2| match sew {
                BaseSew::E16 => ArbitraryFloat::F32(vs2 as i16 as f32),
                BaseSew::E32 => ArbitraryFloat::F64(vs2 as i32 as f64),
                _ => unreachable!(),
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn ffv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |vs2| {
            vs2.double_precision()
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn rtzxufv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            vs2.to_u128().unwrap()
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn rtzxfv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get_wide(vd)?.iter_eew(), |vs2| {
            vs2.to_i128().unwrap() as u128
        })
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
