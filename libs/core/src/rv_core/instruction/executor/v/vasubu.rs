use crate::rv_core::instruction::executor::prelude::*;

use super::utils::rounding::Roundoff;

pub fn vv(
    Opmvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opmvv,
    v: &mut VectorContext<'_>,
) {
    let roundoff_unsigned = Roundoff::new_unsigned(v.csr);

    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            roundoff_unsigned((vs2 as u128).wrapping_sub(vs1 as u128), 1)
        })
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
    let roundoff_unsigned = Roundoff::new_unsigned(v.csr);

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            roundoff_unsigned((vs2 as u128).wrapping_sub(x[rs1] as u128), 1)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
