use crate::rv_core::{
    arbitrary_float::ArbitraryFloat,
    vector_engine::sew::{DoubleFpSew, DoubleSew, Sew},
};

#[derive(Clone)]
pub struct WideVreg {
    pub raw: Vec<u8>,
    pub eew: DoubleSew,
}

impl WideVreg {
    pub fn new(raw: Vec<u8>, eew: DoubleSew) -> Self {
        Self { raw, eew }
    }

    pub fn iter_byte(&self) -> WideVregByteIterator<'_> {
        WideVregByteIterator { vreg: self, ptr: 0 }
    }

    pub fn iter_eew(&self) -> WideVregEEWIterator<'_> {
        WideVregEEWIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew,
        }
    }

    pub fn iter_fp(&self) -> Result<WideVregFPIterator<'_>, String> {
        Ok(WideVregFPIterator {
            byte_iterator: self.iter_byte(),
            eew: self.eew.fp()?,
        })
    }
}

/// Iterators

// byte-by-byte

pub struct WideVregByteIterator<'a> {
    vreg: &'a WideVreg,
    ptr: usize,
}

impl<'a> Iterator for WideVregByteIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.vreg.raw.get(self.ptr).copied();
        self.ptr += 1;
        element
    }
}

impl<'a> ExactSizeIterator for WideVregByteIterator<'a> {
    fn len(&self) -> usize {
        self.vreg.raw.len() - self.ptr
    }
}

// EEW

pub struct WideVregEEWIterator<'a> {
    byte_iterator: WideVregByteIterator<'a>,
    eew: DoubleSew,
}

impl<'a> Iterator for WideVregEEWIterator<'a> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_iterator.len() == 0 {
            return None;
        }

        let mut bytes = [0x00_u8; std::mem::size_of::<u128>()];

        for byte_element in bytes.iter_mut().take(self.eew.byte_length()) {
            let byte = self
                .byte_iterator
                .next()
                .expect("WideVregEEWIterator finished early, EEW is not divisible by VLEN*EMUL?");
            *byte_element = byte;
        }

        Some(u128::from_le_bytes(bytes))
    }
}

// Float EEW

pub struct WideVregFPIterator<'a> {
    byte_iterator: WideVregByteIterator<'a>,
    eew: DoubleFpSew,
}

impl<'a> Iterator for WideVregFPIterator<'a> {
    type Item = ArbitraryFloat;

    fn next(&mut self) -> Option<Self::Item> {
        match self.eew {
            DoubleFpSew::E64 => self
                .byte_iterator
                .next_chunk()
                .map(f64::from_le_bytes)
                .map(ArbitraryFloat::F64)
                .ok(),
        }
    }
}
