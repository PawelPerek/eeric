use crate::rv_core::vector_engine::sew::{DoubleSew, Sew};

pub fn shamt(value: u64, sew: impl Sew) -> u64 {
    value & ((1 << sew.bit_length().ilog2()) - 1)
}

pub fn narrow_shamt(value: u64, sew: DoubleSew) -> u64 {
    shamt(value, sew)
}
