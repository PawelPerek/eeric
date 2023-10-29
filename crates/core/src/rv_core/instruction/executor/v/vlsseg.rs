use crate::rv_core::instruction::executor::prelude::*;

pub fn v(
    Vls { vd, rs1, rs2, vm }: Vls,
    eew: BaseSew,
    nf: usize,
    v: &mut VectorContext<'_>,
    x: &IntegerRegisters,
    mem: &Memory,
) {
    let base_addr = x[rs1] as usize;
    let stride = x[rs2] as usize;
    let element_amount = v.vlmax();

    for segment in 0..nf {
        let mut vn = Vec::new();

        for offset in 0..element_amount {
            let element_addr = base_addr
                .wrapping_add(offset.wrapping_mul(stride))
                .wrapping_add(segment.wrapping_mul(eew.byte_length()));
            let element: u64 = match eew.byte_length() {
                1 => u8::from_le_bytes(mem.get(element_addr)) as u64,
                2 => u16::from_le_bytes(mem.get(element_addr)) as u64,
                4 => u32::from_le_bytes(mem.get(element_addr)) as u64,
                8 => u64::from_le_bytes(mem.get(element_addr)),
                _ => panic!("Unsupported EEW byte length."),
            };

            vn.push(element);
        }

        let vreg = v
            .get(vd + segment)
            .iter_eew()
            .enumerate()
            .masked_map(
                v.default_mask(vm),
                v.get(vd + segment).iter_eew(),
                |(index, _)| vn[index],
            )
            .collect_with_eew(v.vec_engine.sew);

        v.apply(vd + segment, vreg)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::rv_core::vector_engine::VectorEngineBuilder;

//     use super::*;

//     #[test]
//     fn vlsseg_basic() {
//         let mem_content = (0..100).collect_vec();

//         let mem = Memory::from(mem_content);
//         let mut x = IntegerRegisters::default();
//         let vec_engine = VectorEngineBuilder::default().vlen(VLEN::V64).build();
//         let mut v = VectorRegisters::default(&vec_engine);

//         x[5] = 0; // base address
//         x[6] = 4; // stride

//         super::v(
//             Vls {
//                 vd: 0,
//                 rs1: 5,
//                 rs2: 6,
//                 vm: false,
//             },
//             SEW::E8,
//             2,
//             &mut v,
//             &vec_engine,
//             &x,
//             &mem,
//         );

//         let first_segment = v.get(0, &vec_engine).iter_eew().collect_vec();
//         let second_segment = v.get(1, &vec_engine).iter_eew().collect_vec();

//         assert_eq!(first_segment, vec![0, 4, 8, 12, 16, 20, 24, 28]);
//         assert_eq!(second_segment, vec![1, 5, 9, 13, 17, 21, 25, 29]);
//     }

//     #[test]
//     fn vlsseg_zero() {
//         let mem_content = (0..100).collect_vec();

//         let mem = Memory::from(mem_content);
//         let mut x = IntegerRegisters::default();
//         let vec_engine = VectorEngineBuilder::default().vlen(VLEN::V64).build();
//         let mut v = VectorRegisters::default(&vec_engine);

//         x[5] = 10; // base address
//         x[6] = 0; // stride

//         super::v(
//             Vls {
//                 vd: 0,
//                 rs1: 5,
//                 rs2: 6,
//                 vm: false,
//             },
//             SEW::E8,
//             2,
//             &mut v,
//             &vec_engine,
//             &x,
//             &mem,
//         );

//         let first_segment = v.get(0, &vec_engine).iter_eew().collect_vec();
//         let second_segment = v.get(1, &vec_engine).iter_eew().collect_vec();

//         assert_eq!(first_segment, vec![10, 10, 10, 10, 10, 10, 10, 10]);
//         assert_eq!(second_segment, vec![11, 11, 11, 11, 11, 11, 11, 11]);
//     }

//     #[test]
//     fn vlsseg_negative() {
//         let mem_content = (0..100).collect_vec();

//         let mem = Memory::from(mem_content);
//         let mut x = IntegerRegisters::default();
//         let vec_engine = VectorEngineBuilder::default().vlen(VLEN::V64).build();
//         let mut v = VectorRegisters::default(&vec_engine);

//         x[5] = 20; // base address
//         x[6] = -2_i64 as u64; // stride

//         super::v(
//             Vls {
//                 vd: 0,
//                 rs1: 5,
//                 rs2: 6,
//                 vm: false,
//             },
//             SEW::E8,
//             2,
//             &mut v,
//             &vec_engine,
//             &x,
//             &mem,
//         );

//         let first_segment = v.get(0, &vec_engine).iter_eew().collect_vec();
//         let second_segment = v.get(1, &vec_engine).iter_eew().collect_vec();

//         assert_eq!(first_segment, vec![20, 18, 16, 14, 12, 10, 8, 6]);
//         assert_eq!(second_segment, vec![21, 19, 17, 15, 13, 11, 9, 7]);
//     }
// }
