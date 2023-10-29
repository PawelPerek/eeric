use std::collections::VecDeque;

use super::snapshot::Snapshotable;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Memory {
    raw: VecDeque<u8>,
    data_ptr: usize,
}

impl Snapshotable for Memory {
    type Snapshot = VecDeque<u8>;

    fn snapshot(&self) -> Self::Snapshot {
        self.raw.clone()
    }
}

impl Memory {
    pub fn new(raw: impl ExactSizeIterator<Item = u8>) -> Self {
        Self {
            raw: VecDeque::from_iter(raw),
            data_ptr: 0,
        }
    }

    pub fn get<const BYTES: usize>(&self, address: usize) -> [u8; BYTES] {
        let mut bytes = [0; BYTES];

        bytes[..BYTES].copy_from_slice(&self.raw.as_slices().0[address..(BYTES + address)]);

        bytes
    }

    pub fn fallible_get<const BYTES: usize>(&self, address: usize) -> Option<[u8; BYTES]> {
        let mut bytes = [0; BYTES];

        for (offset, byte_element) in bytes.iter_mut().enumerate().take(BYTES) {
            let Some(byte) = self.raw.get(address + offset).cloned() else {
                return None;
            };

            *byte_element = byte;
        }

        Some(bytes)
    }

    pub fn set<const BYTES: usize>(&mut self, address: usize, value: [u8; BYTES]) {
        self.raw.as_mut_slices().0[address..(BYTES + address)].copy_from_slice(&value[..BYTES]);
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn assign(&mut self, data: &[u8]) {
        self.raw.as_mut_slices().0[self.data_ptr..self.data_ptr + data.len()].copy_from_slice(data);
        self.data_ptr += data.len();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(vec![0; 0x1000].into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_works() {
        let mut mem = Memory::new(vec![0; 0x10].into_iter());

        assert_eq!(mem.data_ptr, 0);

        mem.assign(&[1, 2]);

        assert_eq!(mem.data_ptr, 2);
        assert_eq!(mem.raw, &[1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        mem.assign(&[5, 10, 15]);

        assert_eq!(mem.data_ptr, 5);
        assert_eq!(mem.raw, &[1, 2, 5, 10, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
