use num_traits::Float;

use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vfunary1 {
        dest: vd, vs2, vm, ..
    }: Vfunary1,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let vreg = v
        .get(vs2)
        .iter_fp()?
        .masked_map(v.default_mask(vm), v.get(vd).iter_fp()?, |vs2| vs2.sqrt())
        .collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
