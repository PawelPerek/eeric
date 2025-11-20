use crate::rv_core::instruction::executor::prelude::*;

pub fn mm(
    Opmvv {
        dest,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let vs2 = v.get(vs2);
    let vs1 = v.get(vs1);
    let vd  = v.get(dest);

    let vreg = izip!(
        vd.iter_eew(),
        vs2.iter_mask(),
        vs1.iter_mask(),
    )
    .map(|(vd, vs2, vs1)| vd.with_mask_bit(vs2 | vs1))
    .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}
