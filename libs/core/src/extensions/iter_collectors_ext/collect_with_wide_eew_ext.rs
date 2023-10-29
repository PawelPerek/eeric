use crate::rv_core::{
    registers::vector::Vreg,
    vector_engine::sew::{BaseSew, Sew},
};

pub trait IterEEWWidenCollectorExt {
    fn collect_with_wide_eew(self, eew: BaseSew) -> Vreg;
}

impl<I> IterEEWWidenCollectorExt for I
where
    I: Iterator<Item = u128>,
{
    fn collect_with_wide_eew(self, eew: BaseSew) -> Vreg {
        Vreg {
            raw: self
                .map(u128::to_le_bytes)
                .flat_map(|bytes| bytes[0..eew.byte_length()].to_owned())
                .collect(),
            eew,
        }
    }
}
