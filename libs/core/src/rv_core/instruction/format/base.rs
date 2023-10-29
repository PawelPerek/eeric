#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct R {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct I {
    pub rd: usize,
    pub rs1: usize,
    pub imm12: i32,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct S {
    pub rs1: usize,
    pub rs2: usize,
    pub imm12: i32,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct U {
    pub rd: usize,
    pub imm20: i32,
}
