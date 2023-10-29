use crate::rv_core::instruction::executor::prelude::*;

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let last_value = f64::to_le_bytes(f[rs1]);

    let vreg_values: Vreg = v
        .get(vs2)
        .iter_byte()
        .take(v.vlmax() - v.vec_engine.sew.byte_length())
        .chain(
            last_value[0..v.vec_engine.sew.fp()?.byte_length()]
                .iter()
                .copied(),
        )
        .collect();

    let vreg = vreg_values
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vd| vd)
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
