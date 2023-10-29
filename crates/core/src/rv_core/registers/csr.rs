use crate::{prelude::Snapshotable, rv_core::vector_engine::Vlen};

use super::aliases::csr::VLENB;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum CsrPrivilege {
    ReadOnly,
    ReadWrite,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CsrRegister {
    value: u64,
    pub privilege: CsrPrivilege,
}

impl CsrRegister {
    pub fn read(&self) -> u64 {
        self.value
    }

    pub fn write(&mut self, value: u64) -> Result<(), String> {
        if self.privilege == CsrPrivilege::ReadOnly {
            return Err("Cannot write to read-only register".to_owned());
        }

        unsafe {
            self.set(value);
        }

        Ok(())
    }

    pub unsafe fn set(&mut self, value: u64) {
        self.value = value;
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CsrRegisters([CsrRegister; 4096]);

impl Snapshotable for CsrRegisters {
    type Snapshot = [CsrRegister; 4096];

    fn snapshot(&self) -> Self::Snapshot {
        self.0.clone()
    }
}

impl CsrRegisters {
    pub fn new(vlen: &Vlen) -> Self {
        let mut regs = Self::default();

        unsafe { regs[VLENB].set(vlen.byte_length() as u64) }

        regs
    }
}

impl Default for CsrRegisters {
    fn default() -> Self {
        let mut index = 0;
        Self([0; 4096].map(|_| {
            let privilege = if ((index >> 10) & 0b11) == 0b11 {
                CsrPrivilege::ReadOnly
            } else {
                CsrPrivilege::ReadWrite
            };
            let register = CsrRegister {
                value: 0,
                privilege,
            };

            index += 1;

            register
        }))
    }
}

impl std::ops::Index<usize> for CsrRegisters {
    type Output = CsrRegister;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for CsrRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
