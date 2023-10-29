use crate::rv_core::instruction::executor::prelude::*;

pub fn vf(
    Opfvf {
        vd,
        rs1,
        vs2: _,
        vm: _,
    }: Opfvf,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let vreg = v
        .get(vd)
        .iter_fp()?
        .map(|vs2| ArbitraryFloat::copy_type(&vs2, f[rs1]))
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}

pub fn fs(
    Vwfunary0 {
        dest: rd,
        vs2,
        vm: _,
        ..
    }: Vwfunary0,
    v: &VectorContext<'_>,
    f: &mut FloatRegisters,
) -> Result<(), String> {
    let first_value = v.get(vs2).iter_fp()?.next().unwrap();

    let value = match first_value {
        ArbitraryFloat::F32(fp) => {
            let (_, rest) = decompose(f[rd]);
            compose(fp, rest)
        }
        ArbitraryFloat::F64(fp) => fp,
    };

    f[rd] = value;

    Ok(())
}

pub fn sf(
    Vrfunary0 { vd, rs1, vm: _, .. }: Vrfunary0,
    v: &mut VectorContext<'_>,
    f: &FloatRegisters,
) -> Result<(), String> {
    let first_value = f64::to_le_bytes(f[rs1]);

    let vreg = v.get(vd);
    let mut vreg_data = vreg.iter_byte().collect_vec();

    vreg_data[..v.vec_engine.sew.fp()?.byte_length()]
        .copy_from_slice(&first_value[..v.vec_engine.sew.byte_length()]);

    v.apply(vd, vreg_data.into_iter().collect());

    Ok(())
}
