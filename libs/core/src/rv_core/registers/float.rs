use crate::prelude::Snapshotable;

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FloatRegisters([f64; 32]);

impl Snapshotable for FloatRegisters {
    type Snapshot = [f64; 32];

    fn snapshot(&self) -> Self::Snapshot {
        self.0
    }
}

impl std::ops::Index<usize> for FloatRegisters {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for FloatRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
