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
                algo: Algorithm::Daxpy,
                variant: Variant::Scalar,
            },
            Self {
                algo: Algorithm::Daxpy,
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
            (Algorithm::Strcpy, Variant::Scalar) => include_str!("examples/strcpy_scalar.S"),
            (Algorithm::Strcpy, Variant::Vector) => include_str!("examples/strcpy_vector.S"),
            (Algorithm::Strncpy, Variant::Scalar) => include_str!("examples/strncpy_scalar.S"),
            (Algorithm::Strncpy, Variant::Vector) => include_str!("examples/strncpy_vector.S"),
            (Algorithm::Strlen, Variant::Scalar) => include_str!("examples/strlen_scalar.S"),
            (Algorithm::Strlen, Variant::Vector) => include_str!("examples/strlen_vector.S"),
            (Algorithm::Daxpy, Variant::Scalar) => include_str!("examples/daxpy_scalar.S"),
            (Algorithm::Daxpy, Variant::Vector) => include_str!("examples/daxpy_vector.S"),
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
    Daxpy,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let algo_name = match self {
            Self::Memcpy => "memcpy",
            Self::Strcpy => "strcpy",
            Self::Strncpy => "strncpy",
            Self::Strlen => "strlen",
            Self::Daxpy => "daxpy",
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
