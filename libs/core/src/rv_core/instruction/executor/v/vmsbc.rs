use crate::rv_core::instruction::executor::prelude::*;

fn checked_sub_3(x: u64, y: u64, z: u64) -> Option<u64> {
    x.checked_sub(y).and_then(|sum| sum.checked_sub(z))
}

pub fn vvm(
    Opivv {
        dest,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorContext<'_>,
) {
    let vs2 = v.get(vs2);
    let vs1 = v.get(vs1);
    let vd  = v.get(dest);

    let vreg = izip!(
        vd.iter_eew(),
        vs1.iter_eew(),
        vs2.iter_eew(),
        v.default_mask(true)
    )
    .map(|(vd, vs1, vs2, mask)| (vd, checked_sub_3(vs1, vs2, mask)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vxm(
    Opivx {
        dest,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) {
    let vs2 = v.get(vs2);
    let vd  = v.get(dest);

    let vreg = izip!(
        vd.iter_eew(),
        vs2.iter_eew(),
        v.default_mask(true)
    )
    .map(|(vd, vs2, mask)| (vd, checked_sub_3(x[rs1], vs2, mask)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vv(
    Opivv {
        dest,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorContext<'_>,
) {
    let vs2 = v.get(vs2);
    let vs1 = v.get(vs1);
    let vd  = v.get(dest);

    let vreg = izip!(
        vd.iter_eew(),
        vs1.iter_eew(),
        vs2.iter_eew()
    )
    .map(|(vd, vs2, vs1)| (vd, vs1.checked_sub(vs2)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vx(
    Opivx {
        dest,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) {
    let vs2 = v.get(vs2);
    let vd  = v.get(dest);

    let vreg = izip!(vd.iter_eew(), vs2.iter_eew())
        .map(|(vd, vs2)| (vd, x[rs1].checked_sub(vs2)))
        .map(|(vd, maybe_sum)| {
            vd.with_mask_bit(match maybe_sum {
                Some(_) => 1,
                None => 0,
            })
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}
