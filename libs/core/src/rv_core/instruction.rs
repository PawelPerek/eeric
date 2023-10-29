pub mod executor;
pub mod format;

use format::*;

use super::vector_engine::sew::BaseSew;

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    /// Base instructions
    // Arithmetic Operations
    Add(R),
    Addw(R),
    Sub(R),
    Subw(R),
    Addi(I),
    Addiw(I),
    Slt(R),
    Slti(I),
    Sltu(R),
    Sltiu(I),
    Lui(U),
    Auipc(U),

    // Logic operations
    And(R),
    Or(R),
    Xor(R),
    Andi(I),
    Ori(I),
    Xori(I),
    Sll(R),
    Sllw(R),
    Srl(R),
    Srlw(R),
    Sra(R),
    Sraw(R),
    Slli(I),
    Slliw(I),
    Srli(I),
    Srliw(I),
    Srai(I),
    Sraiw(I),

    // Load/store operations
    Ld(I),
    Lw(I),
    Lh(I),
    Lb(I),
    Lwu(I),
    Lhu(I),
    Lbu(I),
    Sd(S),
    Sw(S),
    Sh(S),
    Sb(S),

    // Branching
    Beq(S),
    Bne(S),
    Bge(S),
    Bgeu(S),
    Blt(S),
    Bltu(S),
    Jal(U),
    Jalr(I),

    /// Zicsr extension
    // RV32/64Zicsr
    Csrrw(Csrr),
    Csrrs(Csrr),
    Csrrc(Csrr),
    Csrrwi(Csri),
    Csrrsi(Csri),
    Csrrci(Csri),

    /// M extension
    // RV32M
    Mul(R),
    Mulh(R),
    Mulhsu(R),
    Mulhu(R),
    Div(R),
    Divu(R),
    Rem(R),
    Remu(R),

    // RV64M
    Mulw(R),
    Divw(R),
    Divuw(R),
    Remw(R),
    Remuw(R),

    /// F extension
    // RV32F
    Flw(I),
    Fsw(S),
    Fmadds(R4),
    Fmsubs(R4),
    Fnmsubs(R4),
    Fnmadds(R4),
    Fadds(R),
    Fsubs(R),
    Fmuls(R),
    Fdivs(R),
    Fsqrts(R),
    Fsgnjs(R),
    Fsgnjns(R),
    Fsgnjxs(R),
    Fmins(R),
    Fmaxs(R),
    Fcvtws(R),
    Fcvtwus(R),
    Fmvxw(R),
    Feqs(R),
    Flts(R),
    Fles(R),
    Fclasss(R),
    Fcvtsw(R),
    Fcvtswu(R),
    Fmvwx(R),

    // RV64F
    Fcvtls(R),
    Fcvtlus(R),
    Fcvtsl(R),
    Fcvtslu(R),

    /// D extension
    // RV32D
    Fld(I),
    Fsd(S),
    Fmaddd(R4),
    Fmsubd(R4),
    Fnmsubd(R4),
    Fnmaddd(R4),
    Faddd(R),
    Fsubd(R),
    Fmuld(R),
    Fdivd(R),
    Fsqrtd(R),
    Fsgnjd(R),
    Fsgnjnd(R),
    Fsgnjxd(R),
    Fmind(R),
    Fmaxd(R),
    Fcvtsd(R),
    Fcvtds(R),
    Feqd(R),
    Fltd(R),
    Fled(R),
    Fclassd(R),
    Fcvtwd(R),
    Fcvtwud(R),
    Fcvtdw(R),
    Fcvtdwu(R),

    // RV64D
    Fcvtld(R),
    Fcvtlud(R),
    Fmvxd(R),
    Fcvtdl(R),
    Fcvtdlu(R),
    Fmvdx(R),

    /// V extension
    // VCFG
    Vsetvli(Vsetvli),
    Vsetivli(Vsetivli),
    Vsetvl(Vsetvl),

    // VMEM
    Vlv {
        data: Vl,
        eew: BaseSew,
    },
    Vsv {
        data: Vs,
        eew: BaseSew,
    },

    Vlmv(Vl),
    Vsmv(Vs),

    Vlsv {
        data: Vls,
        eew: BaseSew,
    },
    Vssv {
        data: Vss,
        eew: BaseSew,
    },

    Vluxv {
        data: Vlx,
        eew: BaseSew,
    },
    Vloxv {
        data: Vlx,
        eew: BaseSew,
    },
    Vsuxv {
        data: Vsx,
        eew: BaseSew,
    },
    Vsoxv {
        data: Vsx,
        eew: BaseSew,
    },

    Vlffv {
        data: Vl,
        eew: BaseSew,
    },

    Vlsegv {
        data: Vl,
        eew: BaseSew,
        nf: usize,
    },
    Vssegv {
        data: Vs,
        eew: BaseSew,
        nf: usize,
    },

    Vlssegv {
        data: Vls,
        eew: BaseSew,
        nf: usize,
    },
    Vsssegv {
        data: Vss,
        eew: BaseSew,
        nf: usize,
    },

    Vluxsegv {
        data: Vlx,
        eew: BaseSew,
        nf: usize,
    },
    Vloxsegv {
        data: Vlx,
        eew: BaseSew,
        nf: usize,
    },
    Vsuxsegv {
        data: Vsx,
        eew: BaseSew,
        nf: usize,
    },
    Vsoxsegv {
        data: Vsx,
        eew: BaseSew,
        nf: usize,
    },

    Vlrv {
        data: Vlr,
        eew: BaseSew,
        nf: usize,
    },
    Vsrv {
        data: Vsr,
        nf: usize,
    },

    // VALU
    Vaddvv(Opivv),
    Vaddvx(Opivx),
    Vaddvi(Opivi),

    Vsubvv(Opivv),
    Vsubvx(Opivx),

    Vrsubvx(Opivx),
    Vrsubvi(Opivi),

    Vminuvv(Opivv),
    Vminuvx(Opivx),

    Vminvv(Opivv),
    Vminvx(Opivx),

    Vmaxuvv(Opivv),
    Vmaxuvx(Opivx),

    Vmaxvv(Opivv),
    Vmaxvx(Opivx),

    Vandvv(Opivv),
    Vandvx(Opivx),
    Vandvi(Opivi),

    Vorvv(Opivv),
    Vorvx(Opivx),
    Vorvi(Opivi),

    Vxorvv(Opivv),
    Vxorvx(Opivx),
    Vxorvi(Opivi),

    Vrgathervv(Opivv),
    Vrgathervx(Opivx),
    Vrgathervi(Opivi),

    Vrgatherei16vv(Opivv),

    Vslideupvx(Opivx),
    Vslideupvi(Opivi),

    Vslidedownvx(Opivx),
    Vslidedownvi(Opivi),

    Vadcvvm(Opivv),
    Vadcvxm(Opivx),
    Vadcvim(Opivi),

    Vmadcvvm(Opivv),
    Vmadcvxm(Opivx),
    Vmadcvim(Opivi),
    Vmadcvv(Opivv),
    Vmadcvx(Opivx),
    Vmadcvi(Opivi),

    Vsbcvvm(Opivv),
    Vsbcvxm(Opivx),

    Vmsbcvvm(Opivv),
    Vmsbcvxm(Opivx),

    Vmsbcvv(Opivv),
    Vmsbcvx(Opivx),

    Vmergevvm(Opivv),
    Vmergevxm(Opivx),
    Vmergevim(Opivi),

    Vmvvv(Opivv),
    Vmvvx(Opivx),
    Vmvvi(Opivi),

    Vmseqvv(Opivv),
    Vmseqvx(Opivx),
    Vmseqvi(Opivi),

    Vmsnevv(Opivv),
    Vmsnevx(Opivx),
    Vmsnevi(Opivi),

    Vmsltuvv(Opivv),
    Vmsltuvx(Opivx),

    Vmsltvv(Opivv),
    Vmsltvx(Opivx),

    Vmsleuvv(Opivv),
    Vmsleuvx(Opivx),
    Vmsleuvi(Opivi),

    Vmslevv(Opivv),
    Vmslevx(Opivx),
    Vmslevi(Opivi),

    Vmsgtuvx(Opivx),
    Vmsgtuvi(Opivi),

    Vmsgtvx(Opivx),
    Vmsgtvi(Opivi),

    Vsadduvv(Opivv),
    Vsadduvx(Opivx),
    Vsadduvi(Opivi),

    Vsaddvv(Opivv),
    Vsaddvx(Opivx),
    Vsaddvi(Opivi),

    Vssubuvv(Opivv),
    Vssubuvx(Opivx),

    Vssubvv(Opivv),
    Vssubvx(Opivx),

    Vsllvv(Opivv),
    Vsllvx(Opivx),
    Vsllvi(Opivi),

    Vsmulvv(Opivv),
    Vsmulvx(Opivx),

    Vmv1rv(Opivi),
    Vmv2rv(Opivi),
    Vmv4rv(Opivi),
    Vmv8rv(Opivi),

    Vsrlvv(Opivv),
    Vsrlvx(Opivx),
    Vsrlvi(Opivi),

    Vsravv(Opivv),
    Vsravx(Opivx),
    Vsravi(Opivi),

    Vssrlvv(Opivv),
    Vssrlvx(Opivx),
    Vssrlvi(Opivi),

    Vssravv(Opivv),
    Vssravx(Opivx),
    Vssravi(Opivi),

    Vnsrlwv(Opivv),
    Vnsrlwx(Opivx),
    Vnsrlwi(Opivi),

    Vnsrawv(Opivv),
    Vnsrawx(Opivx),
    Vnsrawi(Opivi),

    Vnclipuwv(Opivv),
    Vnclipuwx(Opivx),
    Vnclipuwi(Opivi),

    Vnclipwv(Opivv),
    Vnclipwx(Opivx),
    Vnclipwi(Opivi),

    Vwredsumuvs(Opivv),

    Vwredsumvs(Opivv),

    Vredsumvs(Opmvv),

    Vredandvs(Opmvv),

    Vredorvs(Opmvv),

    Vredxorvs(Opmvv),

    Vredminuvs(Opmvv),

    Vredminvs(Opmvv),

    Vredmaxuvs(Opmvv),

    Vredmaxvs(Opmvv),

    Vaadduvv(Opmvv),
    Vaadduvx(Opmvx),

    Vaaddvv(Opmvv),
    Vaaddvx(Opmvx),

    Vasubuvv(Opmvv),
    Vasubuvx(Opmvx),

    Vasubvv(Opmvv),
    Vasubvx(Opmvx),

    Vslide1upvx(Opmvx),

    Vslide1downvx(Opmvx),

    Vmvxs(Vwxunary0),
    Vcpopm(Vwxunary0),
    Vfirstm(Vwxunary0),

    Vmvsx(Vrxunary0),

    Vzextvf8(Vxunary0),
    Vsextvf8(Vxunary0),
    Vzextvf4(Vxunary0),
    Vsextvf4(Vxunary0),
    Vzextvf2(Vxunary0),
    Vsextvf2(Vxunary0),

    Vmsbfm(Vmunary0),
    Vmsofm(Vmunary0),
    Vmsifm(Vmunary0),
    Viotam(Vmunary0),
    Vidv(Vmunary0),

    Vcompressvm(Opmvv),

    Vmandnmm(Opmvv),

    Vmandmm(Opmvv),

    Vmormm(Opmvv),

    Vmxormm(Opmvv),

    Vmornmm(Opmvv),

    Vmnandmm(Opmvv),

    Vmnormm(Opmvv),

    Vmxnormm(Opmvv),

    Vdivuvv(Opmvv),
    Vdivuvx(Opmvx),

    Vdivvv(Opmvv),
    Vdivvx(Opmvx),

    Vremuvv(Opmvv),
    Vremuvx(Opmvx),

    Vremvv(Opmvv),
    Vremvx(Opmvx),

    Vmulhuvv(Opmvv),
    Vmulhuvx(Opmvx),

    Vmulvv(Opmvv),
    Vmulvx(Opmvx),

    Vmulhsuvv(Opmvv),
    Vmulhsuvx(Opmvx),

    Vmulhvv(Opmvv),
    Vmulhvx(Opmvx),

    Vmaddvv(Opmvv),
    Vmaddvx(Opmvx),

    Vnmsubvv(Opmvv),
    Vnmsubvx(Opmvx),

    Vmaccvv(Opmvv),
    Vmaccvx(Opmvx),

    Vnmsacvv(Opmvv),
    Vnmsacvx(Opmvx),

    Vwadduvv(Opmvv),
    Vwadduvx(Opmvx),

    Vwaddvv(Opmvv),
    Vwaddvx(Opmvx),

    Vwsubuvv(Opmvv),
    Vwsubuvx(Opmvx),

    Vwsubvv(Opmvv),
    Vwsubvx(Opmvx),

    Vwadduwv(Opmvv),
    Vwadduwx(Opmvx),

    Vwaddwv(Opmvv),
    Vwaddwx(Opmvx),

    Vwsubuwv(Opmvv),
    Vwsubuwx(Opmvx),

    Vwsubwv(Opmvv),
    Vwsubwx(Opmvx),

    Vwmuluvv(Opmvv),
    Vwmuluvx(Opmvx),

    Vwmulsuvv(Opmvv),
    Vwmulsuvx(Opmvx),

    Vwmulvv(Opmvv),
    Vwmulvx(Opmvx),

    Vwmaccuvv(Opmvv),
    Vwmaccuvx(Opmvx),

    Vwmaccvv(Opmvv),
    Vwmaccvx(Opmvx),

    Vwmaccusvx(Opmvx),

    Vwmaccsuvv(Opmvv),
    Vwmaccsuvx(Opmvx),

    Vfaddvv(Opfvv),
    Vfaddvf(Opfvf),

    Vfredusumvs(Opfvv),

    Vfsubvv(Opfvv),
    Vfsubvf(Opfvf),

    Vfredosumvs(Opfvv),

    Vfminvv(Opfvv),
    Vfminvf(Opfvf),

    Vfredminvs(Opfvv),

    Vfmaxvv(Opfvv),
    Vfmaxvf(Opfvf),

    Vfredmaxvs(Opfvv),

    Vfsgnjvv(Opfvv),
    Vfsgnjvf(Opfvf),

    Vfsgnjnvv(Opfvv),
    Vfsgnjnvf(Opfvf),

    Vfsgnjxvv(Opfvv),
    Vfsgnjxvf(Opfvf),

    Vfslide1upvf(Opfvf),

    Vfslide1downvf(Opfvf),

    Vfmvfs(Vwfunary0),

    Vfmvsf(Vrfunary0),

    Vfcvtxufv(Vfunary0),
    Vfcvtxfv(Vfunary0),
    Vfcvtfxuv(Vfunary0),
    Vfcvtfxv(Vfunary0),
    VfcvtRtzxufv(Vfunary0),
    VfcvtRtzxfv(Vfunary0),

    Vfwcvtxufv(Vfunary0),
    Vfwcvtxfv(Vfunary0),
    Vfwcvtfxuv(Vfunary0),
    Vfwcvtfxv(Vfunary0),
    Vfwcvtffv(Vfunary0),
    VfwcvtRtzxufv(Vfunary0),
    VfwcvtRtzxfv(Vfunary0),

    Vfncvtxufw(Vfunary0),
    Vfncvtxfw(Vfunary0),
    Vfncvtfxuw(Vfunary0),
    Vfncvtfxw(Vfunary0),
    Vfncvtffw(Vfunary0),
    VfncvtRodffw(Vfunary0),
    VfncvtRtzxufw(Vfunary0),
    VfncvtRtzxfw(Vfunary0),

    Vfsqrtv(Vfunary1),
    Vfrsqrt7v(Vfunary1),
    Vfrec7v(Vfunary1),
    Vfclassv(Vfunary1),

    Vfmergevfm(Opfvf),
    Vfmvvf(Opfvf),

    Vmfeqvv(Opfvv),
    Vmfeqvf(Opfvf),

    Vmflevv(Opfvv),
    Vmflevf(Opfvf),

    Vmfltvv(Opfvv),
    Vmfltvf(Opfvf),

    Vmfnevv(Opfvv),
    Vmfnevf(Opfvf),

    Vmfgtvf(Opfvf),

    Vmfgevf(Opfvf),

    Vfdivvv(Opfvv),
    Vfdivvf(Opfvf),

    Vfrdivvf(Opfvf),

    Vfmulvv(Opfvv),
    Vfmulvf(Opfvf),

    Vfrsubvf(Opfvf),

    Vfmaddvv(Opfvv),
    Vfmaddvf(Opfvf),

    Vfnmaddvv(Opfvv),
    Vfnmaddvf(Opfvf),

    Vfmsubvv(Opfvv),
    Vfmsubvf(Opfvf),

    Vfnmsubvv(Opfvv),
    Vfnmsubvf(Opfvf),

    Vfmaccvv(Opfvv),
    Vfmaccvf(Opfvf),

    Vfnmaccvv(Opfvv),
    Vfnmaccvf(Opfvf),

    Vfmsacvv(Opfvv),
    Vfmsacvf(Opfvf),

    Vfnmsacvv(Opfvv),
    Vfnmsacvf(Opfvf),

    Vfwaddvv(Opfvv),
    Vfwaddvf(Opfvf),

    Vfwredusumvs(Opfvv),

    Vfwsubvv(Opfvv),
    Vfwsubvf(Opfvf),

    Vfwredosumvs(Opfvv),

    Vfwaddwv(Opfvv),
    Vfwaddwf(Opfvf),

    Vfwsubwv(Opfvv),
    Vfwsubwf(Opfvf),

    Vfwmulvv(Opfvv),
    Vfwmulvf(Opfvf),

    Vfwmaccvv(Opfvv),
    Vfwmaccvf(Opfvf),

    Vfwnmaccvv(Opfvv),
    Vfwnmaccvf(Opfvf),

    Vfwmsacvv(Opfvv),
    Vfwmsacvf(Opfvf),

    Vfwnmsacvv(Opfvv),
    Vfwnmsacvf(Opfvf),

    // Instruction fusion for pseudo instructions
    Fusion(Box<[Instruction]>),
}
