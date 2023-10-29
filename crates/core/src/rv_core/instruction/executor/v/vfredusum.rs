use num_traits::Zero;

use crate::rv_core::instruction::executor::prelude::*;

fn binary_tree_sum(input: impl IntoIterator<Item = ArbitraryFloat>) -> ArbitraryFloat {
    let input_vec: Vec<_> = input.into_iter().collect_vec();

    if input_vec.len() == 1 {
        return input_vec[0];
    }

    let result: Vec<ArbitraryFloat> = input_vec
        .array_chunks::<2>()
        .map(|&[f1, f2]| f1 + f2)
        .collect();

    binary_tree_sum(result)
}

pub fn vs(
    Opfvv {
        dest: vd,
        vs1,
        vs2,
        vm,
    }: Opfvv,
    v: &mut VectorContext<'_>,
) -> Result<(), String> {
    let initial_value = v.get(vs1).iter_fp()?.next().unwrap();
    let binding = v.get(vs2);
    let values = izip!(binding.iter_fp()?, v.default_mask(vm)).map(|(vs2, mask)| {
        if mask == 1 {
            vs2
        } else {
            ArbitraryFloat::zero()
        }
    });

    let sum = initial_value + binary_tree_sum(values);

    let mut vd_snapshot = v.get(vd).iter_fp()?.collect_vec();
    vd_snapshot[0] = sum;

    let vreg = vd_snapshot.into_iter().collect_fp();

    v.apply(vd, vreg);

    Ok(())
}
