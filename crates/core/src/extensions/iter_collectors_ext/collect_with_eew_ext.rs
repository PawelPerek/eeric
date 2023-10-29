use crate::rv_core::{
    registers::vector::Vreg,
    vector_engine::sew::{BaseSew, Sew},
};

pub trait IterEEWCollectorExt {
    fn collect_with_eew(self, eew: BaseSew) -> Vreg;
}

impl<I> IterEEWCollectorExt for I
where
    I: Iterator<Item = u64>,
{
    fn collect_with_eew(self, eew: BaseSew) -> Vreg {
        Vreg {
            raw: self
                .map(u64::to_le_bytes)
                .flat_map(|bytes| bytes[0..eew.byte_length()].to_owned())
                .collect(),
            eew,
        }
    }
}
