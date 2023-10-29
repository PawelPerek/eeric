use crate::rv_core::instruction::executor::prelude::*;

use super::utils::rounding::Roundoff;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let roundoff_signed = Roundoff::new_signed(v.csr);

    let int_max = i64::MAX >> (64 - v.vec_engine.sew.bit_length());
    let int_min = i64::MIN >> (64 - v.vec_engine.sew.bit_length());

    let vec_engine = *v.vec_engine;

    let vreg = izip!(v.get(vs2).iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            let is_overflow = vs2 == vs1 && vs1 == int_min as u64;

            if is_overflow {
                v.csr[VXSAT].write(1);
                int_max as u64
            } else {
                roundoff_signed(
                    vs2 as u128 * vs1 as u128,
                    v.vec_engine.sew.bit_length() as u8 - 1,
                )
            }
        })
        .collect_with_eew(vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let roundoff_signed = Roundoff::new_signed(v.csr);

    let int_max = i64::MAX >> (64 - v.vec_engine.sew.bit_length());
    let int_min = i64::MIN >> (64 - v.vec_engine.sew.bit_length());

    let vec_engine = *v.vec_engine;

    let vreg = v
        .get(vs2)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            let is_overflow = vs2 == x[rs1] && x[rs1] == int_min as u64;

            if is_overflow {
                v.csr[VXSAT].write(1);
                int_max as u64
            } else {
                roundoff_signed(
                    vs2 as u128 * x[rs1] as u128,
                    v.vec_engine.sew.bit_length() as u8 - 1,
                )
            }
        })
        .collect_with_eew(vec_engine.sew);

    v.apply(vd, vreg);
}
