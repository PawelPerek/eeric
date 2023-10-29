use crate::rv_core::instruction::executor::prelude::*;

fn checked_add_3(x: u64, y: u64, z: u64) -> Option<u64> {
    x.checked_add(y).and_then(|sum| sum.checked_add(z))
}

pub fn vvm(
    Opivv {
        vd,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew(),
        v.default_mask(true)
    )
    .map(|(vd, vs1, vs2, mask)| (vd, checked_add_3(vs1, vs2, mask)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vxm(
    Opivx {
        vd,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs2).iter_eew(),
        v.default_mask(true)
    )
    .map(|(vd, vs2, mask)| (vd, checked_add_3(x[rs1], vs2, mask)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vim(
    Opivi {
        vd,
        imm5,
        vs2,
        vm: _,
    }: Opivi,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs2).iter_eew(),
        v.default_mask(true)
    )
    .map(|(vd, vs2, mask)| (vd, checked_add_3(imm5 as u64, vs2, mask)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vv(
    Opivv {
        vd,
        vs1,
        vs2,
        vm: _,
    }: Opivv,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew()
    )
    .map(|(vd, vs2, vs1)| (vd, vs1.checked_add(vs2)))
    .map(|(vd, maybe_sum)| {
        vd.with_mask_bit(match maybe_sum {
            Some(_) => 1,
            None => 0,
        })
    })
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vx(
    Opivx {
        vd,
        rs1,
        vs2,
        vm: _,
    }: Opivx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) {
    let vreg = izip!(v.get(vd).iter_eew(), v.get(vs2).iter_eew())
        .map(|(vd, vs2)| (vd, x[rs1].checked_add(vs2)))
        .map(|(vd, maybe_sum)| {
            vd.with_mask_bit(match maybe_sum {
                Some(_) => 1,
                None => 0,
            })
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vi(
    Opivi {
        vd,
        imm5,
        vs2,
        vm: _,
    }: Opivi,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(v.get(vd).iter_eew(), v.get(vs2).iter_eew())
        .map(|(vd, vs2)| (vd, (imm5 as u64).checked_add(vs2)))
        .map(|(vd, maybe_sum)| {
            vd.with_mask_bit(match maybe_sum {
                Some(_) => 1,
                None => 0,
            })
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
