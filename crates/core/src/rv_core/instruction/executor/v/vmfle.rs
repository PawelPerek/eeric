use crate::rv_core::instruction::executor::prelude::*;

pub fn vv(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = izip!(v.get(vs2).iter_fp()?, v.get(vs1).iter_fp()?)
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |(vs2, vs1)| {
            if vs2 <= vs1 {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}

pub fn vf(
    Opfvf { vd, rs1, vs2, vm }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_eew(), |vs2| {
            if vs2 <= ArbitraryFloat::copy_type(&vs2, f[rs1]) {
                1
            } else {
                0
            }
        })
        .collect_with_eew(v.vec_engine.sew);

    v.apply(vd, vreg);

    Ok(())
}
