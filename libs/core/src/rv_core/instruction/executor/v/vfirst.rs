use crate::rv_core::instruction::executor::prelude::*;

pub fn m(
    Vwxunary0 {
        dest: rd, vs2, vm, ..
    }: Vwxunary0,
    v: &VectorContext<'_>,
    x: &mut IntegerRegisters,
) {
    let index = izip!(v.default_mask(vm), v.get(vs2).iter_mask())
        .enumerate()
        .find(|&(_, (v0_mask, vs2_mask))| v0_mask == 1 && vs2_mask == 1)
        .map(|(index, _)| index as u64)
        .unwrap_or(u64::MAX);

    x[rd] = index;
}
