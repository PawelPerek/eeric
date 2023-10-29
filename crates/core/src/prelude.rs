pub use crate::rv_core::{
    instruction::{format, Instruction},
    memory::Memory,
    registers::{CsrPrivilege, Registers, RegistersSnapshot},
    snapshot::Snapshotable,
    vector_engine::{
        sew::{BaseSew, Sew},
        Lmul, MaskBehavior, VectorEngine, VectorEngineBuilder, Vlen,
    },
    RvCore, RvCoreBuilder,
};

pub mod alias {
    pub use crate::rv_core::registers::aliases::csr::*;
    pub use crate::rv_core::registers::aliases::float::*;
    pub use crate::rv_core::registers::aliases::integer::*;
    pub use crate::rv_core::registers::aliases::vector::*;
}

#[macro_export]
macro_rules! fuse {
    ($($instr:expr),+ $(,)? ) => {
        Instruction::Fusion(vec![$($instr),+].into_boxed_slice())
    };
}
