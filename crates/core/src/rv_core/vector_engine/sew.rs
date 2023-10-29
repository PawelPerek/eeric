pub trait Sew {
    fn bit_length(&self) -> usize;
    fn byte_length(&self) -> usize {
        self.bit_length() / 8
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum BaseSew {
    #[default]
    E8,
    E16,
    E32,
    E64,
}

impl TryFrom<usize> for BaseSew {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let sew = match value {
            8 => Self::E8,
            16 => Self::E16,
            32 => Self::E32,
            64 => Self::E64,
            _ => {
                return Err(format!(
                    "Expected SEW = 8b, 16b, 32b or 64b, got SEW = {}b",
                    value
                ))
            }
        };

        Ok(sew)
    }
}

impl BaseSew {
    pub fn double(self) -> DoubleSew {
        match self {
            Self::E8 => DoubleSew::E16,
            Self::E16 => DoubleSew::E32,
            Self::E32 => DoubleSew::E64,
            Self::E64 => DoubleSew::E128,
        }
    }

    pub fn half(self) -> Result<HalfSew, String> {
        let halved = match self {
            Self::E64 => HalfSew::E32,
            Self::E32 => HalfSew::E16,
            Self::E16 => HalfSew::E8,
            _ => {
                return Err(format!(
                    "SEW has to be equal 16b, 32b or 64b for halving operations, got SEW = {}b",
                    self.bit_length()
                ))
            }
        };

        Ok(halved)
    }

    pub fn fourth(self) -> Result<FourthSew, String> {
        let fourthed = match self {
            Self::E64 => FourthSew::E16,
            Self::E32 => FourthSew::E8,
            _ => {
                return Err(format!(
                    "SEW has to be equal 32b or 64b for fourthing operations, got SEW = {}b",
                    self.bit_length()
                ))
            }
        };

        Ok(fourthed)
    }

    pub fn eighth(self) -> Result<EighthSew, String> {
        let eighted = match self {
            Self::E64 => EighthSew::E8,
            _ => {
                return Err(format!(
                    "SEW has to be equal 64b for eighthing operations, got SEW = {}b",
                    self.bit_length()
                ))
            }
        };

        Ok(eighted)
    }

    pub fn fp(self) -> Result<FpSew, String> {
        let fp_sew = match self {
            Self::E32 => FpSew::E32,
            Self::E64 => FpSew::E64,
            _ => {
                return Err(format!(
                "When using FP vector operations, SEW has to be either 32b or 64b, got SEW = {}b",
                self.bit_length()
            ))
            }
        };

        Ok(fp_sew)
    }
}

impl Sew for BaseSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E8 => 8,
            Self::E16 => 16,
            Self::E32 => 32,
            Self::E64 => 64,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DoubleSew {
    E16,
    E32,
    E64,
    E128,
}

impl DoubleSew {
    pub fn fp(self) -> Result<DoubleFpSew, String> {
        let fp_sew = match self {
            Self::E64 => DoubleFpSew::E64,
            _ => {
                return Err(format!(
                "When using widening FP vector operations, SEW has to be either 32b, got SEW = {}b",
                self.bit_length() / 2
            ))
            }
        };

        Ok(fp_sew)
    }
}

impl Sew for DoubleSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E16 => 16,
            Self::E32 => 32,
            Self::E64 => 64,
            Self::E128 => 128,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum HalfSew {
    E8,
    E16,
    E32,
}

impl Sew for HalfSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E8 => 8,
            Self::E16 => 16,
            Self::E32 => 32,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FourthSew {
    E8,
    E16,
}

impl Sew for FourthSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E8 => 8,
            Self::E16 => 16,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum EighthSew {
    E8,
}

impl Sew for EighthSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E8 => 8,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FpSew {
    E32,
    E64,
}

impl Sew for FpSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E32 => 32,
            Self::E64 => 64,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DoubleFpSew {
    E64,
}

impl Sew for DoubleFpSew {
    fn bit_length(&self) -> usize {
        match self {
            Self::E64 => 64,
        }
    }
}
