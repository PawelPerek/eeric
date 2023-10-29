use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    vsx: Vsx,
    eew: BaseSew,
    nf: usize,
    v: &VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    super::vsoxseg::v(vsx, eew, nf, v, x, mem)
}
