pub mod integer {
    pub const ZERO: usize = 0;
    pub const RA: usize = 1;
    pub const SP: usize = 2;
    pub const GP: usize = 3;
    pub const TP: usize = 4;
    pub const T0: usize = 5;
    pub const T1: usize = 6;
    pub const T2: usize = 7;
    pub const S0: usize = 8;
    pub const S1: usize = 9;
    pub const A0: usize = 10;
    pub const A1: usize = 11;
    pub const A2: usize = 12;
    pub const A3: usize = 13;
    pub const A4: usize = 14;
    pub const A5: usize = 15;
    pub const A6: usize = 16;
    pub const A7: usize = 17;
    pub const S2: usize = 18;
    pub const S3: usize = 19;
    pub const S4: usize = 20;
    pub const S5: usize = 21;
    pub const S6: usize = 22;
    pub const S7: usize = 23;
    pub const S8: usize = 24;
    pub const S9: usize = 25;
    pub const S10: usize = 26;
    pub const S11: usize = 27;
    pub const T3: usize = 28;
    pub const T4: usize = 29;
    pub const T5: usize = 30;
    pub const T6: usize = 31;
}

pub mod csr {
    pub const INSTRET: usize = 0x0c02;
    pub const CYCLE: usize = 0x0c00;
    pub const TIME: usize = 0x0c01;
    pub const MARCHID: usize = 0x0f12;
    pub const FCSR: usize = 0x0003;
    pub const FFLAGS: usize = 0x0001;
    pub const FRM: usize = 0x0002;
    pub const MSTATUS: usize = 0x0300;
    pub const VSSTATUS: usize = 0x0200;
    pub const VTYPE: usize = 0x0c21;
    pub const VL: usize = 0x0C20;
    pub const VLENB: usize = 0x0C22;
    pub const VSTART: usize = 0x0008;
    pub const VXRM: usize = 0x000A;
    pub const VXSAT: usize = 0x0009;
    pub const VCSR: usize = 0x000F;
}

pub mod float {
    pub const FT0: usize = 0;
    pub const FT1: usize = 1;
    pub const FT2: usize = 2;
    pub const FT3: usize = 3;
    pub const FT4: usize = 4;
    pub const FT5: usize = 5;
    pub const FT6: usize = 6;
    pub const FT7: usize = 7;
    pub const FS0: usize = 8;
    pub const FS1: usize = 9;
    pub const FA0: usize = 10;
    pub const FA1: usize = 11;
    pub const FA2: usize = 12;
    pub const FA3: usize = 13;
    pub const FA4: usize = 14;
    pub const FA5: usize = 15;
    pub const FA6: usize = 16;
    pub const FA7: usize = 17;
    pub const FS2: usize = 18;
    pub const FS3: usize = 19;
    pub const FS4: usize = 20;
    pub const FS5: usize = 21;
    pub const FS6: usize = 22;
    pub const FS7: usize = 23;
    pub const FS8: usize = 24;
    pub const FS9: usize = 25;
    pub const FS10: usize = 26;
    pub const FS11: usize = 27;
    pub const FT8: usize = 28;
    pub const FT9: usize = 29;
    pub const FT10: usize = 30;
    pub const FT11: usize = 31;
}

pub mod vector {}
