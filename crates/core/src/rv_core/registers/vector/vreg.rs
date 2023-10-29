use crate::rv_core::{
    arbitrary_float::ArbitraryFloat,
    vector_engine::sew::{BaseSew, FpSew, Sew},
};

#[derive(Clone)]
// A wrapper over vector unit raw data
pub struct Vreg {
    pub raw: Vec<u8>,
    pub eew: BaseSew,
}

impl Vreg {
    pub fn new(raw: Vec<u8>, eew: BaseSew) -> Vreg {
        Vreg { raw, eew }
    }

    pub fn iter_byte(&self) -> VregByteIterator<'_> {
        VregByteIterator { vreg: self, ptr: 0 }
    }

    pub fn iter_eew(&self) -> VregEEWIterator<'_, BaseSew> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew,
        }
    }

    pub fn iter_custom_eew<S: Sew>(&self, eew: S) -> VregEEWIterator<'_, S> {
        VregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew,
        }
    }

    pub fn iter_mask(&self) -> VregMaskIterator<'_> {
        VregMaskIterator {
            vreg: self,
            bit_pos: 0,
        }
    }

    pub fn iter_fp(&self) -> Result<VregFPIterator<'_>, String> {
        Ok(VregFPIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.fp()?,
        })
    }
}

impl FromIterator<u8> for Vreg {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut raw = Vec::new();
        raw.extend(iter);

        Vreg {
            raw,
            eew: BaseSew::E8,
        }
    }
}

/// Iterators

// byte-by-byte

pub struct VregByteIterator<'a> {
    vreg: &'a Vreg,
    ptr: usize,
}

impl<'a> Iterator for VregByteIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.vreg.raw.get(self.ptr).copied();
        self.ptr += 1;
        element
    }
}

impl<'a> ExactSizeIterator for VregByteIterator<'a> {
    fn len(&self) -> usize {
        self.vreg.raw.len() - self.ptr
    }
}

// Integer EEW

pub struct VregEEWIterator<'a, S: Sew> {
    byte_iterator: VregByteIterator<'a>,
    eew: S,
}

impl<'a, S: Sew> Iterator for VregEEWIterator<'a, S> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.byte_iterator.len();

        if len == 0 {
            return None;
        }

        let mut bytes = [0x00_u8; std::mem::size_of::<u64>()];

        for byte_element in bytes.iter_mut().take(self.eew.byte_length()) {
            let byte = self
                .byte_iterator
                .next()
                .expect("VregEEWIterator finished early, EEW is not divisible by VLEN*EMUL?");
            *byte_element = byte;
        }

        Some(u64::from_le_bytes(bytes))
    }
}

// mask (1u64 or 0u64)

pub struct VregMaskIterator<'a> {
    vreg: &'a Vreg,
    bit_pos: usize,
}

impl<'a> Iterator for VregMaskIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let vreg_data = self.vreg.raw.get(self.bit_pos / 8)?;
        let element = (*vreg_data >> (self.bit_pos % 8)) & 1;
        self.bit_pos += 1;
        Some(element as u64)
    }
}

// Float EEW

pub struct VregFPIterator<'a> {
    byte_iterator: VregByteIterator<'a>,
    eew: FpSew,
}

impl<'a> Iterator for VregFPIterator<'a> {
    type Item = ArbitraryFloat;

    fn next(&mut self) -> Option<Self::Item> {
        match self.eew {
            FpSew::E32 => self
                .byte_iterator
                .next_chunk()
                .map(f32::from_le_bytes)
                .map(ArbitraryFloat::F32)
                .ok(),
            FpSew::E64 => self
                .byte_iterator
                .next_chunk()
                .map(f64::from_le_bytes)
                .map(ArbitraryFloat::F64)
                .ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e8() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, BaseSew::E8);

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0xef));
        assert_eq!(iter.next(), Some(0xcd));
        assert_eq!(iter.next(), Some(0xab));
        assert_eq!(iter.next(), Some(0x89));
        assert_eq!(iter.next(), Some(0x67));
        assert_eq!(iter.next(), Some(0x45));
        assert_eq!(iter.next(), Some(0x23));
        assert_eq!(iter.next(), Some(0x01));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e16() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, BaseSew::E16);

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0xcdef));
        assert_eq!(iter.next(), Some(0x89ab));
        assert_eq!(iter.next(), Some(0x4567));
        assert_eq!(iter.next(), Some(0x0123));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e32() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, BaseSew::E32);

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x89abcdef));
        assert_eq!(iter.next(), Some(0x01234567));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn e64() {
        let vector_data = vec![0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];

        let vreg = Vreg::new(vector_data, BaseSew::E64);

        let mut iter = vreg.iter_eew();

        assert_eq!(iter.next(), Some(0x0123456789abcdef));
        assert_eq!(iter.next(), None);
    }
}
