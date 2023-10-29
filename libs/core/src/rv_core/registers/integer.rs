use crate::{prelude::Snapshotable, rv_core::memory::Memory};

use super::aliases::integer::SP;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct IntegerRegisters([u64; 31]);

impl IntegerRegisters {
    pub fn new(mem: &Memory) -> Self {
        let mut regs: [u64; 31] = Default::default();

        regs[SP - 1] = mem.len() as u64 - 1;

        Self(regs)
    }
}

impl Snapshotable for IntegerRegisters {
    type Snapshot = [u64; 32];

    fn snapshot(&self) -> Self::Snapshot {
        let mut regs = [0; 32];
        regs[1..].copy_from_slice(&self.0);
        regs
    }
}

impl std::ops::Index<usize> for IntegerRegisters {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &0,
            _ => &self.0[index - 1],
        }
    }
}

impl std::ops::IndexMut<usize> for IntegerRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Rust prohibits returning &mut 0 because reference would outlive the value on the stack.
        // Small static variables are basically free so IMO it's the best second choice
        static mut DISCARD_VALUE: u64 = 0;

        match index {
            0 => unsafe { &mut DISCARD_VALUE },
            _ => &mut self.0[index - 1],
        }
    }
}
