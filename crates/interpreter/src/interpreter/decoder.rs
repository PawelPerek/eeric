mod data;
mod operand;

use std::{ascii, cmp::Ordering, collections::HashMap};

use eeric_core::{
    fuse,
    prelude::{format::*, *},
};
use operand::{csr, float, integer, vector};
use Instruction::*;

pub struct Decoder;

#[derive(Clone)]
pub enum Section {
    Data,
    Text,
}

pub enum Data {
    Byte(Vec<u8>),
    Half(Vec<u16>),
    Word(Vec<u32>),
    Quad(Vec<u64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    Ascii(Vec<ascii::Char>),
    Zero(usize),
}

impl From<Data> for Vec<u8> {
    fn from(val: Data) -> Self {
        match val {
            Data::Byte(bytes) => bytes,
            Data::Half(halves) => halves.into_iter().flat_map(u16::to_le_bytes).collect(),
            Data::Word(words) => words.into_iter().flat_map(u32::to_le_bytes).collect(),
            Data::Quad(quads) => quads.into_iter().flat_map(u64::to_le_bytes).collect(),
            Data::Float(floats) => floats.into_iter().flat_map(f32::to_le_bytes).collect(),
            Data::Double(doubles) => doubles.into_iter().flat_map(f64::to_le_bytes).collect(),
            Data::Ascii(string) => string.into_iter().map(|char| char as u8).collect(),
            Data::Zero(length) => std::iter::repeat(0).take(length).collect(),
        }
    }
}

pub enum AssemblerDirective {
    Section(Section),
    Data(Data),
}
pub enum PreprocDirective {
    Define(String, String),
}

pub enum LineClassification {
    AssemblerDirective(AssemblerDirective),
    PreprocDirective(PreprocDirective),
    Instruction(String),
    Label(String),
    Empty,
}

impl Decoder {
    pub fn classify(line: &str) -> Result<LineClassification, String> {
        let trimmed_line = line.split('#').next().unwrap_or("").trim();

        Ok(if trimmed_line.is_empty() {
            LineClassification::Empty
        } else if trimmed_line.starts_with('#') {
            match trimmed_line {
                "#define" => return Err("#define directive not supported yet".to_owned()),
                _ => {
                    return Err(format!(
                        "Unrecognized preprocessor directive: {}",
                        trimmed_line
                    ))
                }
            }
        } else if let Some(label) = trimmed_line.strip_suffix(':') {
            if !label.contains(' ') {
                LineClassification::Label(label.to_string())
            } else {
                return Err(format!("Label {} cannot have whitespaces", label));
            }
        } else if trimmed_line.starts_with('.') {
            match trimmed_line {
                ".section .data" | ".data" => LineClassification::AssemblerDirective(
                    AssemblerDirective::Section(Section::Data),
                ),
                ".section .text" | ".text" => LineClassification::AssemblerDirective(
                    AssemblerDirective::Section(Section::Text),
                ),
                _ => LineClassification::AssemblerDirective(AssemblerDirective::Data(
                    Self::decode_data_section(trimmed_line)?,
                )),
            }
        } else {
            LineClassification::Instruction(trimmed_line.to_string())
        })
    }

