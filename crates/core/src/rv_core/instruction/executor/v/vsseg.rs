use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vs { vs3, rs1, vm }: Vs,
    eew: BaseSew,
    nf: usize,
    v: &VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;

    for segment in 0..nf {
        izip!(
            v.get(vs3 + segment).iter_custom_eew(eew),
            v.default_mask(vm)
        )
        .enumerate()
        .for_each(|(index, (value, mask))| {
            let segment_size = nf.wrapping_mul(eew.byte_length());
            let segment_index = segment.wrapping_mul(eew.byte_length());

            let offset = index.wrapping_mul(segment_size).wrapping_add(segment_index);

            let address = addr.wrapping_add(offset);
            if mask == 1 {
                match eew {
                    BaseSew::E8 => mem.set(address, (value as u8).to_le_bytes()),
                    BaseSew::E16 => mem.set(address, (value as u16).to_le_bytes()),
                    BaseSew::E32 => mem.set(address, (value as u32).to_le_bytes()),
                    BaseSew::E64 => mem.set(address, value.to_le_bytes()),
                };
            }
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn vseeg_basic() {
//         let mut mem = Memory::default();
//         let mut x = IntegerRegisters::default();
//         let vec_engine = VectorEngineBuilder::default().vlen(VLEN::V64).build();
//         let mut v = VectorRegisters::default(&vec_engine);

//         let vs3 = 0;
//         let rs1 = 0;

//         x[rs1] = 4;

//         v.apply(
//             vs3,
//             vec![1, 2, 3, 4, 5, 6, 7, 8].into_iter().collect::<Vreg>(),
//             &vec_engine,
//         );
//         v.apply(
//             vs3 + 1,
//             vec![2, 4, 6, 8, 10, 12, 14, 16]
//                 .into_iter()
//                 .collect::<Vreg>(),
//             &vec_engine,
//         );
//         v.apply(
//             vs3 + 2,
//             vec![3, 6, 9, 12, 15, 18, 21, 24]
//                 .into_iter()
//                 .collect::<Vreg>(),
//             &vec_engine,
//         );

//         super::v(
//             Vs {
//                 vs3,
//                 rs1,
//                 vm: false,
//             },
//             SEW::E8,
//             3,
//             &v,
//             &vec_engine,
//             &x,
//             &mut mem,
//         );

//         let memory = mem.get(x[rs1] as usize);

//         assert_eq!(
//             memory,
//             [1, 2, 3, 2, 4, 6, 3, 6, 9, 4, 8, 12, 5, 10, 15, 6, 12, 18, 7, 14, 21, 8, 16, 24]
//         );
//     }
// }
