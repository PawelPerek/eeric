#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct R4 {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub rs3: usize,
}
