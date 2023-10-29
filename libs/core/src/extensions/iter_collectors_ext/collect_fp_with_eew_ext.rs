use crate::rv_core::{
    arbitrary_float::ArbitraryFloat, registers::vector::Vreg, vector_engine::sew::BaseSew,
};

pub trait IterFPCollectorExt {
    fn collect_fp(self) -> Vreg;
}

impl<I> IterFPCollectorExt for I
where
    I: Iterator<Item = ArbitraryFloat>,
{
    fn collect_fp(self) -> Vreg {
        let mut eew = BaseSew::E8;

        let raw = self
            .flat_map(|fp| match fp {
                ArbitraryFloat::F32(f) => {
                    eew = BaseSew::E32;
                    f.to_le_bytes().to_vec()
                }
                ArbitraryFloat::F64(f) => {
                    eew = BaseSew::E64;
                    f.to_le_bytes().to_vec()
                }
            })
            .collect();

        Vreg { raw, eew }
    }
}
