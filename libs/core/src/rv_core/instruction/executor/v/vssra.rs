use crate::rv_core::instruction::executor::prelude::*;

use super::utils::{rounding::Roundoff, shamt::shamt};

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let roundoff_signed = Roundoff::new_signed(v.csr);

    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            roundoff_signed(vs2 as u128, shamt(vs1, v.vec_engine.sew) as u8)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let roundoff_signed = Roundoff::new_signed(v.csr);

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            roundoff_signed(vs2 as u128, shamt(x[rs1], v.vec_engine.sew) as u8)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) {
    let roundoff_signed = Roundoff::new_signed(v.csr);

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            roundoff_signed(vs2 as u128, shamt(imm5 as u64, v.vec_engine.sew) as u8)
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
