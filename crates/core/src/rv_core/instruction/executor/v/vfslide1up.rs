use std::convert::identity;

use crate::rv_core::instruction::executor::prelude::*;

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let first_value = f64::to_le_bytes(f[rs1]);
    let vreg_values: Vreg = first_value[0..v.vec_engine.sew.fp()?.byte_length()]
        .iter()
        .copied()
        .chain(v.get(vs2).iter_byte().skip(v.vec_engine.sew.byte_length()))
        .take(v.vlmax())
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), identity)
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
