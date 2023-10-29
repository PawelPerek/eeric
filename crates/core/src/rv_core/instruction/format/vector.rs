/// Configuration formats

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vsetvli {
    pub rd: usize,
    pub rs1: usize,
    pub vtypei: u32,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vsetivli {
    pub rd: usize,
    pub uimm: u32,
    pub vtypei: u32,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vsetvl {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

/// Load/store formats

// Loads

// unit-stride
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vl {
    pub vd: usize,
    pub rs1: usize,
    pub vm: bool,
}

// strided
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vls {
    pub vd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub vm: bool,
}

// indexed
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vlx {
    pub vd: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// whole register
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vlr {
    pub vd: usize,
    pub rs1: usize,
}

// Stores

// unit-stride
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vs {
    pub vs3: usize,
    pub rs1: usize,
    pub vm: bool,
}

// strided
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vss {
    pub vs3: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub vm: bool,
}

// indexed
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vsx {
    pub vs3: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// whole register
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vsr {
    pub vs3: usize,
    pub rs1: usize,
}

/// OPI - Integer instructions

// Vector - Vector
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opivv {
    pub vd: usize,
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// Vector - Scalar (Register)

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opivx {
    pub vd: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// Vector - Immediate
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opivi {
    pub vd: usize,
    pub imm5: i32,
    pub vs2: usize,
    pub vm: bool,
}

/// OPM - Mask instructions

// Vector - Vector
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opmvv {
    pub dest: usize, // Note: can be either vd/rd
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// Vector - Scalar (Register)
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opmvx {
    pub dest: usize, // Note: can be either vd/rd
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

/// OPF - Floating point instructions

// Vector - Vector
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opfvv {
    pub dest: usize, // Note: can be either vd/rd
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

// Vector - Scalar (FP Register)
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Opfvf {
    pub vd: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool,
}

/// Other encoding spaces

// VRXUNARY0 - OPMVX with vs2 as function opcode
pub type Vrxunary0 = Opmvx;

// VWXUNARY0 - OPMVV with rs1 as function opcode
pub type Vwxunary0 = Opmvv;

// VXUNARY0 - OPMVV with vs1 as function opcode
pub type Vxunary0 = Opmvv;

// VMUNARY0 - OPMVV with vs1 as function opcode
pub type Vmunary0 = Opmvv;

// VWFUNARY0 - OPFVV with vs1 as function opcode
pub type Vwfunary0 = Opfvv;

// VRFUNARY0 - OPFVF with vs2 as function opcode
pub type Vrfunary0 = Opfvf;

// VFUNARY0 - OPFVV with vs1 as function opcode
pub type Vfunary0 = Opfvv;

// VFUNARY1 - OPFVV with vs1 as function opcode
pub type Vfunary1 = Opfvv;
