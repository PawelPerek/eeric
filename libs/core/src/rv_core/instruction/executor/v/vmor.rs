use crate::rv_core::instruction::executor::prelude::*;

pub fn mm(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(
        v.get(vd).iter_eew(),
        v.get(vs2).iter_mask(),
        v.get(vs1).iter_mask(),
    )
    .map(|(vd, vs2, vs1)| vd.with_mask_bit(vs2 | vs1))
    .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
