/// Vector length multiplier
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Lmul {
    // Fractional multipliers
    MF8,
    MF4,
    MF2,

    // Integer multipliers
    #[default]
    M1,
    M2,
    M4,
    M8,
}

impl Lmul {
    pub fn multiply(&self, other: usize) -> usize {
        match self {
            Self::MF8 => other / 8,
            Self::MF4 => other / 4,
            Self::MF2 => other / 2,
            Self::M1 => other,
            Self::M2 => other * 2,
            Self::M4 => other * 4,
            Self::M8 => other * 8,
        }
    }

    pub fn ratio(&self) -> f32 {
        match self {
            Self::MF8 => 0.125,
            Self::MF4 => 0.25,
            Self::MF2 => 0.5,
            Self::M1 => 1.0,
            Self::M2 => 2.0,
            Self::M4 => 4.0,
            Self::M8 => 8.0,
        }
    }

    pub fn double(self) -> Result<Self, &'static str> {
        let doubled = match self {
            Self::MF8 => Self::MF4,
            Self::MF4 => Self::MF2,
            Self::MF2 => Self::M1,
            Self::M1 => Self::M2,
            Self::M2 => Self::M4,
            Self::M4 => Self::M8,
            Self::M8 => return Err("Doubling LMUL=8 is reserved"),
        };

        Ok(doubled)
    }
}

impl TryFrom<f32> for Lmul {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let lmul = if value == 0.125 {
            Lmul::MF8
        } else if value == 0.25 {
            Lmul::MF4
        } else if value == 0.5 {
            Lmul::MF2
        } else if value == 1.0 {
            Lmul::M1
        } else if value == 2.0 {
            Lmul::M2
        } else if value == 4.0 {
            Lmul::M4
        } else if value == 8.0 {
            Lmul::M8
        } else {
            return Err(format!("Cannot parse {} into LMUL", value));
        };

        Ok(lmul)
    }
}
