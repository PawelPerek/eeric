use crate::rv_core::instruction::executor::prelude::*;

pub fn vm(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm: _,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_mask())
        .filter_map(|(vs2, vs1)| match vs1 {
            0 => None,
            _ => Some(vs2),
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
