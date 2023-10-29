use std::num::FpCategory;

use crate::rv_core::instruction::executor::prelude::*;

pub fn d(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    // TODO: see f::fclass::s

    x[rd] = match f[rs1].classify() {
        FpCategory::Infinite if f[rs1] < 0.0 => 1 << 0,
        FpCategory::Normal if f[rs1] < 0.0 => 1 << 1,
        FpCategory::Subnormal if f[rs1] < 0.0 => 1 << 2,
        FpCategory::Zero if f[rs1] < 0.0 => 1 << 3,
        FpCategory::Zero if f[rs1] > 0.0 => 1 << 4,
        FpCategory::Subnormal if f[rs1] > 0.0 => 1 << 5,
        FpCategory::Normal if f[rs1] > 0.0 => 1 << 6,
        FpCategory::Infinite if f[rs1] > 0.0 => 1 << 7,
        FpCategory::Nan => 1 << 9,
        _ => 0,
    };
}
