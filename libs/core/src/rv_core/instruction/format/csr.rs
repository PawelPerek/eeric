#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Csrr {
    pub rd: usize,
    pub rs1: usize,
    pub csr: usize,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Csri {
    pub rd: usize,
    pub uimm: usize,
    pub csr: usize,
}
