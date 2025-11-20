use crate::rv_core::instruction::executor::prelude::*;

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

    let vreg = izip!(
        vs2.iter_eew(),
        vs1.iter_eew(),
        v.default_mask(true)
    )
    .map(|(vs2, vs1, mask)| vs2.wrapping_add(vs1).wrapping_add(mask))
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
    let vreg = izip!(v.get(vs2).iter_eew(), v.default_mask(true))
        .map(|(vs2, mask)| vs2.wrapping_add(x[rs1]).wrapping_add(mask))
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vim(
    Opivi {
        dest,
        imm5,
        vs2,
        vm: _,
    }: Opivi,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.default_mask(true))
        .map(|(vs2, mask)| vs2.wrapping_add(imm5 as u64).wrapping_add(mask))
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}
