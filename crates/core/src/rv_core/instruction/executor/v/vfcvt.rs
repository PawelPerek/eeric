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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew);

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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.round().to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn fxuv(
    Vfunary0 {
        dest: vd, vs2, vm, ..
    }: Vfunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let fp_sew = v.vec_engine.sew.fp()?;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_fp()?,
            |vs2| match fp_sew {
                FpSew::E32 => ArbitraryFloat::F32(vs2 as f32),
                FpSew::E64 => ArbitraryFloat::F64(vs2 as f64),
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
    let fp_sew = v.vec_engine.sew.fp()?;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_fp()?,
            |vs2| match fp_sew {
                FpSew::E32 => ArbitraryFloat::F32(vs2 as i64 as f32),
                FpSew::E64 => ArbitraryFloat::F64(vs2 as i64 as f64),
            },
        )
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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_u64().unwrap()
        })
        .collect_with_eew(v.vec_engine.sew);

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
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            vs2.to_i64().unwrap() as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
