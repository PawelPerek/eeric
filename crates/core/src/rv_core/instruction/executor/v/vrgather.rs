use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) {
    let vlmax = v.vlmax();

    let vs2_state = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vs1)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vindex| {
            if vindex as usize >= vlmax {
                0
            } else {
                vs2_state[vindex as usize]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vx(Opivx { vd, rs1, vs2, vm }: Opivx, v: &mut VectorContext<'_>, x: &IntegerRegisters) {
    let vlmax = v.vlmax();
    let index = x[rs1];

    let vs2_state = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vd)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |_| {
            if index as usize >= vlmax {
                0
            } else {
                vs2_state[index as usize]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}

pub fn vi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) {
    let vlmax = v.vlmax();
    let index = imm5;

    let vs2_state = v.get(vs2).iter_eew().collect_vec();

    let vreg = v
        .get(vd)
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |_| {
            if index as usize >= vlmax {
                0
            } else {
                vs2_state[index as usize]
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
