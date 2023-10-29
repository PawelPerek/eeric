use crate::rv_core::instruction::executor::prelude::*;

pub fn vf2(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let half_sew = v.vec_engine.sew.half()?;

    let vreg = v
        .get(vs2)
        .iter_custom_eew(half_sew)
        .take(v.vlmax() / 2)
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |vs2| match half_sew {
                HalfSew::E8 => vs2 as i8 as u64,
                HalfSew::E16 => vs2 as i16 as u64,
                HalfSew::E32 => vs2 as i32 as u64,
            },
        )
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn vf4(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let fourth_sew = v.vec_engine.sew.fourth()?;

    let vreg = v
        .get(vs2)
        .iter_custom_eew(fourth_sew)
        .take(v.vlmax() / 4)
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |vs2| match fourth_sew {
                FourthSew::E8 => vs2 as i8 as u64,
                FourthSew::E16 => vs2 as i16 as u64,
            },
        )
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn vf8(
    Vxunary0 {
        dest: vd, vs2, vm, ..
    }: Vxunary0,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let eighth_sew = v.vec_engine.sew.eighth()?;

    let vreg = v
        .get(vs2)
        .iter_custom_eew(eighth_sew)
        .take(v.vlmax() / 8)
        .masked_map(
            v.default_mask(vm),
            v.get(vd).iter_eew(),
            |vs2| match eighth_sew {
                EighthSew::E8 => vs2 as i8 as u64,
            },
        )
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
