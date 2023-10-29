/// Vector unit size of microarchitecture
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Vlen {
    V64,
    #[default]
    V128,
    V256,
    V512,
}

impl Vlen {
    pub fn bit_length(&self) -> usize {
        match self {
            Self::V64 => 64,
            Self::V128 => 128,
            Self::V256 => 256,
            Self::V512 => 512,
        }
    }

    pub fn byte_length(&self) -> usize {
        self.bit_length() / 8
    }
}