    pub fn decode_text_section(
        instruction_line: &str,
        instruction_labels: &HashMap<String, usize>,
        memory_labels: &HashMap<String, usize>,
        current_address: usize,
    ) -> Result<Instruction, String> {
        let (mnemonic, op) = Self::split_instruction(instruction_line);

        use integer::{
            parse_branch_format as b, parse_i_format as i, parse_load_format as l,
            parse_r_format as r, parse_s_format as s, parse_u_format as u,
        };

        use csr::{parse_csri_format as csri, parse_csrr_format as csrr};

        use float::{
            parse_load_format as fl, parse_r4_format as r4, parse_r_format as fr,
            parse_r_single_reg_format as frs, parse_r_single_reg_to_f_format as frs_to_f,
            parse_r_single_reg_to_x_format as frs_to_x, parse_r_to_x_format as frx,
            parse_store_format as fs,
        };

        use vector::{
            parse_opfvf_fma_format as opfvf_fma, parse_opfvf_format as opfvf,
            parse_opfvv_fma_format as opfvv_fma, parse_opfvv_format as opfvv,
            parse_opivi_format as opivi, parse_opivi_maskless_format as opivi_maskless,
            parse_opivi_v0_format as opivi_v0, parse_opivi_vmv_format as opivi_vmv,
            parse_opivv_format as opivv, parse_opivv_maskless_format as opivv_maskless,
            parse_opivv_v0_format as opivv_v0, parse_opivv_vmv_format as opivv_vmv,
            parse_opivx_format as opivx, parse_opivx_maskless_format as opivx_maskless,
            parse_opivx_v0_format as opivx_v0, parse_opivx_vmv_format as opivx_vmv,
            parse_opmvv_fma_format as opmvv_fma, parse_opmvv_format as opmvv,
            parse_opmvv_maskless_format as opmvv_maskless, parse_opmvx_fma_format as opmvx_fma,
            parse_opmvx_format as opmvx, parse_vfunary0_format as vfunary0,
            parse_vfunary1_format as vfunary1, parse_vl_format as vl, parse_vlm_format as vlm,
            parse_vlr_format as vlr, parse_vls_format as vls, parse_vlx_format as vlx,
            parse_vmunary0_format as vmunary0, parse_vmunary0_vidv_format as vidv,
            parse_vrfunary0_format as vrfunary0, parse_vrxunary0_format as vrxunary0,
            parse_vs_format as vs, parse_vsetivli_format as vsetivli,
            parse_vsetvl_format as vsetvl, parse_vsetvli_format as vsetvli,
            parse_vsm_format as vsm, parse_vsr_format as vsr, parse_vss_format as vss,
            parse_vsx_format as vsx, parse_vwfunary0_format as vwfunary0,
            parse_vwxunary0_format as vwxunary0, parse_vwxunary0_vmvxs_format as vmvxs,
            parse_vxunary0_format as vxunary0,
        };

        let instruction = match Self::rename(mnemonic) {
            "add" => Add(r(op)?),
            "addw" => Addw(r(op)?),
            "sub" => Sub(r(op)?),
            "subw" => Subw(r(op)?),
            "addi" => Addi(i(op)?),
            "addiw" => Addiw(i(op)?),
            "slt" => Slt(r(op)?),
            "slti" => Slti(i(op)?),
            "sltu" => Sltu(r(op)?),
            "sltiu" => Sltiu(i(op)?),
            "lui" => Lui(u(op)?),
            "auipc" => Auipc(u(op)?),

            "and" => And(r(op)?),
            "or" => Or(r(op)?),
            "xor" => Xor(r(op)?),
            "andi" => Andi(i(op)?),
            "ori" => Ori(i(op)?),
            "xori" => Xori(i(op)?),
            "sll" => Sll(r(op)?),
            "sllw" => Sllw(r(op)?),
            "srl" => Srl(r(op)?),
            "srlw" => Srlw(r(op)?),
            "sra" => Sra(r(op)?),
            "sraw" => Sraw(r(op)?),
            "slli" => Slli(i(op)?),
            "slliw" => Slliw(i(op)?),
            "srli" => Srli(i(op)?),
            "srliw" => Srliw(i(op)?),
            "srai" => Srai(i(op)?),
            "sraiw" => Sraiw(i(op)?),

            "ld" => Ld(l(op, memory_labels)?),
            "lw" => Lw(l(op, memory_labels)?),
            "lh" => Lh(l(op, memory_labels)?),
            "lb" => Lb(l(op, memory_labels)?),
            "lwu" => Lwu(l(op, memory_labels)?),
            "lhu" => Lhu(l(op, memory_labels)?),
            "lbu" => Lbu(l(op, memory_labels)?),
            "sd" => Sd(s(op, memory_labels)?),
            "sw" => Sw(s(op, memory_labels)?),
            "sh" => Sh(s(op, memory_labels)?),
            "sb" => Sb(s(op, memory_labels)?),

            "beq" => Beq(b(op, instruction_labels, current_address)?),
            "bne" => Bne(b(op, instruction_labels, current_address)?),
            "bge" => Bge(b(op, instruction_labels, current_address)?),
            "bgeu" => Bgeu(b(op, instruction_labels, current_address)?),
            "blt" => Blt(b(op, instruction_labels, current_address)?),
            "bltu" => Bltu(b(op, instruction_labels, current_address)?),
            "jal" => match u(op) {
                Ok(instruction) => Jal(instruction),
                Err(fst_err) => {
                    match integer::pseudo::parse_label_format(
                        op,
                        instruction_labels,
                        current_address,
                    ) {
                        Ok(diff) => Jal(U { rd: 1, imm20: diff }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                    }
                }
            },
            "jalr" => match l(op, memory_labels) {
                Ok(instruction) => Jalr(instruction),
                Err(fst_err) => match integer::pseudo::parse_op_format(op) {
                    Ok(rs1) => Jalr(I {
                        rd: 1,
                        rs1,
                        imm12: 0,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },

            "csrrw" => Csrrw(csrr(op)?),
            "csrrs" => Csrrs(csrr(op)?),
            "csrrc" => Csrrc(csrr(op)?),
            "csrrwi" => Csrrwi(csri(op)?),
            "csrrsi" => Csrrsi(csri(op)?),
            "csrrci" => Csrrci(csri(op)?),

            "mul" => Mul(r(op)?),
            "mulh" => Mulh(r(op)?),
            "mulhsu" => Mulhsu(r(op)?),
            "mulhu" => Mulhu(r(op)?),
            "div" => Div(r(op)?),
            "divu" => Divu(r(op)?),
            "rem" => Rem(r(op)?),
            "remu" => Remu(r(op)?),
            "mulw" => Mulw(r(op)?),
            "divw" => Divw(r(op)?),
            "divuw" => Divuw(r(op)?),
            "remw" => Remw(r(op)?),
            "remuw" => Remuw(r(op)?),

            "flw" => Flw(fl(op, memory_labels)?),
            "fsw" => Fsw(fs(op, memory_labels)?),
            "fmadd.s" => Fmadds(r4(op)?),
            "fmsub.s" => Fmsubs(r4(op)?),
            "fnmsub.s" => Fnmsubs(r4(op)?),
            "fnmadd.s" => Fnmadds(r4(op)?),
            "fadd.s" => Fadds(fr(op)?),
            "fsub.s" => Fsubs(fr(op)?),
            "fmul.s" => Fmuls(fr(op)?),
            "fdiv.s" => Fdivs(frs(op)?),
            "fsqrt.s" => Fsqrts(fr(op)?),
            "fsgnj.s" => Fsgnjs(fr(op)?),
            "fsgnjn.s" => Fsgnjns(fr(op)?),
            "fsgnjx.s" => Fsgnjxs(fr(op)?),
            "fmin.s" => Fmins(fr(op)?),
            "fmax.s" => Fmaxs(fr(op)?),
            "fcvt.w.s" => Fcvtws(frs_to_x(op)?),
            "fcvt.wu.s" => Fcvtwus(frs_to_x(op)?),
            "fmv.x.w" => Fmvxw(frs_to_x(op)?),
            "feq.s" => Feqs(frx(op)?),
            "flt.s" => Flts(frx(op)?),
            "fle.s" => Fles(frx(op)?),
            "fclass.s" => Fclasss(frs_to_x(op)?),
            "fcvt.s.w" => Fcvtsw(frs_to_f(op)?),
            "fcvt.s.wu" => Fcvtswu(frs_to_f(op)?),
            "fmv.w.x" => Fmvwx(frs_to_f(op)?),
            "fcvt.l.s" => Fcvtls(frs_to_x(op)?),
            "fcvt.lu.s" => Fcvtlus(frs_to_x(op)?),
            "fcvt.s.l" => Fcvtsl(frs_to_f(op)?),
            "fcvt.s.lu" => Fcvtslu(frs_to_f(op)?),

            "fld" => Fld(fl(op, memory_labels)?),
            "fsd" => Fsd(fs(op, memory_labels)?),
            "fmadd.d" => Fmaddd(r4(op)?),
            "fmsub.d" => Fmsubd(r4(op)?),
            "fnmsub.d" => Fnmsubd(r4(op)?),
            "fnmadd.d" => Fnmaddd(r4(op)?),
            "fadd.d" => Faddd(fr(op)?),
            "fsub.d" => Fsubd(fr(op)?),
            "fmul.d" => Fmuld(fr(op)?),
            "fdiv.d" => Fdivd(frs(op)?),
            "fsqrt.d" => Fsqrtd(fr(op)?),
            "fsgnj.d" => Fsgnjd(fr(op)?),
            "fsgnjn.d" => Fsgnjnd(fr(op)?),
            "fsgnjx.d" => Fsgnjxd(fr(op)?),
            "fmin.d" => Fmind(fr(op)?),
            "fmax.d" => Fmaxd(fr(op)?),
            "fcvt.s.d" => Fcvtsd(fr(op)?),
            "fcvt.d.s" => Fcvtds(fr(op)?),
            "feq.d" => Feqd(frx(op)?),
            "flt.d" => Fltd(frx(op)?),
            "fle.d" => Fled(frx(op)?),
            "fclass.d" => Fclassd(frs_to_x(op)?),
            "fcvt.w.d" => Fcvtwd(frs_to_x(op)?),
            "fcvt.wu.d" => Fcvtwud(frs_to_x(op)?),
            "fcvt.d.w" => Fcvtdw(frs_to_f(op)?),
            "fcvt.d.wu" => Fcvtdwu(frs_to_f(op)?),
            "fcvt.l.d" => Fcvtld(frs_to_x(op)?),
            "fcvt.lu.d" => Fcvtlud(frs_to_x(op)?),
            "fmv.x.d" => Fmvxd(frs_to_x(op)?),
            "fcvt.d.l" => Fcvtdl(frs_to_f(op)?),
            "fcvt.d.lu" => Fcvtdlu(frs_to_f(op)?),
            "fmv.d.x" => Fmvdx(frs_to_f(op)?),

            "vsetvli" => Vsetvli(vsetvli(op)?),
            "vsetivli" => Vsetivli(vsetivli(op)?),
            "vsetvl" => Vsetvl(vsetvl(op)?),

            "vle8.v" => Vlv {
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vle16.v" => Vlv {
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vle32.v" => Vlv {
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vle64.v" => Vlv {
                eew: BaseSew::E64,
                data: vl(op)?,
            },

            "vse8.v" => Vsv {
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vse16.v" => Vsv {
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vse32.v" => Vsv {
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vse64.v" => Vsv {
                eew: BaseSew::E64,
                data: vs(op)?,
            },

            "vlm.v" => Vlmv(vlm(op)?),
            "vsm.v" => Vsmv(vsm(op)?),

            "vlse8.v" => Vlsv {
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlse16.v" => Vlsv {
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlse32.v" => Vlsv {
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlse64.v" => Vlsv {
                eew: BaseSew::E64,
                data: vls(op)?,
            },

            "vsse8.v" => Vssv {
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vsse16.v" => Vssv {
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vsse32.v" => Vssv {
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vsse64.v" => Vssv {
                eew: BaseSew::E64,
                data: vss(op)?,
            },

            "vluxei8.v" => Vluxv {
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxei16.v" => Vluxv {
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxei32.v" => Vluxv {
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxei64.v" => Vluxv {
                eew: BaseSew::E64,
                data: vlx(op)?,
            },

            "vloxei8.v" => Vloxv {
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxei16.v" => Vloxv {
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxei32.v" => Vloxv {
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxei64.v" => Vloxv {
                eew: BaseSew::E64,
                data: vlx(op)?,
            },

            "vsuxei8.v" => Vsuxv {
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxei16.v" => Vsuxv {
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxei32.v" => Vsuxv {
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxei64.v" => Vsuxv {
                eew: BaseSew::E64,
                data: vsx(op)?,
            },

            "vsuxeix8.v" => Vsuxv {
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxeix16.v" => Vsuxv {
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxeix32.v" => Vsuxv {
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxeix64.v" => Vsuxv {
                eew: BaseSew::E64,
                data: vsx(op)?,
            },

            "vle8ff.v" => Vlffv {
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vle16ff.v" => Vlffv {
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vle32ff.v" => Vlffv {
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vle64ff.v" => Vlffv {
                eew: BaseSew::E64,
                data: vl(op)?,
            },

            // Note: I need to list all combinations so that I can research const-generification segmented load/stores in the future
            "vlseg1e8.v" => Vlsegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg1e16.v" => Vlsegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg1e32.v" => Vlsegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg1e64.v" => Vlsegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg2e8.v" => Vlsegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg2e16.v" => Vlsegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg2e32.v" => Vlsegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg2e64.v" => Vlsegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg3e8.v" => Vlsegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg3e16.v" => Vlsegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg3e32.v" => Vlsegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg3e64.v" => Vlsegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg4e8.v" => Vlsegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg4e16.v" => Vlsegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg4e32.v" => Vlsegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg4e64.v" => Vlsegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg5e8.v" => Vlsegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg5e16.v" => Vlsegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg5e32.v" => Vlsegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg5e64.v" => Vlsegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg6e8.v" => Vlsegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg6e16.v" => Vlsegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg6e32.v" => Vlsegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg6e64.v" => Vlsegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg7e8.v" => Vlsegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg7e16.v" => Vlsegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg7e32.v" => Vlsegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg7e64.v" => Vlsegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vl(op)?,
            },
            "vlseg8e8.v" => Vlsegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vl(op)?,
            },
            "vlseg8e16.v" => Vlsegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vl(op)?,
            },
            "vlseg8e32.v" => Vlsegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vl(op)?,
            },
            "vlseg8e64.v" => Vlsegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vl(op)?,
            },

            "vsseg1e8.v" => Vssegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg1e16.v" => Vssegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg1e32.v" => Vssegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg1e64.v" => Vssegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg2e8.v" => Vssegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg2e16.v" => Vssegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg2e32.v" => Vssegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg2e64.v" => Vssegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg3e8.v" => Vssegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg3e16.v" => Vssegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg3e32.v" => Vssegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg3e64.v" => Vssegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg4e8.v" => Vssegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg4e16.v" => Vssegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg4e32.v" => Vssegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg4e64.v" => Vssegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg5e8.v" => Vssegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg5e16.v" => Vssegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg5e32.v" => Vssegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg5e64.v" => Vssegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg6e8.v" => Vssegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg6e16.v" => Vssegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg6e32.v" => Vssegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg6e64.v" => Vssegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg7e8.v" => Vssegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg7e16.v" => Vssegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg7e32.v" => Vssegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg7e64.v" => Vssegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vs(op)?,
            },
            "vsseg8e8.v" => Vssegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vs(op)?,
            },
            "vsseg8e16.v" => Vssegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vs(op)?,
            },
            "vsseg8e32.v" => Vssegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vs(op)?,
            },
            "vsseg8e64.v" => Vssegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vs(op)?,
            },

            "vlsseg1e8.v" => Vlssegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg1e16.v" => Vlssegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg1e32.v" => Vlssegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg1e64.v" => Vlssegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg2e8.v" => Vlssegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg2e16.v" => Vlssegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg2e32.v" => Vlssegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg2e64.v" => Vlssegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg3e8.v" => Vlssegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg3e16.v" => Vlssegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg3e32.v" => Vlssegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg3e64.v" => Vlssegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg4e8.v" => Vlssegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg4e16.v" => Vlssegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg4e32.v" => Vlssegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg4e64.v" => Vlssegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg5e8.v" => Vlssegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg5e16.v" => Vlssegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg5e32.v" => Vlssegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg5e64.v" => Vlssegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg6e8.v" => Vlssegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg6e16.v" => Vlssegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg6e32.v" => Vlssegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg6e64.v" => Vlssegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg7e8.v" => Vlssegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg7e16.v" => Vlssegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg7e32.v" => Vlssegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg7e64.v" => Vlssegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vls(op)?,
            },
            "vlsseg8e8.v" => Vlssegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vls(op)?,
            },
            "vlsseg8e16.v" => Vlssegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vls(op)?,
            },
            "vlsseg8e32.v" => Vlssegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vls(op)?,
            },
            "vlsseg8e64.v" => Vlssegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vls(op)?,
            },

            "vssseg1e8.v" => Vsssegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg1e16.v" => Vsssegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg1e32.v" => Vsssegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg1e64.v" => Vsssegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg2e8.v" => Vsssegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg2e16.v" => Vsssegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg2e32.v" => Vsssegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg2e64.v" => Vsssegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg3e8.v" => Vsssegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg3e16.v" => Vsssegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg3e32.v" => Vsssegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg3e64.v" => Vsssegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg4e8.v" => Vsssegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg4e16.v" => Vsssegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg4e32.v" => Vsssegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg4e64.v" => Vsssegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg5e8.v" => Vsssegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg5e16.v" => Vsssegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg5e32.v" => Vsssegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg5e64.v" => Vsssegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg6e8.v" => Vsssegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg6e16.v" => Vsssegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg6e32.v" => Vsssegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg6e64.v" => Vsssegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg7e8.v" => Vsssegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg7e16.v" => Vsssegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg7e32.v" => Vsssegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg7e64.v" => Vsssegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vss(op)?,
            },
            "vssseg8e8.v" => Vsssegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vss(op)?,
            },
            "vssseg8e16.v" => Vsssegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vss(op)?,
            },
            "vssseg8e32.v" => Vsssegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vss(op)?,
            },
            "vssseg8e64.v" => Vsssegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vss(op)?,
            },

            "vluxseg1ei8.v" => Vluxsegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg1ei16.v" => Vluxsegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg1ei32.v" => Vluxsegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg1ei64.v" => Vluxsegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg2ei8.v" => Vluxsegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg2ei16.v" => Vluxsegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg2ei32.v" => Vluxsegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg2ei64.v" => Vluxsegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg3ei8.v" => Vluxsegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg3ei16.v" => Vluxsegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg3ei32.v" => Vluxsegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg3ei64.v" => Vluxsegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg4ei8.v" => Vluxsegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg4ei16.v" => Vluxsegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg4ei32.v" => Vluxsegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg4ei64.v" => Vluxsegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg5ei8.v" => Vluxsegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg5ei16.v" => Vluxsegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg5ei32.v" => Vluxsegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg5ei64.v" => Vluxsegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg6ei8.v" => Vluxsegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg6ei16.v" => Vluxsegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg6ei32.v" => Vluxsegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg6ei64.v" => Vluxsegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg7ei8.v" => Vluxsegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg7ei16.v" => Vluxsegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg7ei32.v" => Vluxsegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg7ei64.v" => Vluxsegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vluxseg8ei8.v" => Vluxsegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vluxseg8ei16.v" => Vluxsegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vluxseg8ei32.v" => Vluxsegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vluxseg8ei64.v" => Vluxsegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },

            "vloxseg1ei8.v" => Vloxsegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg1ei16.v" => Vloxsegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg1ei32.v" => Vloxsegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg1ei64.v" => Vloxsegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg2ei8.v" => Vloxsegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg2ei16.v" => Vloxsegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg2ei32.v" => Vloxsegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg2ei64.v" => Vloxsegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg3ei8.v" => Vloxsegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg3ei16.v" => Vloxsegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg3ei32.v" => Vloxsegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg3ei64.v" => Vloxsegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg4ei8.v" => Vloxsegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg4ei16.v" => Vloxsegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg4ei32.v" => Vloxsegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg4ei64.v" => Vloxsegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg5ei8.v" => Vloxsegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg5ei16.v" => Vloxsegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg5ei32.v" => Vloxsegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg5ei64.v" => Vloxsegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg6ei8.v" => Vloxsegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg6ei16.v" => Vloxsegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg6ei32.v" => Vloxsegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg6ei64.v" => Vloxsegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg7ei8.v" => Vloxsegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg7ei16.v" => Vloxsegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg7ei32.v" => Vloxsegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg7ei64.v" => Vloxsegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },
            "vloxseg8ei8.v" => Vloxsegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vlx(op)?,
            },
            "vloxseg8ei16.v" => Vloxsegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vlx(op)?,
            },
            "vloxseg8ei32.v" => Vloxsegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vlx(op)?,
            },
            "vloxseg8ei64.v" => Vloxsegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vlx(op)?,
            },

            "vsuxseg1ei8.v" => Vsuxsegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg1ei16.v" => Vsuxsegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg1ei32.v" => Vsuxsegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg1ei64.v" => Vsuxsegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg2ei8.v" => Vsuxsegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg2ei16.v" => Vsuxsegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg2ei32.v" => Vsuxsegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg2ei64.v" => Vsuxsegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg3ei8.v" => Vsuxsegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg3ei16.v" => Vsuxsegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg3ei32.v" => Vsuxsegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg3ei64.v" => Vsuxsegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg4ei8.v" => Vsuxsegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg4ei16.v" => Vsuxsegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg4ei32.v" => Vsuxsegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg4ei64.v" => Vsuxsegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg5ei8.v" => Vsuxsegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg5ei16.v" => Vsuxsegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg5ei32.v" => Vsuxsegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg5ei64.v" => Vsuxsegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg6ei8.v" => Vsuxsegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg6ei16.v" => Vsuxsegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg6ei32.v" => Vsuxsegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg6ei64.v" => Vsuxsegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg7ei8.v" => Vsuxsegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg7ei16.v" => Vsuxsegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg7ei32.v" => Vsuxsegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg7ei64.v" => Vsuxsegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsuxseg8ei8.v" => Vsuxsegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsuxseg8ei16.v" => Vsuxsegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsuxseg8ei32.v" => Vsuxsegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsuxseg8ei64.v" => Vsuxsegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },

            "vsoxseg1ei8.v" => Vsoxsegv {
                nf: 1,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg1ei16.v" => Vsoxsegv {
                nf: 1,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg1ei32.v" => Vsoxsegv {
                nf: 1,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg1ei64.v" => Vsoxsegv {
                nf: 1,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg2ei8.v" => Vsoxsegv {
                nf: 2,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg2ei16.v" => Vsoxsegv {
                nf: 2,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg2ei32.v" => Vsoxsegv {
                nf: 2,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg2ei64.v" => Vsoxsegv {
                nf: 2,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg3ei8.v" => Vsoxsegv {
                nf: 3,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg3ei16.v" => Vsoxsegv {
                nf: 3,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg3ei32.v" => Vsoxsegv {
                nf: 3,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg3ei64.v" => Vsoxsegv {
                nf: 3,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg4ei8.v" => Vsoxsegv {
                nf: 4,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg4ei16.v" => Vsoxsegv {
                nf: 4,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg4ei32.v" => Vsoxsegv {
                nf: 4,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg4ei64.v" => Vsoxsegv {
                nf: 4,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg5ei8.v" => Vsoxsegv {
                nf: 5,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg5ei16.v" => Vsoxsegv {
                nf: 5,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg5ei32.v" => Vsoxsegv {
                nf: 5,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg5ei64.v" => Vsoxsegv {
                nf: 5,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg6ei8.v" => Vsoxsegv {
                nf: 6,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg6ei16.v" => Vsoxsegv {
                nf: 6,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg6ei32.v" => Vsoxsegv {
                nf: 6,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg6ei64.v" => Vsoxsegv {
                nf: 6,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg7ei8.v" => Vsoxsegv {
                nf: 7,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg7ei16.v" => Vsoxsegv {
                nf: 7,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg7ei32.v" => Vsoxsegv {
                nf: 7,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg7ei64.v" => Vsoxsegv {
                nf: 7,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },
            "vsoxseg8ei8.v" => Vsoxsegv {
                nf: 8,
                eew: BaseSew::E8,
                data: vsx(op)?,
            },
            "vsoxseg8ei16.v" => Vsoxsegv {
                nf: 8,
                eew: BaseSew::E16,
                data: vsx(op)?,
            },
            "vsoxseg8ei32.v" => Vsoxsegv {
                nf: 8,
                eew: BaseSew::E32,
                data: vsx(op)?,
            },
            "vsoxseg8ei64.v" => Vsoxsegv {
                nf: 8,
                eew: BaseSew::E64,
                data: vsx(op)?,
            },

            "vl1re8.v" => Vlrv {
                nf: 1,
                eew: BaseSew::E8,
                data: vlr(op)?,
            },
            "vl1re16.v" => Vlrv {
                nf: 1,
                eew: BaseSew::E16,
                data: vlr(op)?,
            },
            "vl1re32.v" => Vlrv {
                nf: 1,
                eew: BaseSew::E32,
                data: vlr(op)?,
            },
            "vl1re64.v" => Vlrv {
                nf: 1,
                eew: BaseSew::E64,
                data: vlr(op)?,
            },
            "vl2re8.v" => Vlrv {
                nf: 2,
                eew: BaseSew::E8,
                data: vlr(op)?,
            },
            "vl2re16.v" => Vlrv {
                nf: 2,
                eew: BaseSew::E16,
                data: vlr(op)?,
            },
            "vl2re32.v" => Vlrv {
                nf: 2,
                eew: BaseSew::E32,
                data: vlr(op)?,
            },
            "vl2re64.v" => Vlrv {
                nf: 2,
                eew: BaseSew::E64,
                data: vlr(op)?,
            },
            "vl4re8.v" => Vlrv {
                nf: 4,
                eew: BaseSew::E8,
                data: vlr(op)?,
            },
            "vl4re16.v" => Vlrv {
                nf: 4,
                eew: BaseSew::E16,
                data: vlr(op)?,
            },
            "vl4re32.v" => Vlrv {
                nf: 4,
                eew: BaseSew::E32,
                data: vlr(op)?,
            },
            "vl4re64.v" => Vlrv {
                nf: 4,
                eew: BaseSew::E64,
                data: vlr(op)?,
            },
            "vl8re8.v" => Vlrv {
                nf: 8,
                eew: BaseSew::E8,
                data: vlr(op)?,
            },
            "vl8re16.v" => Vlrv {
                nf: 8,
                eew: BaseSew::E16,
                data: vlr(op)?,
            },
            "vl8re32.v" => Vlrv {
                nf: 8,
                eew: BaseSew::E32,
                data: vlr(op)?,
            },
            "vl8re64.v" => Vlrv {
                nf: 8,
                eew: BaseSew::E64,
                data: vlr(op)?,
            },

            "vs1r.v" => Vsrv {
                nf: 1,
                data: vsr(op)?,
            },
            "vs2r.v" => Vsrv {
                nf: 2,
                data: vsr(op)?,
            },
            "vs4r.v" => Vsrv {
                nf: 4,
                data: vsr(op)?,
            },
            "vs8r.v" => Vsrv {
                nf: 8,
                data: vsr(op)?,
            },

            "vadd.vv" => Vaddvv(opivv(op)?),
            "vadd.vx" => Vaddvx(opivx(op)?),
            "vadd.vi" => Vaddvi(opivi(op)?),

            "vsub.vv" => Vsubvv(opivv(op)?),
            "vsub.vx" => Vsubvx(opivx(op)?),

            "vrsub.vx" => Vrsubvx(opivx(op)?),
            "vrsub.vi" => Vrsubvi(opivi(op)?),

            "vminu.vv" => Vminuvv(opivv(op)?),
            "vminu.vx" => Vminuvx(opivx(op)?),

            "vmin.vv" => Vminvv(opivv(op)?),
            "vmin.vx" => Vminvx(opivx(op)?),

            "vmaxu.vv" => Vmaxuvv(opivv(op)?),
            "vmaxu.vx" => Vmaxuvx(opivx(op)?),

            "vmax.vv" => Vmaxvv(opivv(op)?),
            "vmax.vx" => Vmaxvx(opivx(op)?),

            "vand.vv" => Vandvv(opivv(op)?),
            "vand.vx" => Vandvx(opivx(op)?),
            "vand.vi" => Vandvi(opivi(op)?),

            "vor.vv" => Vorvv(opivv(op)?),
            "vor.vx" => Vorvx(opivx(op)?),
            "vor.vi" => Vorvi(opivi(op)?),

            "vxor.vv" => Vxorvv(opivv(op)?),
            "vxor.vx" => Vxorvx(opivx(op)?),
            "vxor.vi" => Vxorvi(opivi(op)?),

            "vrgather.vv" => Vrgathervv(opivv(op)?),
            "vrgather.vx" => Vrgathervx(opivx(op)?),
            "vrgather.vi" => Vrgathervi(opivi(op)?),

            "vrgatherei16.v" => Vrgatherei16vv(opivv(op)?),

            "vslideup.vx" => Vslideupvx(opivx(op)?),
            "vslideup.vi" => Vslideupvi(opivi(op)?),

            "vslidedown.vx" => Vslidedownvx(opivx(op)?),
            "vslidedown.vi" => Vslidedownvi(opivi(op)?),

            "vadc.vvm" => Vadcvvm(opivv_v0(op)?),
            "vadc.vxm" => Vadcvxm(opivx_v0(op)?),
            "vadc.vim" => Vadcvim(opivi_v0(op)?),

            "vmadc.vvm" => Vmadcvvm(opivv_v0(op)?),
            "vmadc.vxm" => Vmadcvxm(opivx_v0(op)?),
            "vmadc.vim" => Vmadcvim(opivi_v0(op)?),
            "vmadc.vv" => Vmadcvv(opivv_maskless(op)?),
            "vmadc.vx" => Vmadcvx(opivx_maskless(op)?),
            "vmadc.vi" => Vmadcvi(opivi_maskless(op)?),

            "vsbc.vvm" => Vsbcvvm(opivv(op)?),
            "vsbc.vxm" => Vsbcvxm(opivx(op)?),

            "vmsbc.vv" => Vmsbcvv(opivv(op)?),
            "vmsbc.vx" => Vmsbcvx(opivx(op)?),

            "vmerge.vvm" => Vmergevvm(opivv(op)?),
            "vmerge.vxm" => Vmergevxm(opivx(op)?),
            "vmerge.vim" => Vmergevim(opivi(op)?),

            "vmv.v.v" => Vmvvv(opivv_vmv(op)?),
            "vmv.v.x" => Vmvvx(opivx_vmv(op)?),
            "vmv.v.i" => Vmvvi(opivi_vmv(op)?),

            "vmseq.vv" => Vmseqvv(opivv(op)?),
            "vmseq.vx" => Vmseqvx(opivx(op)?),
            "vmseq.vi" => Vmseqvi(opivi(op)?),

            "vmsne.vv" => Vmsnevv(opivv(op)?),
            "vmsne.vx" => Vmsnevx(opivx(op)?),
            "vmsne.vi" => Vmsnevi(opivi(op)?),

            "vmsltu.vv" => Vmsltuvv(opivv(op)?),
            "vmsltu.vx" => Vmsltuvx(opivx(op)?),

            "vmslt.vv" => Vmsltvv(opivv(op)?),
            "vmslt.vx" => Vmsltvx(opivx(op)?),

            "vmsleu.vv" => Vmsleuvv(opivv(op)?),
            "vmsleu.vx" => Vmsleuvx(opivx(op)?),
            "vmsleu.vi" => Vmsleuvi(opivi(op)?),

            "vmsle.vv" => Vmslevv(opivv(op)?),
            "vmsle.vx" => Vmslevx(opivx(op)?),
            "vmsle.vi" => Vmslevi(opivi(op)?),

            "vmsgtu.vx" => Vmsgtuvx(opivx(op)?),
            "vmsgtu.vi" => Vmsgtuvi(opivi(op)?),

            "vmsgt.vx" => Vmsgtvx(opivx(op)?),
            "vmsgt.vi" => Vmsgtvi(opivi(op)?),

            "vsaddu.vv" => Vsadduvv(opivv(op)?),
            "vsaddu.vx" => Vsadduvx(opivx(op)?),
            "vsaddu.vi" => Vsadduvi(opivi(op)?),

            "vsadd.vv" => Vsaddvv(opivv(op)?),
            "vsadd.vx" => Vsaddvx(opivx(op)?),
            "vsadd.vi" => Vsaddvi(opivi(op)?),

            "vssubu.vv" => Vssubuvv(opivv(op)?),
            "vssubu.vx" => Vssubuvx(opivx(op)?),

            "vssub.vv" => Vssubvv(opivv(op)?),
            "vssub.vx" => Vssubvx(opivx(op)?),

            "vsll.vv" => Vsllvv(opivv(op)?),
            "vsll.vx" => Vsllvx(opivx(op)?),
            "vsll.vi" => Vsllvi(opivi(op)?),

            "vsmul.vv" => Vsmulvv(opivv(op)?),
            "vsmul.vx" => Vsmulvx(opivx(op)?),

            "vmv1r.v" => Vmv1rv(opivi(op)?),
            "vmv2r.v" => Vmv2rv(opivi(op)?),
            "vmv4r.v" => Vmv4rv(opivi(op)?),
            "vmv8r.v" => Vmv8rv(opivi(op)?),

            "vsrl.vv" => Vsrlvv(opivv(op)?),
            "vsrl.vx" => Vsrlvx(opivx(op)?),
            "vsrl.vi" => Vsrlvi(opivi(op)?),

            "vsra.vv" => Vsravv(opivv(op)?),
            "vsra.vx" => Vsravx(opivx(op)?),
            "vsra.vi" => Vsravi(opivi(op)?),

            "vssrl.vv" => Vssrlvv(opivv(op)?),
            "vssrl.vx" => Vssrlvx(opivx(op)?),
            "vssrl.vi" => Vssrlvi(opivi(op)?),

            "vssra.vv" => Vssravv(opivv(op)?),
            "vssra.vx" => Vssravx(opivx(op)?),
            "vssra.vi" => Vssravi(opivi(op)?),

            "vnsrl.wv" => Vnsrlwv(opivv(op)?),
            "vnsrl.wx" => Vnsrlwx(opivx(op)?),
            "vnsrl.wi" => Vnsrlwi(opivi(op)?),

            "vnsra.wv" => Vnsrawv(opivv(op)?),
            "vnsra.wx" => Vnsrawx(opivx(op)?),
            "vnsra.wi" => Vnsrawi(opivi(op)?),

            "vnclipu.wv" => Vnclipuwv(opivv(op)?),
            "vnclipu.wx" => Vnclipuwx(opivx(op)?),
            "vnclipu.wi" => Vnclipuwi(opivi(op)?),

            "vnclip.wv" => Vnclipwv(opivv(op)?),
            "vnclip.wx" => Vnclipwx(opivx(op)?),
            "vnclip.wi" => Vnclipwi(opivi(op)?),

            "vwredsumu.vs" => Vwredsumuvs(opivv(op)?),
            "vwredsum.vs" => Vwredsumvs(opivv(op)?),

            "vredsum.vs" => Vredsumvs(opmvv(op)?),
            "vredand.vs" => Vredandvs(opmvv(op)?),
            "vredor.vs" => Vredorvs(opmvv(op)?),
            "vredxor.vs" => Vredxorvs(opmvv(op)?),
            "vredminu.vs" => Vredminuvs(opmvv(op)?),
            "vredmin.vs" => Vredminvs(opmvv(op)?),
            "vredmaxu.vs" => Vredmaxuvs(opmvv(op)?),
            "vredmax.vs" => Vredmaxvs(opmvv(op)?),

            "vaaddu.vv" => Vaadduvv(opmvv(op)?),
            "vaaddu.vx" => Vaadduvx(opmvx(op)?),

            "vaadd.vv" => Vaaddvv(opmvv(op)?),
            "vaadd.vx" => Vaaddvx(opmvx(op)?),

            "vasubu.vv" => Vasubuvv(opmvv(op)?),
            "vasubu.vx" => Vasubuvx(opmvx(op)?),

            "vasub.vv" => Vasubvv(opmvv(op)?),
            "vasub.vx" => Vasubvx(opmvx(op)?),

            "vslide1up.vx" => Vslide1upvx(opmvx(op)?),

            "vslide1down.vx" => Vslide1downvx(opmvx(op)?),

            "vmv.x.s" => Vmvxs(vmvxs(op)?),
            "vcpop.m" => Vcpopm(vwxunary0(op)?),
            "vfirst.m" => Vfirstm(vwxunary0(op)?),

            "vmv.s.x" => Vmvsx(vrxunary0(op)?),

            "vsext.vf2" => Vsextvf2(vxunary0(op)?),
            "vsext.vf4" => Vsextvf4(vxunary0(op)?),
            "vsext.vf8" => Vsextvf8(vxunary0(op)?),

            "vzext.vf2" => Vzextvf2(vxunary0(op)?),
            "vzext.vf4" => Vzextvf4(vxunary0(op)?),
            "vzext.vf8" => Vzextvf8(vxunary0(op)?),

            "vmsbf.m" => Vmsbfm(vmunary0(op)?),
            "vmsof.m" => Vmsofm(vmunary0(op)?),
            "vmsif.m" => Vmsifm(vmunary0(op)?),
            "viota.m" => Viotam(vmunary0(op)?),
            "vid.v" => Vidv(vidv(op)?),

            "vcompress.vm" => Vcompressvm(opmvv_maskless(op)?),

            "vmandn.mm" => Vmandnmm(opmvv_maskless(op)?),
            "vmand.mm" => Vmandmm(opmvv_maskless(op)?),
            "vmor.mm" => Vmormm(opmvv_maskless(op)?),
            "vmxor.mm" => Vmxormm(opmvv_maskless(op)?),
            "vmorn.mm" => Vmornmm(opmvv_maskless(op)?),
            "vmnand.mm" => Vmnandmm(opmvv_maskless(op)?),
            "vmnor.mm" => Vmnormm(opmvv_maskless(op)?),
            "vmxnor.mm" => Vmxnormm(opmvv_maskless(op)?),

            "vdivu.vv" => Vdivuvv(opmvv(op)?),
            "vdivu.vx" => Vdivuvx(opmvx(op)?),

            "vdiv.vv" => Vdivvv(opmvv(op)?),
            "vdiv.vx" => Vdivvx(opmvx(op)?),

            "vremu.vv" => Vremuvv(opmvv(op)?),
            "vremu.vx" => Vremuvx(opmvx(op)?),

            "vrem.vv" => Vremvv(opmvv(op)?),
            "vrem.vx" => Vremvx(opmvx(op)?),

            "vmulhu.vv" => Vmulhuvv(opmvv(op)?),
            "vmulhu.vx" => Vmulhuvx(opmvx(op)?),

            "vmul.vv" => Vmulvv(opmvv(op)?),
            "vmul.vx" => Vmulvx(opmvx(op)?),

            "vmulhsu.vv" => Vmulhsuvv(opmvv(op)?),
            "vmulhsu.vx" => Vmulhsuvx(opmvx(op)?),

            "vmulh.vv" => Vmulhvv(opmvv(op)?),
            "vmulh.vx" => Vmulhvx(opmvx(op)?),

            "vmadd.vv" => Vmaddvv(opmvv_fma(op)?),
            "vmadd.vx" => Vmaddvx(opmvx_fma(op)?),

            "vnmsub.vv" => Vnmsubvv(opmvv_fma(op)?),
            "vnmsub.vx" => Vnmsubvx(opmvx_fma(op)?),

            "vmacc.vv" => Vmaccvv(opmvv_fma(op)?),
            "vmacc.vx" => Vmaccvx(opmvx_fma(op)?),

            "vnmsac.vv" => Vnmsacvv(opmvv_fma(op)?),
            "vnmsac.vx" => Vnmsacvx(opmvx_fma(op)?),

            "vwaddu.vv" => Vwadduvv(opmvv(op)?),
            "vwaddu.vx" => Vwadduvx(opmvx(op)?),

            "vwadd.vv" => Vwaddvv(opmvv(op)?),
            "vwadd.vx" => Vwaddvx(opmvx(op)?),

            "vwsubu.vv" => Vwsubuvv(opmvv(op)?),
            "vwsubu.vx" => Vwsubuvx(opmvx(op)?),

            "vwsub.vv" => Vwsubvv(opmvv(op)?),
            "vwsub.vx" => Vwsubvx(opmvx(op)?),

            "vwaddu.wv" => Vwadduwv(opmvv(op)?),
            "vwaddu.wx" => Vwadduwx(opmvx(op)?),

            "vwadd.wv" => Vwaddwv(opmvv(op)?),
            "vwadd.wx" => Vwaddwx(opmvx(op)?),

            "vwsubu.wv" => Vwsubuwv(opmvv(op)?),
            "vwsubu.wx" => Vwsubuwx(opmvx(op)?),

            "vwsub.wv" => Vwsubwv(opmvv(op)?),
            "vwsub.wx" => Vwsubwx(opmvx(op)?),

            "vwmulu.vv" => Vwmuluvv(opmvv(op)?),
            "vwmulu.vx" => Vwmuluvx(opmvx(op)?),

            "vwmulsu.vv" => Vwmulsuvv(opmvv(op)?),
            "vwmulsu.vx" => Vwmulsuvx(opmvx(op)?),

            "vwmul.vv" => Vwmulvv(opmvv(op)?),
            "vwmul.vx" => Vwmulvx(opmvx(op)?),

            "vwmaccu.vv" => Vwmaccuvv(opmvv_fma(op)?),
            "vwmaccu.vx" => Vwmaccuvx(opmvx_fma(op)?),

            "vwmacc.vv" => Vwmaccvv(opmvv_fma(op)?),
            "vwmacc.vx" => Vwmaccvx(opmvx_fma(op)?),

            "vwmaccus.vx" => Vwmaccusvx(opmvx_fma(op)?),

            "vwmaccsu.vv" => Vwmaccsuvv(opmvv_fma(op)?),
            "vwmaccsu.vx" => Vwmaccsuvx(opmvx_fma(op)?),

            "vfadd.vv" => Vfaddvv(opfvv(op)?),
            "vfadd.vf" => Vfaddvf(opfvf(op)?),

            "vfredusum.vs" => Vfredusumvs(opfvv(op)?),

            "vfsub.vv" => Vfsubvv(opfvv(op)?),
            "vfsub.vf" => Vfsubvf(opfvf(op)?),

            "vfredosum.vs" => Vfredosumvs(opfvv(op)?),

            "vfmin.vv" => Vfminvv(opfvv(op)?),
            "vfmin.vf" => Vfminvf(opfvf(op)?),

            "vfredmin.vs" => Vfredminvs(opfvv(op)?),

            "vfmax.vv" => Vfmaxvv(opfvv(op)?),
            "vfmax.vf" => Vfmaxvf(opfvf(op)?),

            "vfredmax.vs" => Vfredmaxvs(opfvv(op)?),

            "vfsgnj.vv" => Vfsgnjvv(opfvv(op)?),
            "vfsgnj.vf" => Vfsgnjvf(opfvf(op)?),

            "vfsgnjn.vv" => Vfsgnjnvv(opfvv(op)?),
            "vfsgnjn.vf" => Vfsgnjnvf(opfvf(op)?),

            "vfsgnjx.vv" => Vfsgnjxvv(opfvv(op)?),
            "vfsgnjx.vf" => Vfsgnjxvf(opfvf(op)?),

            "vfslide1up.vf" => Vfslide1upvf(opfvf(op)?),

            "vfslide1down.vf" => Vfslide1downvf(opfvf(op)?),

            "vfmv.f.s" => Vfmvfs(vwfunary0(op)?),

            "vfmv.s.f" => Vfmvsf(vrfunary0(op)?),

            "vfcvt.xu.f.v" => Vfcvtxufv(vfunary0(op)?),
            "vfcvt.x.f.v" => Vfcvtxfv(vfunary0(op)?),
            "vfcvt.f.xu.v" => Vfcvtfxuv(vfunary0(op)?),
            "vfcvt.f.x.v" => Vfcvtfxv(vfunary0(op)?),
            "vfcvt.rtz.xu.f.v" => VfcvtRtzxufv(vfunary0(op)?),
            "vfcvt.rtz.x.f.v" => VfcvtRtzxfv(vfunary0(op)?),

            "vfwcvt.xu.f.v" => Vfwcvtxufv(vfunary0(op)?),
            "vfwcvt.x.f.v" => Vfwcvtxfv(vfunary0(op)?),
            "vfwcvt.f.xu.v" => Vfwcvtfxuv(vfunary0(op)?),
            "vfwcvt.f.x.v" => Vfwcvtfxv(vfunary0(op)?),
            "vfwcvt.f.f.v" => Vfwcvtffv(vfunary0(op)?),
            "vfwcvt.rtz.xu.f.v" => VfwcvtRtzxufv(vfunary0(op)?),
            "vfwcvt.rtz.x.f.v" => VfwcvtRtzxfv(vfunary0(op)?),

            "vfncvt.xu.f.w" => Vfncvtxufw(vfunary0(op)?),
            "vfncvt.x.f.w" => Vfncvtxfw(vfunary0(op)?),
            "vfncvt.f.xu.w" => Vfncvtfxuw(vfunary0(op)?),
            "vfncvt.f.x.w" => Vfncvtfxw(vfunary0(op)?),
            "vfncvt.f.f.w" => Vfncvtffw(vfunary0(op)?),
            "vfncvt.rod.f.f.w" => VfncvtRodffw(vfunary0(op)?),
            "vfncvt.rtz.xu.f.w" => VfncvtRtzxufw(vfunary0(op)?),
            "vfncvt.rtz.x.f.w" => VfncvtRtzxfw(vfunary0(op)?),

            "vfsqrt.v" => Vfsqrtv(vfunary1(op)?),
            "vfrsqrt7.v" => Vfrsqrt7v(vfunary1(op)?),
            "vfrec7.v" => Vfrec7v(vfunary1(op)?),
            "vfclass.v" => Vfclassv(vfunary1(op)?),

            "vfmerge.vfm" => Vfmergevfm(opfvf(op)?),
            "vfmv.v.f" => Vfmvvf(opfvf(op)?),

            "vmfeq.vv" => Vmfeqvv(opfvv(op)?),
            "vmfeq.vf" => Vmfeqvf(opfvf(op)?),

            "vmfle.vv" => Vmflevv(opfvv(op)?),
            "vmfle.vf" => Vmflevf(opfvf(op)?),

            "vmflt.vv" => Vmfltvv(opfvv(op)?),
            "vmflt.vf" => Vmfltvf(opfvf(op)?),

            "vmfne.vv" => Vmfnevv(opfvv(op)?),
            "vmfne.vf" => Vmfnevf(opfvf(op)?),

            "vmfgt.vf" => Vmfgtvf(opfvf(op)?),

            "vmfge.vf" => Vmfgevf(opfvf(op)?),

            "vfdiv.vv" => Vfdivvv(opfvv(op)?),
            "vfdiv.vf" => Vfdivvf(opfvf(op)?),

            "vfrdiv.vf" => Vfrdivvf(opfvf(op)?),

            "vfmul.vv" => Vfmulvv(opfvv(op)?),
            "vfmul.vf" => Vfmulvf(opfvf(op)?),

            "vfrsub.vf" => Vfrsubvf(opfvf(op)?),

            "vfmadd.vv" => Vfmaddvv(opfvv_fma(op)?),
            "vfmadd.vf" => Vfmaddvf(opfvf_fma(op)?),

            "vfnmadd.vv" => Vfnmaddvv(opfvv_fma(op)?),
            "vfnmadd.vf" => Vfnmaddvf(opfvf_fma(op)?),

            "vfmsub.vv" => Vfmsubvv(opfvv_fma(op)?),
            "vfmsub.vf" => Vfmsubvf(opfvf_fma(op)?),

            "vfnmsub.vv" => Vfnmsubvv(opfvv_fma(op)?),
            "vfnmsub.vf" => Vfnmsubvf(opfvf_fma(op)?),

            "vfmacc.vv" => Vfmaccvv(opfvv_fma(op)?),
            "vfmacc.vf" => Vfmaccvf(opfvf_fma(op)?),

            "vfnmacc.vv" => Vfnmaccvv(opfvv_fma(op)?),
            "vfnmacc.vf" => Vfnmaccvf(opfvf_fma(op)?),

            "vfmsac.vv" => Vfmsacvv(opfvv_fma(op)?),
            "vfmsac.vf" => Vfmsacvf(opfvf_fma(op)?),

            "vfnmsac.vv" => Vfnmsacvv(opfvv_fma(op)?),
            "vfnmsac.vf" => Vfnmsacvf(opfvf_fma(op)?),

            "vfwadd.vv" => Vfwaddvv(opfvv(op)?),
            "vfwadd.vf" => Vfwaddvf(opfvf(op)?),

            "vfwredusum.vs" => Vfwredusumvs(opfvv(op)?),

            "vfwsub.vv" => Vfwsubvv(opfvv(op)?),
            "vfwsub.vf" => Vfwsubvf(opfvf(op)?),

            "vfwredosum.vs" => Vfwredosumvs(opfvv(op)?),

            "vfwadd.wv" => Vfwaddwv(opfvv(op)?),
            "vfwadd.wf" => Vfwaddwf(opfvf(op)?),

            "vfwsub.wv" => Vfwsubwv(opfvv(op)?),
            "vfwsub.wf" => Vfwsubwf(opfvf(op)?),

            "vfwmul.vv" => Vfwmulvv(opfvv(op)?),
            "vfwmul.vf" => Vfwmulvf(opfvf(op)?),

            "vfwmacc.vv" => Vfwmaccvv(opfvv_fma(op)?),
            "vfwmacc.vf" => Vfwmaccvf(opfvf_fma(op)?),

            "vfwnmacc.vv" => Vfwnmaccvv(opfvv_fma(op)?),
            "vfwnmacc.vf" => Vfwnmaccvf(opfvf_fma(op)?),

            "vfwmsac.vv" => Vfwmsacvv(opfvv_fma(op)?),
            "vfwmsac.vf" => Vfwmsacvf(opfvf_fma(op)?),

            "vfwnmsac.vv" => Vfwnmsacvv(opfvv_fma(op)?),
            "vfwnmsac.vf" => Vfwnmsacvf(opfvf_fma(op)?),

            // Pseudoinstructions
            "la" => {
                let (rd, mem_addr) =
                    integer::pseudo::parse_op_memory_label_format(op, memory_labels)?;

                fuse![
                    Auipc(U {
                        rd,
                        imm20: mem_addr >> 12,
                    }),
                    Addi(I {
                        rd,
                        rs1: rd,
                        imm12: mem_addr & 0xfff,
                    })
                ]
            }
            "nop" => Addi(I {
                rd: 0,
                rs1: 0,
                imm12: 0,
            }),
            "li" => {
                let (reg, imm) = integer::pseudo::parse_op_imm_format(op)?;

                match imm.cmp(&4096) {
                    Ordering::Less => Addi(I {
                        rd: reg,
                        rs1: 0,
                        imm12: imm,
                    }),
                    Ordering::Equal => Lui(U {
                        rd: reg,
                        imm20: imm,
                    }),
                    Ordering::Greater => fuse![
                        Lui(U {
                            rd: reg,
                            imm20: imm >> 12,
                        }),
                        Addi(I {
                            rd: reg,
                            rs1: reg,
                            imm12: imm & 0xfff,
                        }),
                    ],
                }
            }
            "mv" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Addi(I { rd, rs1, imm12: 0 })
            }
            "not" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Xori(I { rd, rs1, imm12: -1 })
            }
            "neg" => {
                let (rd, rs2) = integer::pseudo::parse_op_op_format(op)?;
                Sub(R { rd, rs1: 0, rs2 })
            }
            "negw" => {
                let (rd, rs2) = integer::pseudo::parse_op_op_format(op)?;
                Subw(R { rd, rs1: 0, rs2 })
            }
            "sext.b" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;

                fuse![
                    Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 8,
                    }),
                    Srai(I {
                        rd,
                        rs1,
                        imm12: 64 - 8,
                    }),
                ]
            }
            "sext.h" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                fuse![
                    Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    }),
                    Srai(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    }),
                ]
            }
            "sext.w" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Addiw(I { rd, rs1, imm12: 0 })
            }
            "zext.b" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Andi(I {
                    rd,
                    rs1,
                    imm12: 0xff,
                })
            }
            "zext.h" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;

                fuse![
                    Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    }),
                    Srli(I {
                        rd,
                        rs1: rd,
                        imm12: 64 - 16,
                    }),
                ]
            }
            "zext.w" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;

                fuse![
                    Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 32,
                    }),
                    Srli(I {
                        rd,
                        rs1: rd,
                        imm12: 64 - 32,
                    }),
                ]
            }
            "seqz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Sltiu(I { rd, rs1, imm12: 1 })
            }
            "snez" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Sltu(R { rd, rs1, rs2: 0 })
            }
            "sltz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Slt(R { rd, rs1, rs2: 0 })
            }
            "sgtz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(op)?;
                Slt(R {
                    rd,
                    rs1: 0,
                    rs2: rs1,
                })
            }
            "fmv.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjs(R { rd, rs1, rs2: rs1 })
            }
            "fabs.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjxs(R { rd, rs1, rs2: rs1 })
            }
            "fneg.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjns(R { rd, rs1, rs2: rs1 })
            }
            "fmv.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjd(R { rd, rs1, rs2: rs1 })
            }
            "fabs.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjxd(R { rd, rs1, rs2: rs1 })
            }
            "fneg.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(op)?;
                Fsgnjnd(R { rd, rs1, rs2: rs1 })
            }
            "beqz" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Beq(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bnez" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bne(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "blez" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bge(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgez" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bge(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bltz" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Blt(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgtz" => {
                let (rs1, diff) = integer::pseudo::parse_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Blt(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgt" => {
                let (rs1, rs2, diff) = integer::pseudo::parse_op_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Blt(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "ble" => {
                let (rs1, rs2, diff) = integer::pseudo::parse_op_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bge(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "bgtu" => {
                let (rs1, rs2, diff) = integer::pseudo::parse_op_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bltu(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "bleu" => {
                let (rs1, rs2, diff) = integer::pseudo::parse_op_op_label_format(
                    op,
                    instruction_labels,
                    current_address,
                )?;
                Bgeu(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "j" => {
                let diff =
                    integer::pseudo::parse_label_format(op, instruction_labels, current_address)?;
                Jal(U { rd: 0, imm20: diff })
            }
            "jr" => {
                let rs1 = integer::pseudo::parse_op_format(op)?;
                Jalr(I {
                    rd: 0,
                    rs1,
                    imm12: 0,
                })
            }
            "ret" => Jalr(I {
                rd: 0,
                rs1: 1,
                imm12: 0,
            }),
            "call" => {
                let diff =
                    integer::pseudo::parse_label_format(op, instruction_labels, current_address)?;

                fuse![
                    Auipc(U {
                        rd: 1,
                        imm20: diff >> 12,
                    }),
                    Jalr(I {
                        rd: 1,
                        rs1: 1,
                        imm12: diff & 0xfff,
                    }),
                ]
            }
            "tail" => {
                let diff =
                    integer::pseudo::parse_label_format(op, instruction_labels, current_address)?;

                fuse![
                    Auipc(U {
                        rd: 6,
                        imm20: diff >> 12,
                    }),
                    Jalr(I {
                        rd: 0,
                        rs1: 6,
                        imm12: diff & 0xfff,
                    }),
                ]
            }
            "rdinstret" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::INSTRET,
                    rs1: 0,
                })
            }
            "rdcycle" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::CYCLE,
                    rs1: 0,
                })
            }
            "rdtime" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::TIME,
                    rs1: 0,
                })
            }
            "csrr" => {
                let (rd, csr) = csr::pseudo::parse_op_csr_format(op)?;
                Csrrs(Csrr { rd, csr, rs1: 0 })
            }
            "csrw" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(op)?;
                Csrrw(Csrr { rd: 0, csr, rs1 })
            }
            "csrs" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(op)?;
                Csrrs(Csrr { rd: 0, csr, rs1 })
            }
            "csrc" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(op)?;
                Csrrc(Csrr { rd: 0, csr, rs1 })
            }
            "frcsr" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FCSR,
                    rs1: 0,
                })
            }
            "fscsr" => match integer::pseudo::parse_op_op_format(op) {
                Ok((rd, rs1)) => Csrrw(Csrr {
                    rd,
                    csr: alias::FCSR,
                    rs1,
                }),
                Err(fst_err) => match integer::pseudo::parse_op_format(op) {
                    Ok(rs) => Csrrw(Csrr {
                        rd: 0,
                        csr: alias::FCSR,
                        rs1: rs,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },
            "frrm" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FRM,
                    rs1: 0,
                })
            }
            "fsrm" => match integer::pseudo::parse_op_op_format(op) {
                Ok((rd, rs1)) => Csrrw(Csrr {
                    rd,
                    csr: alias::FRM,
                    rs1,
                }),
                Err(fst_err) => match integer::pseudo::parse_op_format(op) {
                    Ok(rs) => Csrrw(Csrr {
                        rd: 0,
                        csr: alias::FRM,
                        rs1: rs,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },
            "fsrmi" => match integer::pseudo::parse_op_imm_format(op) {
                Ok((rd, imm)) => Csrrwi(Csri {
                    rd,
                    csr: alias::FRM,
                    uimm: imm as u32 as usize,
                }),
                Err(fst_err) => match integer::pseudo::parse_imm_format(op) {
                    Ok(imm) => Csrrwi(Csri {
                        rd: 0,
                        csr: alias::FRM,
                        uimm: imm as u32 as usize,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },
            "frflags" => {
                let rd = integer::pseudo::parse_op_format(op)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FFLAGS,
                    rs1: 0,
                })
            }
            "fsflags" => match integer::pseudo::parse_op_op_format(op) {
                Ok((rd, rs1)) => Csrrw(Csrr {
                    rd,
                    csr: alias::FFLAGS,
                    rs1,
                }),
                Err(fst_err) => match integer::pseudo::parse_op_format(op) {
                    Ok(rs1) => Csrrw(Csrr {
                        rd: 0,
                        csr: alias::FFLAGS,
                        rs1,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },
            "fsflagsi" => match integer::pseudo::parse_op_imm_format(op) {
                Ok((rd, imm)) => Csrrwi(Csri {
                    rd,
                    csr: alias::FFLAGS,
                    uimm: imm as u32 as usize,
                }),
                Err(fst_err) => match integer::pseudo::parse_imm_format(op) {
                    Ok(imm) => Csrrwi(Csri {
                        rd: 0,
                        csr: alias::FFLAGS,
                        uimm: imm as u32 as usize,
                    }),
                    Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err)),
                },
            },

            "vneg.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vrsubvx(Opivx {
                    vd,
                    rs1: alias::ZERO,
                    vs2,
                    vm,
                })
            }
            "vwcvt.x.x.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vwaddvx(Opmvx {
                    dest: vd,
                    rs1: alias::ZERO,
                    vs2,
                    vm,
                })
            }
            "vwcvtu.x.x.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vwadduvx(Opmvx {
                    dest: vd,
                    rs1: alias::ZERO,
                    vs2,
                    vm,
                })
            }
            "vnot.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vxorvi(Opivi {
                    vd,
                    imm5: -1,
                    vs2,
                    vm,
                })
            }
            "vncvt.x.x.w" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vnsrlwx(Opivx {
                    vd,
                    rs1: alias::ZERO,
                    vs2,
                    vm,
                })
            }
            "vmsgt.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmsltvv(Opivv {
                    vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmsgtu.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmsltuvv(Opivv {
                    vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmsge.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmslevv(Opivv {
                    vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmsgeu.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmsleuvv(Opivv {
                    vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmslt.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(op)?;
                Vmslevi(Opivi {
                    vd,
                    imm5: imm - 1,
                    vs2,
                    vm,
                })
            }
            "vmsltu.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(op)?;
                Vmsleuvi(Opivi {
                    vd,
                    imm5: imm - 1,
                    vs2,
                    vm,
                })
            }
            "vmsge.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(op)?;
                Vmsgtvi(Opivi {
                    vd,
                    imm5: imm - 1,
                    vs2,
                    vm,
                })
            }
            "vmsgeu.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(op)?;
                Vmsgtuvi(Opivi {
                    vd,
                    imm5: imm - 1,
                    vs2,
                    vm,
                })
            }
            "vmsge.vx" => match vector::pseudo::parse_op_op_xreg_format(op) {
                Ok((vd, vs2, rs1)) => fuse![
                    Vmsltvx(Opivx {
                        vd,
                        rs1,
                        vs2,
                        vm: false,
                    }),
                    Vmnandmm(Opmvv {
                        dest: vd,
                        vs1: vd,
                        vs2: vd,
                        vm: false,
                    }),
                ],
                Err(fst_err) => match vector::pseudo::parse_op_op_xreg_mask_vd_nonzero_format(op) {
                    Ok((vd, vs2, rs1)) => fuse![
                        Vmsltvx(Opivx {
                            vd,
                            rs1,
                            vs2,
                            vm: true,
                        }),
                        Vmxormm(Opmvv {
                            dest: vd,
                            vs1: 0,
                            vs2: vd,
                            vm: false,
                        }),
                    ],
                    Err(snd_err) => match vector::pseudo::parse_op_op_xreg_mask_temp_format(op) {
                        Ok((vd, vs2, rs1, vt)) => {
                            if vd == 0 {
                                fuse![
                                    Vmsltvx(Opivx {
                                        vd: vt,
                                        rs1,
                                        vs2,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vd,
                                        vs1: vt,
                                        vs2: vd,
                                        vm: false,
                                    }),
                                ]
                            } else {
                                fuse![
                                    Vmsltvx(Opivx {
                                        vd: vt,
                                        rs1,
                                        vs2,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vt,
                                        vs1: vt,
                                        vs2: 0,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vd,
                                        vs1: 0,
                                        vs2: vd,
                                        vm: false,
                                    }),
                                    Vmormm(Opmvv {
                                        dest: vd,
                                        vs1: vd,
                                        vs2: vt,
                                        vm: false,
                                    }),
                                ]
                            }
                        }
                        Err(trd_err) => {
                            return Err(format!("{}, {} or {}", fst_err, snd_err, trd_err))
                        }
                    },
                },
            },
            "vmsgeu.vx" => match vector::pseudo::parse_op_op_xreg_format(op) {
                Ok((vd, vs2, rs1)) => fuse![
                    Vmsltuvx(Opivx {
                        vd,
                        rs1,
                        vs2,
                        vm: false,
                    }),
                    Vmnandmm(Opmvv {
                        dest: vd,
                        vs1: vd,
                        vs2: vd,
                        vm: false,
                    })
                ],
                Err(fst_err) => match vector::pseudo::parse_op_op_xreg_mask_vd_nonzero_format(op) {
                    Ok((vd, vs2, rs1)) => fuse![
                        Vmsltuvx(Opivx {
                            vd,
                            rs1,
                            vs2,
                            vm: true,
                        }),
                        Vmxormm(Opmvv {
                            dest: vd,
                            vs1: 0,
                            vs2: vd,
                            vm: false,
                        })
                    ],
                    Err(snd_err) => match vector::pseudo::parse_op_op_xreg_mask_temp_format(op) {
                        Ok((vd, vs2, rs1, vt)) => {
                            if vd == 0 {
                                fuse![
                                    Vmsltuvx(Opivx {
                                        vd: vt,
                                        rs1,
                                        vs2,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vd,
                                        vs1: vt,
                                        vs2: vd,
                                        vm: false,
                                    }),
                                ]
                            } else {
                                fuse![
                                    Vmsltuvx(Opivx {
                                        vd: vt,
                                        rs1,
                                        vs2,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vt,
                                        vs1: vt,
                                        vs2: 0,
                                        vm: false,
                                    }),
                                    Vmandnmm(Opmvv {
                                        dest: vd,
                                        vs1: 0,
                                        vs2: vd,
                                        vm: false,
                                    }),
                                    Vmormm(Opmvv {
                                        dest: vd,
                                        vs1: vd,
                                        vs2: vt,
                                        vm: false,
                                    })
                                ]
                            }
                        }
                        Err(trd_err) => {
                            return Err(format!("{}, {} or {}", fst_err, snd_err, trd_err))
                        }
                    },
                },
            },
            "vfneg.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vfsgnjnvv(Opfvv {
                    dest: vd,
                    vs1: vs2,
                    vs2,
                    vm,
                })
            }
            "vfabs.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(op)?;
                Vfsgnjxvv(Opfvv {
                    dest: vd,
                    vs1: vs2,
                    vs2,
                    vm,
                })
            }
            "vmfgt.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmfltvv(Opfvv {
                    dest: vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmfge.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(op)?;
                Vmflevv(Opfvv {
                    dest: vd,
                    vs1: vs2,
                    vs2: vs1,
                    vm,
                })
            }
            "vmmv.m" => {
                let (vd, vs2) = vector::pseudo::parse_op_op_format(op)?;
                Vmandmm(Opmvv {
                    dest: vd,
                    vs1: vs2,
                    vs2,
                    vm: false,
                })
            }
            "vmclr.m" => {
                let vd = vector::pseudo::parse_op_format(op)?;
                Vmxormm(Opmvv {
                    dest: vd,
                    vs1: vd,
                    vs2: vd,
                    vm: false,
                })
            }
            "vmset.m" => {
                let vd = vector::pseudo::parse_op_format(op)?;
                Vmxnormm(Opmvv {
                    dest: vd,
                    vs1: vd,
                    vs2: vd,
                    vm: false,
                })
            }
            "vmnot.m" => {
                let (vd, vs2) = vector::pseudo::parse_op_op_format(op)?;
                Vmnandmm(Opmvv {
                    dest: vd,
                    vs1: vs2,
                    vs2,
                    vm: false,
                })
            }
            _ => return Err(format!("Unknown mnemonic: {}", mnemonic)),
        };

        Ok(instruction)
    }

    pub fn decode_data_section(data_line: &str) -> Result<Data, String> {
        let (data_type, values) = Self::split_instruction(data_line);

        match data_type {
            ".byte" => data::parse_bytes(values),
            ".2byte" | ".half" | ".short" => data::parse_halves(values),
            ".4byte" | ".word" | ".long" => data::parse_words(values),
            ".8byte" | ".dword" | ".quad" => data::parse_quads(values),
            ".float" => data::parse_floats(values),
            ".double" => data::parse_doubles(values),
            ".string" => data::parse_string(values),
            ".asciz" => data::parse_asciz(values),
            ".zero" => data::parse_zero(values),
            _ => Err(format!("Unknown data type: {}", data_type)),
        }
    }

    fn rename(old: &str) -> &str {
        match old {
            "vle1.v" => "vlm.v",
            "vse1.v" => "vsm.v",
            "vfredsum.vs" => "vfredusum.vs",
            "vfwredsum.vs" => "vfwredusum.vs",
            "vmandnot.mm" => "vmandn.mm",
            "vmornot.mm" => "vmorn.mm",
            "vpopc.m" => "vcpop.m",
            "vfrsqrte7.v" => "vfrsqrt7.v",
            "vfrece7.v" => "vfrec7.v",
            "vmcpy.m" => "vmmv.m",

            // Technically pseudoinstructions, but without custom parsers
            "vl1r.v" => "vl1re8.v",
            "vl2r.v" => "vl2re8.v",
            "vl4r.v" => "vl4re8.v",
            "vl8r.v" => "vl8re8.v",

            _ => old,
        }
    }

    fn split_instruction(instruction_line: &str) -> (&str, &str) {
        let mut lane = instruction_line.splitn(2, char::is_whitespace);
        let mnemonic = lane.next().unwrap_or_default().trim();
        let operands = lane.next().unwrap_or_default().trim();
        (mnemonic, operands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_correctly() {
        let old_mnemonics = [
            "vle1.v",
            "vse1.v",
            "vfredsum.vs",
            "vfwredsum.vs",
            "vmandnot.mm",
            "vmornot.mm",
            "vpopc.m",
            "vfrsqrte7.v",
            "vfrece7.v",
            "vmcpy.m",
        ];

        old_mnemonics
            .map(|mnemonic| (mnemonic, Decoder::rename(mnemonic)))
            .iter()
            .for_each(|(old, new)| assert_ne!(old, new));
    }

    #[test]
    fn la_works() {
        let mut memory_labels = HashMap::new();

        memory_labels.insert("to_copy".to_owned(), 12);

        let instruction =
            Decoder::decode_text_section("la x1, to_copy", &HashMap::new(), &memory_labels, 0);

        assert_eq!(
            instruction,
            Ok(fuse![
                Auipc(U { rd: 1, imm20: 0 }),
                Addi(I {
                    rd: 1,
                    rs1: 1,
                    imm12: 12
                })
            ])
        );
    }

    #[test]
    fn la_works_far() {
        let mut memory_labels = HashMap::new();

        memory_labels.insert("to_copy".to_owned(), 5000);

        let instruction =
            Decoder::decode_text_section("la x1, to_copy", &HashMap::new(), &memory_labels, 0);

        assert_eq!(
            instruction,
            Ok(fuse![
                Auipc(U { rd: 1, imm20: 1 }),
                Addi(I {
                    rd: 1,
                    rs1: 1,
                    imm12: 904
                })
            ])
        );
    }
}
