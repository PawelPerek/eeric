pub mod aliases;
mod csr;
mod float;
mod integer;
pub mod vector;

use super::{memory::Memory, snapshot::Snapshotable, vector_engine::VectorEngine};

pub use csr::{CsrPrivilege, CsrRegister, CsrRegisters};
pub use float::FloatRegisters;
pub use integer::IntegerRegisters;
pub use vector::VectorRegisters;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Registers {
    pub pc: u64,
    pub x: IntegerRegisters,
    pub c: CsrRegisters,
    pub f: FloatRegisters,
    pub v: VectorRegisters,
}

impl Registers {
    pub fn new(vec_engine: &VectorEngine, mem: &Memory) -> Self {
        Self {
            pc: 0,
            x: IntegerRegisters::new(mem),
            c: CsrRegisters::new(&vec_engine.vlen),
            f: FloatRegisters::default(),
            v: VectorRegisters::default(vec_engine),
        }
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RegistersSnapshot {
    pub x: [u64; 32],
    pub c: [CsrRegister; 4096],
    pub f: [f64; 32],
    pub v: Vec<u8>,
    pub pc: u64,
}

impl Snapshotable for Registers {
    type Snapshot = RegistersSnapshot;

    fn snapshot(&self) -> RegistersSnapshot {
        RegistersSnapshot {
            x: self.x.snapshot(),
            c: self.c.snapshot(),
            f: self.f.snapshot(),
            v: self.v.snapshot(),
            pc: self.pc,
        }
    }
}

impl Default for RegistersSnapshot {
    fn default() -> Self {
        Self {
            x: [0; 32],
            c: CsrRegisters::default().snapshot(),
            f: [0.0; 32],
            v: Vec::new(),
            pc: 0,
        }
    }
}
