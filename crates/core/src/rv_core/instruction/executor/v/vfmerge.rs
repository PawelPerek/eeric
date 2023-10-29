use crate::rv_core::instruction::executor::prelude::*;

pub fn vfm(
    Opfvf {
        vd,
        rs1,
        vs2,
        vm: _,
    }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let vreg = izip!(v.default_mask(true), v.get(vs2).iter_fp()?)
        .map(|(mask, vs2)| {
            if mask == 1 {
                ArbitraryFloat::copy_type(&vs2, f[rs1])
            } else {
                vs2
            }
        })
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
