use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    vlx: Vlx,
    eew: BaseSew,
    nf: usize,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    super::vloxseg::v(vlx, eew, nf, v, x, mem)
}
