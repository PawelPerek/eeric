use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(Opivx { dest, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let offset = x[rs1] as usize;

    let vs2_snapshot = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(dest)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(dest).iter_eew(), |(index, dest)| {
            if index < offset {
                dest
            } else {
                vs2_snapshot[index - offset]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}

pub fn vi(Opivi { dest, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) {
    let offset = imm5 as usize;

    let vs2_snapshot = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(dest)
        .iter_eew()
        .enumerate()
        .masked_map(v.default_mask(vm), v.get(dest).iter_eew(), |(index, dest)| {
            if index < offset {
                dest
            } else {
                vs2_snapshot[index - offset]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(dest, vreg);
}
