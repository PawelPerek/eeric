use std::num::FpCategory;

use num_traits::{Float, Zero};

use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vfunary1 {
        dest: vd, vs2, vm, ..
    }: Vfunary1,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            match vs2.classify() {
                FpCategory::Infinite if vs2 < ArbitraryFloat::zero() => 1 << 0,
                FpCategory::Normal if vs2 < ArbitraryFloat::zero() => 1 << 1,
                FpCategory::Subnormal if vs2 < ArbitraryFloat::zero() => 1 << 2,
                FpCategory::Zero if vs2 < ArbitraryFloat::zero() => 1 << 3,
                FpCategory::Zero if vs2 > ArbitraryFloat::zero() => 1 << 4,
                FpCategory::Subnormal if vs2 > ArbitraryFloat::zero() => 1 << 5,
                FpCategory::Normal if vs2 > ArbitraryFloat::zero() => 1 << 6,
                FpCategory::Infinite if vs2 > ArbitraryFloat::zero() => 1 << 7,
                FpCategory::Nan => 1 << 9,
                _ => 0,
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
