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
) -> Result<(), String> {
    let vreg = izip!(v.get(vs2).iter_eew(), v.get_wide(vd)?.iter_eew())
        .masked_map(
            v.default_mask(vm),
            v.get_wide(vd)?.iter_eew(),
            |(vs2, vd)| {
                (vs2 as i64 as u128)
                    .wrapping_mul(x[rs1] as u128)
                    .wrapping_add(vd)
            },
        )
        .collect_with_wide_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
