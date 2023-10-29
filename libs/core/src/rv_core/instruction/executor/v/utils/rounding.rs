use crate::rv_core::instruction::executor::prelude::*;

pub enum RoundingMode {
    // Round to Nearest, ties Up
    Rnu,

    // Round to Nearest, ties to Even
    Rne,

    // Round Down (truncate)
    Rdn,

    // Round to Odd
    Rod,
}

pub struct Roundoff {
    pub mode: RoundingMode,
    pub signed: bool,
}

impl Roundoff {
    fn parse_csr(csr: &CsrRegisters) -> RoundingMode {
        match csr[VXRM].read() {
            0b00 => RoundingMode::Rnu,
            0b01 => RoundingMode::Rne,
            0b10 => RoundingMode::Rdn,
            0b11 => RoundingMode::Rod,
            _ => unreachable!(),
        }
    }

    pub fn new_unsigned(csr: &CsrRegisters) -> Self {
        Self {
            mode: Self::parse_csr(csr),
            signed: false,
        }
    }

    pub fn new_signed(csr: &CsrRegisters) -> Self {
        Self {
            mode: Self::parse_csr(csr),
            signed: true,
        }
    }
}

// NOTE: u8 is fine as long as max SEW = 256b, which is the case for now
impl FnOnce<(u128, u8)> for Roundoff {
    type Output = u64;

    extern "rust-call" fn call_once(self, args: (u128, u8)) -> Self::Output {
        if self.signed {
            roundoff_signed_internal(args.0, args.1, &self.mode)
        } else {
            roundoff_unsigned_internal(args.0, args.1, &self.mode)
        }
    }
}

impl FnMut<(u128, u8)> for Roundoff {
    extern "rust-call" fn call_mut(&mut self, args: (u128, u8)) -> Self::Output {
        if self.signed {
            roundoff_signed_internal(args.0, args.1, &self.mode)
        } else {
            roundoff_unsigned_internal(args.0, args.1, &self.mode)
        }
    }
}

impl Fn<(u128, u8)> for Roundoff {
    extern "rust-call" fn call(&self, args: (u128, u8)) -> Self::Output {
        if self.signed {
            roundoff_signed_internal(args.0, args.1, &self.mode)
        } else {
            roundoff_unsigned_internal(args.0, args.1, &self.mode)
        }
    }
}

fn roundoff_unsigned_internal(v: u128, d: u8, mode: &RoundingMode) -> u64 {
    use RoundingMode::*;

    let r = match mode {
        Rnu => v >> (d - 1) & 1,
        Rne => (v >> (d - 1) & 1) & (if (v >> (d - 2)) != 0 { 1 } else { 0 } | (v >> d & 1)),
        Rdn => 0,
        Rod => !(v >> d & 1) & if (v >> (d - 1)) != 0 { 1 } else { 0 },
    };

    ((v >> d) + r) as u64
}

fn roundoff_signed_internal(v: u128, d: u8, mode: &RoundingMode) -> u64 {
    use RoundingMode::*;

    let r = match mode {
        Rnu => v >> (d - 1) & 1,
        Rne => (v >> (d - 1) & 1) & (if (v >> (d - 2)) != 0 { 1 } else { 0 } | (v >> d & 1)),
        Rdn => 0,
        Rod => !(v >> d & 1) & if (v >> (d - 1)) != 0 { 1 } else { 0 },
    };

    ((v as i128 >> d) + r as i128) as u64
}
