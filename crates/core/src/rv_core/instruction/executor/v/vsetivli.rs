use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetivli(
    Vsetivli { rd, uimm, vtypei }: Vsetivli,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    v.set_vtype(vtypei as u64).unwrap();
    unsafe { v.csr[VL].set(uimm as u64) };
    x[rd] = uimm as u64;
}
