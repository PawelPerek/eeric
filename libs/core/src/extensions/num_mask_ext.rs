pub trait NumMaskExt {
    fn with_mask_bit(self, value: u64) -> u64;
    fn get_mask_bit(&self) -> u64;
}

impl NumMaskExt for u64 {
    fn with_mask_bit(self, bit: u64) -> u64 {
        if bit == 1 {
            self | 1
        } else {
            self & !1
        }
    }

    fn get_mask_bit(&self) -> u64 {
        *self & 1
    }
}
