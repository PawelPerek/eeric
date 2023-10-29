use std::num::FpCategory;

use crate::rv_core::instruction::executor::prelude::*;

pub fn s(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    // TODO: Rust doesn't distinguish signalling NaN and quiet NaN.
    // I will assume that all NaNs are quiet NaNs.
    // Maybe there is some way to hack it?

    x[rd] = match fs1.classify() {
        FpCategory::Infinite if fs1 < 0.0 => 1 << 0,
        FpCategory::Normal if fs1 < 0.0 => 1 << 1,
        FpCategory::Subnormal if fs1 < 0.0 => 1 << 2,
        FpCategory::Zero if fs1 < 0.0 => 1 << 3,
        FpCategory::Zero if fs1 > 0.0 => 1 << 4,
        FpCategory::Subnormal if fs1 > 0.0 => 1 << 5,
        FpCategory::Normal if fs1 > 0.0 => 1 << 6,
        FpCategory::Infinite if fs1 > 0.0 => 1 << 7,
        FpCategory::Nan => 1 << 9,
        _ => 0,
    };
}
