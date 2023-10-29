mod vreg;
mod wide_vreg;

use crate::prelude::*;
pub use vreg::Vreg;
pub use wide_vreg::WideVreg;

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct VectorRegisters(pub Vec<u8>);

impl Snapshotable for VectorRegisters {
    type Snapshot = Vec<u8>;

    fn snapshot(&self) -> Self::Snapshot {
        self.0.clone()
    }
}

impl VectorRegisters {
    pub fn default(vec_engine: &VectorEngine) -> Self {
        let vlenb = vec_engine.vlen.byte_length();

        Self(vec![0x00; vlenb * 32])
    }
}
