use num_traits::{Float, ToPrimitive};

use crate::rv_core::instruction::executor::prelude::*;

pub fn xufw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn xfw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn fxuw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let fp_sew = v.vec_engine.sew.fp()?;

    let vreg = v
        .get_wide(vs2)?
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_fp()?,
            |vs2| match fp_sew {
                FpSew::E32 => ArbitraryFloat::F32(vs2 as u32 as f32),
                FpSew::E64 => ArbitraryFloat::F64(vs2 as u64 as f64),
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn fxw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let fp_sew = v.vec_engine.sew.fp()?;

    let vreg = v
        .get_wide(vs2)?
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_fp()?,
            |vs2| match fp_sew {
                FpSew::E32 => ArbitraryFloat::F32(vs2 as i32 as f32),
                FpSew::E64 => ArbitraryFloat::F64(vs2 as i64 as f64),
            },
        )
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn ffw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |vs2| {
            vs2.half_precision(RoundingMode::Nearest)
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn rodffw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |vs2| {
            vs2.half_precision(RoundingMode::TowardsOdd)
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn rtzxufw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn rtzxfw(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
