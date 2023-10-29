use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vs { vs3, rs1, vm }: Vs,
    eew: BaseSew,
    x: &IntegerRegisters,
    v: &VectorContext<'_>,
    mem: &mut Memory,
) {
    let addr = x[rs1] as usize;

    izip!(v.get(vs3).iter_custom_eew(eew), v.default_mask(vm))
        .enumerate()
        .for_each(|(index, (vs3, mask))| {
            let address = addr.wrapping_add(index.wrapping_mul(eew.byte_length()));

            if mask == 1 {
                match eew {
                    BaseSew::E8 => mem.set(address, (vs3 as u8).to_le_bytes()),
                    BaseSew::E16 => mem.set(address, (vs3 as u16).to_le_bytes()),
                    BaseSew::E32 => mem.set(address, (vs3 as u32).to_le_bytes()),
                    BaseSew::E64 => mem.set(address, vs3.to_le_bytes()),
                }
            }
        });
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn vse_basic() {
//         let mut mem = Memory::default();
//         let mut x = IntegerRegisters::default();
//         let vec_engine = VectorEngineBuilder::default().vlen(VLEN::V64).build();
//         let mut v = VectorRegisters::default(&vec_engine);

//         let vs3 = 0;
//         let rs1 = 0;

//         x[rs1] = 0;

//         v.apply(
//             vs3,
//             vec![5, 7, 2, 2, 2, 2, 6, 8].into_iter().collect::<Vreg>(),
//             &vec_engine,
//         );

//         super::v(
//             Vs {
//                 vs3,
//                 rs1,
//                 vm: false,
//             },
//             SEW::E16,
//             &x,
//             &v,
//             &vec_engine,
//             &mut mem,
//         );

//         let memory = mem.get(0);

//         assert_eq!(memory, [5, 7, 2, 2, 2, 2, 6, 8, 0])
//     }
// }
