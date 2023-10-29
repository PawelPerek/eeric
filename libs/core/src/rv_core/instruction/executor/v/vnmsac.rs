use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs1).iter_eew(),
        v.get(vs2).iter_eew()
    )
    .masked_map(
        v.default_mask(vm),
        v.get(vd).iter_eew(),
        |(vd, vs2, vs1)| vd.wrapping_sub(vs1.wrapping_mul(vs2)),
    )
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
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
) {
    let vreg = izip!(v.get(vd).iter_eew(), v.get(vs2).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vd, vs2)| {
            vd.wrapping_sub(x[rs1].wrapping_mul(vs2))
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
