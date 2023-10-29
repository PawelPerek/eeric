mod lmul;
mod mask_behaviour;
pub mod sew;
mod vlen;

use derive_builder::Builder;
pub use mask_behaviour::MaskBehavior;

pub use lmul::Lmul;
use sew::BaseSew;
pub use vlen::Vlen;

use super::snapshot::Snapshotable;

#[derive(Builder, Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[builder(build_fn(skip))]
pub struct VectorEngine {
    pub lmul: Lmul,
    pub vlen: Vlen,
    pub sew: BaseSew,
    #[allow(dead_code)]
    pub tail_elements: MaskBehavior,
    #[allow(dead_code)]
    pub inactive_elements: MaskBehavior,
}

impl VectorEngine {
    pub fn new(
        lmul: Lmul,
        vlen: Vlen,
        sew: BaseSew,
        tail_elements: MaskBehavior,
        inactive_elements: MaskBehavior,
    ) -> Self {
        Self {
            lmul,
            vlen,
            sew,
            tail_elements,
            inactive_elements,
        }
    }
}

impl VectorEngineBuilder {
    pub fn build(&self) -> VectorEngine {
        VectorEngine {
            lmul: self.lmul.unwrap_or_default(),
            vlen: self.vlen.unwrap_or_default(),
            sew: self.sew.unwrap_or_default(),
            tail_elements: self.tail_elements.unwrap_or_default(),
            inactive_elements: self.inactive_elements.unwrap_or_default(),
        }
    }
}

impl Snapshotable for VectorEngine {
    type Snapshot = Self;

    fn snapshot(&self) -> Self::Snapshot {
        Self {
            lmul: self.lmul,
            vlen: self.vlen,
            sew: self.sew,
            tail_elements: self.tail_elements,
            inactive_elements: self.inactive_elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        VectorEngineBuilder::default().build();
    }
}
