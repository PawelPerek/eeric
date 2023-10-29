use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vlx { vd, rs1, vs2, vm }: Vlx,
    eew: BaseSew,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    super::vlox::v(Vlx { vd, rs1, vs2, vm }, eew, v, x, mem)
}
