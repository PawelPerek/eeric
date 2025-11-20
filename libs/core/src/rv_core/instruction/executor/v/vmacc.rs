use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opmvv {
        dest,
        vs1,
        vs2,
        vm,
    }: Opmvv,
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
    .masked_map(
        v.default_mask(vm),
        vd.iter_eew(),
        |(vd, vs2, vs1)| vd.wrapping_add(vs1.wrapping_mul(vs2)),
    )
    .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
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
            vd.wrapping_add(x[rs1].wrapping_mul(vs2))
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
