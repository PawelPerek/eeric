use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Example {
    algo: Algorithm,
    variant: Variant,
}

impl Example {
    pub fn all_combinations() -> [Example; 5 * 2] {
        [
            Self {
                algo: Algorithm::Memcpy,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Memcpy,
                variant: Variant::Vector,
            },
            Self {
                algo: Algorithm::Strcpy,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Strcpy,
                variant: Variant::Vector,
            },
            Self {
                algo: Algorithm::Strncpy,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Strncpy,
                variant: Variant::Vector,
            },
            Self {
                algo: Algorithm::Strlen,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Strlen,
                variant: Variant::Vector,
            },
            Self {
                algo: Algorithm::Saxpy,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Saxpy,
                variant: Variant::Vector,
            },
        ]
    }

    pub fn name(&self) -> String {
        format!("{} ({})", self.algo, self.variant)
    }

    pub fn asm(&self) -> &'static str {
        match (self.algo, self.variant) {
            (Algorithm::Memcpy, Variant::Scalar) => include_str!("examples/memcpy_scalar.S"),
            (Algorithm::Memcpy, Variant::Vector) => include_str!("examples/memcpy_vector.S"),
            (Algorithm::Strcpy, Variant::Scalar) => include_str!("examples/strncpy.S"),
            (Algorithm::Strcpy, Variant::Vector) => include_str!("examples/strlen.S"),
            (Algorithm::Strncpy, Variant::Scalar) => include_str!("examples/saxpy.S"),
            (Algorithm::Strncpy, Variant::Vector) => todo!(),
            (Algorithm::Strlen, Variant::Scalar) => todo!(),
            (Algorithm::Strlen, Variant::Vector) => todo!(),
            (Algorithm::Saxpy, Variant::Scalar) => todo!(),
            (Algorithm::Saxpy, Variant::Vector) => todo!(),
        }
    }
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.algo, self.variant)
    }
}

impl Default for Example {
    fn default() -> Self {
        Self {
            algo: Algorithm::Memcpy,
            variant: Variant::Vector,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Algorithm {
    Memcpy,
    Strcpy,
    Strncpy,
    Strlen,
    Saxpy,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let algo_name = match self {
            Self::Memcpy => "memcpy",
            Self::Strcpy => "strcpy",
            Self::Strncpy => "strncpy",
            Self::Strlen => "strlen",
            Self::Saxpy => "saxpy",
        };

        write!(f, "{}", algo_name)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Scalar,
    Vector,
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let algo_name = match self {
            Self::Scalar => "scalar",
            Self::Vector => "vector",
        };

        write!(f, "{}", algo_name)
    }
}
