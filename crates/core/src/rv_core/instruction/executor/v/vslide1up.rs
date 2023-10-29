use crate::rv_core::instruction::executor::prelude::*;

pub fn vx(
    Opmvx {
        dest: vd,
        rs1,
        vs2,
        vm,
    }: Opmvx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) {
    let first_value = u64::to_le_bytes(x[rs1]);
    let vreg_values: Vreg = first_value[0..v.vec_engine.sew.byte_length()]
        .iter()
        .copied()
        .chain(v.get(vs2).iter_byte().skip(v.vec_engine.sew.byte_length()))
        .take(v.vlmax())
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vd| vd)
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);
}
