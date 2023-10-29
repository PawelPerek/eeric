use crate::rv_core::instruction::executor::prelude::*;

use super::utils::shamt::narrow_shamt;

pub fn wv(Opivv { vd, vs1, vs2, vm }: Opivv, v: &mut VectorContext<'_>) -> Result<(), String> {
    let vreg = izip!(v.get_wide(vs2)?.iter_eew(), v.get(vs1).iter_eew())
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            (vs2 >> narrow_shamt(vs1, v.vec_engine.sew.double())) as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn wx(
    Opivx { vd, rs1, vs2, vm }: Opivx,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            (vs2 >> narrow_shamt(x[rs1], v.vec_engine.sew.double())) as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn wi(Opivi { vd, imm5, vs2, vm }: Opivi, v: &mut VectorContext<'_>) -> Result<(), String> {
    let vreg = v
        .get_wide(vs2)?
        .iter_eew()
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            (vs2 >> narrow_shamt(imm5 as u64, v.vec_engine.sew.double())) as u64
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
