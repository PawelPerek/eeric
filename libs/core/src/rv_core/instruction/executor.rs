use prelude::*;

mod base;
mod d;
mod f;
mod m;
mod v;
mod zicsr;

mod prelude;
mod vector_context;

use super::Instruction;
pub use vector_context::VectorContext;

pub struct Executor<'core> {
    memory: &'core mut Memory,
    registers: &'core mut Registers,
    vec_engine: &'core mut VectorEngine,
}

impl<'c> Executor<'c> {
    pub fn new(
        registers: &'c mut Registers,
        memory: &'c mut Memory,
        vec_engine: &'c mut VectorEngine,
    ) -> Self {
        Self {
            registers,
            memory,
            vec_engine,
        }
    }

    pub fn execute(&mut self, input: Instruction) -> Result<(), String> {
        use Instruction::*;

        let current_cycle = self.registers.c[CYCLE].read();
        unsafe {
            self.registers.c[CYCLE].set(current_cycle + 1);
        }

        unsafe { self.registers.c[TIME].set(current_cycle) }

        match input {
            Add(args) => base::add(args, &mut self.registers.x),
            Addw(args) => base::addw(args, &mut self.registers.x),
            Sub(args) => base::sub(args, &mut self.registers.x),
            Subw(args) => base::subw(args, &mut self.registers.x),
            Addi(args) => base::addi(args, &mut self.registers.x),
            Addiw(args) => base::addiw(args, &mut self.registers.x),
            Slt(args) => base::slt(args, &mut self.registers.x),
            Slti(args) => base::slti(args, &mut self.registers.x),
            Sltu(args) => base::sltu(args, &mut self.registers.x),
            Sltiu(args) => base::sltiu(args, &mut self.registers.x),
            Lui(args) => base::lui(args, &mut self.registers.x),
            Auipc(args) => base::auipc(args, &mut self.registers.x, self.registers.pc),
            And(args) => base::and(args, &mut self.registers.x),
            Or(args) => base::or(args, &mut self.registers.x),
            Xor(args) => base::xor(args, &mut self.registers.x),
            Andi(args) => base::andi(args, &mut self.registers.x),
            Ori(args) => base::ori(args, &mut self.registers.x),
            Xori(args) => base::xori(args, &mut self.registers.x),
            Sll(args) => base::sll(args, &mut self.registers.x),
            Sllw(args) => base::sllw(args, &mut self.registers.x),
            Srl(args) => base::srl(args, &mut self.registers.x),
            Srlw(args) => base::srlw(args, &mut self.registers.x),
            Sra(args) => base::sra(args, &mut self.registers.x),
            Sraw(args) => base::sraw(args, &mut self.registers.x),
            Slli(args) => base::slli(args, &mut self.registers.x),
            Slliw(args) => base::slliw(args, &mut self.registers.x),
            Srli(args) => base::srli(args, &mut self.registers.x),
            Srliw(args) => base::srliw(args, &mut self.registers.x),
            Srai(args) => base::srai(args, &mut self.registers.x),
            Sraiw(args) => base::sraiw(args, &mut self.registers.x),
            Ld(args) => base::ld(args, &mut self.registers.x, self.memory),
            Lw(args) => base::lw(args, &mut self.registers.x, self.memory),
            Lwu(args) => base::lwu(args, &mut self.registers.x, self.memory),
            Lh(args) => base::lh(args, &mut self.registers.x, self.memory),
            Lhu(args) => base::lhu(args, &mut self.registers.x, self.memory),
            Lb(args) => base::lb(args, &mut self.registers.x, self.memory),
            Lbu(args) => base::lbu(args, &mut self.registers.x, self.memory),
            Sd(args) => base::sd(args, &self.registers.x, self.memory),
            Sw(args) => base::sw(args, &self.registers.x, self.memory),
            Sh(args) => base::sh(args, &self.registers.x, self.memory),
            Sb(args) => base::sb(args, &self.registers.x, self.memory),
            Beq(args) => base::beq(args, &self.registers.x, &mut self.registers.pc),
            Bne(args) => base::bne(args, &self.registers.x, &mut self.registers.pc),
            Bge(args) => base::bge(args, &self.registers.x, &mut self.registers.pc),
            Bgeu(args) => base::bgeu(args, &self.registers.x, &mut self.registers.pc),
            Blt(args) => base::blt(args, &self.registers.x, &mut self.registers.pc),
            Bltu(args) => base::bltu(args, &self.registers.x, &mut self.registers.pc),
            Jal(args) => base::jal(args, &mut self.registers.x, &mut self.registers.pc),
            Jalr(args) => base::jalr(args, &mut self.registers.x, &mut self.registers.pc),

            Csrrw(args) => zicsr::csrrw(args, &mut self.registers.x, &mut self.registers.c)?,
            Csrrs(args) => zicsr::csrrs(args, &mut self.registers.x, &mut self.registers.c)?,
            Csrrc(args) => zicsr::csrrc(args, &mut self.registers.x, &mut self.registers.c)?,
            Csrrwi(args) => zicsr::csrrwi(args, &mut self.registers.x, &mut self.registers.c)?,
            Csrrsi(args) => zicsr::csrrsi(args, &mut self.registers.x, &mut self.registers.c)?,
            Csrrci(args) => zicsr::csrrci(args, &mut self.registers.x, &mut self.registers.c)?,

            Mul(args) => m::mul(args, &mut self.registers.x),
            Mulh(args) => m::mulh(args, &mut self.registers.x),
            Mulhsu(args) => m::mulhsu(args, &mut self.registers.x),
            Mulhu(args) => m::mulhu(args, &mut self.registers.x),
            Div(args) => m::div(args, &mut self.registers.x),
            Divu(args) => m::divu(args, &mut self.registers.x),
            Rem(args) => m::rem(args, &mut self.registers.x),
            Remu(args) => m::remu(args, &mut self.registers.x),
            Mulw(args) => m::mulw(args, &mut self.registers.x),
            Divw(args) => m::divw(args, &mut self.registers.x),
            Divuw(args) => m::divuw(args, &mut self.registers.x),
            Remw(args) => m::remw(args, &mut self.registers.x),
            Remuw(args) => m::remuw(args, &mut self.registers.x),

            Flw(args) => f::flw(args, &self.registers.x, &mut self.registers.f, self.memory),
            Fsw(args) => f::fsw(args, &self.registers.x, &self.registers.f, self.memory),
            Fmadds(args) => f::fmadd::s(args, &mut self.registers.f),
            Fmsubs(args) => f::fmsub::s(args, &mut self.registers.f),
            Fnmsubs(args) => f::fnmsub::s(args, &mut self.registers.f),
            Fnmadds(args) => f::fnmadd::s(args, &mut self.registers.f),
            Fadds(args) => f::fadd::s(args, &mut self.registers.f),
            Fsubs(args) => f::fsub::s(args, &mut self.registers.f),
            Fmuls(args) => f::fmul::s(args, &mut self.registers.f),
            Fdivs(args) => f::fdiv::s(args, &mut self.registers.f),
            Fsqrts(args) => f::fsqrt::s(args, &mut self.registers.f),
            Fsgnjs(args) => f::fsgnj::s(args, &mut self.registers.f),
            Fsgnjns(args) => f::fsgnjn::s(args, &mut self.registers.f),
            Fsgnjxs(args) => f::fsgnjx::s(args, &mut self.registers.f),
            Fmins(args) => f::fmin::s(args, &mut self.registers.f),
            Fmaxs(args) => f::fmax::s(args, &mut self.registers.f),
            Fcvtws(args) => f::fcvt::ws(args, &mut self.registers.x, &self.registers.f),
            Fcvtwus(args) => f::fcvt::wus(args, &mut self.registers.x, &self.registers.f),
            Fmvxw(args) => f::fmv::xw(args, &mut self.registers.x, &self.registers.f),
            Feqs(args) => f::feq::s(args, &mut self.registers.x, &self.registers.f),
            Flts(args) => f::flt::s(args, &mut self.registers.x, &self.registers.f),
            Fles(args) => f::fle::s(args, &mut self.registers.x, &self.registers.f),
            Fclasss(args) => f::fclass::s(args, &mut self.registers.x, &self.registers.f),
            Fcvtsw(args) => f::fcvt::sw(args, &self.registers.x, &mut self.registers.f),
            Fcvtswu(args) => f::fcvt::swu(args, &self.registers.x, &mut self.registers.f),
            Fmvwx(args) => f::fmv::wx(args, &self.registers.x, &mut self.registers.f),
            Fcvtls(args) => f::fcvt::ls(args, &mut self.registers.x, &self.registers.f),
            Fcvtlus(args) => f::fcvt::lus(args, &mut self.registers.x, &self.registers.f),
            Fcvtsl(args) => f::fcvt::sl(args, &self.registers.x, &mut self.registers.f),
            Fcvtslu(args) => f::fcvt::slu(args, &self.registers.x, &mut self.registers.f),

            Fld(args) => d::fld(args, &self.registers.x, &mut self.registers.f, self.memory),
            Fsd(args) => d::fsd(args, &self.registers.x, &self.registers.f, self.memory),
            Fmaddd(args) => d::fmadd::d(args, &mut self.registers.f),
            Fmsubd(args) => d::fmsub::d(args, &mut self.registers.f),
            Fnmsubd(args) => d::fnmsub::d(args, &mut self.registers.f),
            Fnmaddd(args) => d::fnmadd::d(args, &mut self.registers.f),
            Faddd(args) => d::fadd::d(args, &mut self.registers.f),
            Fsubd(args) => d::fsub::d(args, &mut self.registers.f),
            Fmuld(args) => d::fmul::d(args, &mut self.registers.f),
            Fdivd(args) => d::fdiv::d(args, &mut self.registers.f),
            Fsqrtd(args) => d::fsqrt::d(args, &mut self.registers.f),
            Fsgnjd(args) => d::fsgnj::d(args, &mut self.registers.f),
            Fsgnjnd(args) => d::fsgnjn::d(args, &mut self.registers.f),
            Fsgnjxd(args) => d::fsgnjx::d(args, &mut self.registers.f),
            Fmind(args) => d::fmin::d(args, &mut self.registers.f),
            Fmaxd(args) => d::fmax::d(args, &mut self.registers.f),
            Fcvtsd(args) => d::fcvt::sd(args, &mut self.registers.f),
            Fcvtds(args) => d::fcvt::ds(args, &mut self.registers.f),
            Feqd(args) => d::feq::d(args, &mut self.registers.x, &self.registers.f),
            Fltd(args) => d::flt::d(args, &mut self.registers.x, &self.registers.f),
            Fled(args) => d::fle::d(args, &mut self.registers.x, &self.registers.f),
            Fclassd(args) => d::fclass::d(args, &mut self.registers.x, &self.registers.f),
            Fcvtwd(args) => d::fcvt::wd(args, &mut self.registers.x, &self.registers.f),
            Fcvtwud(args) => d::fcvt::wud(args, &mut self.registers.x, &self.registers.f),
            Fcvtdw(args) => d::fcvt::dw(args, &self.registers.x, &mut self.registers.f),
            Fcvtdwu(args) => d::fcvt::dwu(args, &self.registers.x, &mut self.registers.f),
            Fcvtld(args) => d::fcvt::ld(args, &mut self.registers.x, &self.registers.f),
            Fcvtlud(args) => d::fcvt::lud(args, &mut self.registers.x, &self.registers.f),
            Fmvxd(args) => d::fmv::xd(args, &mut self.registers.x, &self.registers.f),
            Fcvtdl(args) => d::fcvt::dl(args, &self.registers.x, &mut self.registers.f),
            Fcvtdlu(args) => d::fcvt::dlu(args, &self.registers.x, &mut self.registers.f),
            Fmvdx(args) => d::fmv::dx(args, &self.registers.x, &mut self.registers.f),

            Fusion(instructions) => {
                // FIXME
                instructions
                    .iter()
                    .map(|instruction| self.execute(instruction.clone()))
                    .collect::<Result<Vec<()>, _>>()?;

                self.registers.pc = self
                    .registers
                    .pc
                    .wrapping_sub(instructions.len() as u64 * 4);
            }

            _ => self.vector_execute(input)?,
        };

        let current_instret = self.registers.c[INSTRET].read();
        unsafe {
            self.registers.c[INSTRET].set(current_instret + 1);
        }
        self.registers.pc = self.registers.pc.wrapping_add(4);

        Ok(())
    }

    fn vector_execute(&mut self, input: Instruction) -> Result<(), String> {
        use Instruction::*;

        let mut vctx = VectorContext {
            v: &mut self.registers.v,
            csr: &mut self.registers.c,
            vec_engine: self.vec_engine,
        };

        match input {
            Vsetvli(args) => v::vsetvli(args, &mut self.registers.x, &mut vctx),
            Vsetivli(args) => v::vsetivli(args, &mut self.registers.x, &mut vctx),
            Vsetvl(args) => v::vsetvl(args, &mut self.registers.x, &mut vctx),

            Vlv { data: args, eew } => {
                v::vl::v(args, eew, &self.registers.x, &mut vctx, self.memory)?
            }
            Vsv { data: args, eew } => v::vs::v(args, eew, &self.registers.x, &vctx, self.memory),

            Vlmv(args) => v::vlm::v(args, &mut vctx, &self.registers.x, self.memory),
            Vsmv(args) => v::vsm::v(args, &vctx, &self.registers.x, self.memory),

            Vlsv { data: args, eew } => {
                v::vls::v(args, eew, &self.registers.x, &mut vctx, self.memory)
            }
            Vssv { data: args, eew } => v::vss::v(args, eew, &vctx, &self.registers.x, self.memory),

            Vluxv { data: args, eew } => {
                v::vlux::v(args, eew, &mut vctx, &self.registers.x, self.memory)
            }
            Vloxv { data: args, eew } => {
                v::vlox::v(args, eew, &mut vctx, &self.registers.x, self.memory)
            }
            Vsuxv { data: args, eew } => {
                v::vsux::v(args, eew, &vctx, &self.registers.x, self.memory)
            }
            Vsoxv { data: args, eew } => {
                v::vsox::v(args, eew, &vctx, &self.registers.x, self.memory)
            }

            Vlffv { data: args, eew } => {
                v::vlff::v(args, eew, &mut vctx, &self.registers.x, self.memory)?
            }

            Vlsegv {
                data: args,
                eew,
                nf,
            } => v::vlseg::v(args, eew, nf, &mut vctx, &self.registers.x, self.memory),
            Vssegv {
                data: args,
                eew,
                nf,
            } => v::vsseg::v(args, eew, nf, &vctx, &self.registers.x, self.memory),

            Vlssegv {
                data: args,
                eew,
                nf,
            } => v::vlsseg::v(args, eew, nf, &mut vctx, &self.registers.x, self.memory),
            Vsssegv {
                data: args,
                eew,
                nf,
            } => v::vssseg::v(args, eew, nf, &vctx, &self.registers.x, self.memory),

            Vluxsegv {
                data: args,
                eew,
                nf,
            } => v::vluxseg::v(args, eew, nf, &mut vctx, &self.registers.x, self.memory),
            Vloxsegv {
                data: args,
                eew,
                nf,
            } => v::vloxseg::v(args, eew, nf, &mut vctx, &self.registers.x, self.memory),
            Vsuxsegv {
                data: args,
                eew,
                nf,
            } => v::vsuxseg::v(args, eew, nf, &vctx, &self.registers.x, self.memory),
            Vsoxsegv {
                data: args,
                eew,
                nf,
            } => v::vsoxseg::v(args, eew, nf, &vctx, &self.registers.x, self.memory),

            Vlrv {
                data: args,
                eew: _,
                nf,
            } => v::vlr::v(args, nf, &mut vctx, &self.registers.x, self.memory),
            Vsrv { data: args, nf } => v::vsr::v(args, nf, &vctx, &self.registers.x, self.memory),

            Vaddvv(args) => v::vadd::vv(args, &mut vctx),
            Vaddvx(args) => v::vadd::vx(args, &mut vctx, &self.registers.x),
            Vaddvi(args) => v::vadd::vi(args, &mut vctx),

            Vsubvv(args) => v::vsub::vv(args, &mut vctx),
            Vsubvx(args) => v::vsub::vx(args, &mut vctx, &self.registers.x),

            Vrsubvx(args) => v::vrsub::vx(args, &mut vctx, &self.registers.x),
            Vrsubvi(args) => v::vrsub::vi(args, &mut vctx),

            Vminuvv(args) => v::vminu::vv(args, &mut vctx),
            Vminuvx(args) => v::vminu::vx(args, &mut vctx, &self.registers.x),

            Vminvv(args) => v::vmin::vv(args, &mut vctx),
            Vminvx(args) => v::vmin::vx(args, &mut vctx, &self.registers.x),

            Vmaxuvv(args) => v::vmaxu::vv(args, &mut vctx),
            Vmaxuvx(args) => v::vmaxu::vx(args, &mut vctx, &self.registers.x),

            Vmaxvv(args) => v::vmax::vv(args, &mut vctx),
            Vmaxvx(args) => v::vmax::vx(args, &mut vctx, &self.registers.x),

            Vandvv(args) => v::vand::vv(args, &mut vctx),
            Vandvx(args) => v::vand::vx(args, &mut vctx, &self.registers.x),
            Vandvi(args) => v::vand::vi(args, &mut vctx),

            Vorvv(args) => v::vor::vv(args, &mut vctx),
            Vorvx(args) => v::vor::vx(args, &mut vctx, &self.registers.x),
            Vorvi(args) => v::vor::vi(args, &mut vctx),

            Vxorvv(args) => v::vxor::vv(args, &mut vctx),
            Vxorvx(args) => v::vxor::vx(args, &mut vctx, &self.registers.x),
            Vxorvi(args) => v::vxor::vi(args, &mut vctx),

            Vrgathervv(args) => v::vrgather::vv(args, &mut vctx),
            Vrgathervx(args) => v::vrgather::vx(args, &mut vctx, &self.registers.x),
            Vrgathervi(args) => v::vrgather::vi(args, &mut vctx),

            Vrgatherei16vv(args) => v::vrgatherei16::vv(args, &mut vctx),

            Vslideupvx(args) => v::vslideup::vx(args, &mut vctx, &self.registers.x),
            Vslideupvi(args) => v::vslideup::vi(args, &mut vctx),

            Vslidedownvx(args) => v::vslidedown::vx(args, &mut vctx, &self.registers.x),
            Vslidedownvi(args) => v::vslidedown::vi(args, &mut vctx),

            Vadcvvm(args) => v::vadc::vvm(args, &mut vctx),
            Vadcvxm(args) => v::vadc::vxm(args, &mut vctx, &self.registers.x),
            Vadcvim(args) => v::vadc::vim(args, &mut vctx),

            Vmadcvvm(args) => v::vmadc::vvm(args, &mut vctx),
            Vmadcvxm(args) => v::vmadc::vxm(args, &mut vctx, &self.registers.x),
            Vmadcvim(args) => v::vmadc::vim(args, &mut vctx),

            Vmadcvv(args) => v::vmadc::vv(args, &mut vctx),
            Vmadcvx(args) => v::vmadc::vx(args, &mut vctx, &self.registers.x),
            Vmadcvi(args) => v::vmadc::vi(args, &mut vctx),

            Vsbcvvm(args) => v::vsbc::vvm(args, &mut vctx),
            Vsbcvxm(args) => v::vsbc::vxm(args, &mut vctx, &self.registers.x),

            Vmsbcvvm(args) => v::vmsbc::vvm(args, &mut vctx),
            Vmsbcvxm(args) => v::vmsbc::vxm(args, &mut vctx, &self.registers.x),

            Vmsbcvv(args) => v::vmsbc::vv(args, &mut vctx),
            Vmsbcvx(args) => v::vmsbc::vx(args, &mut vctx, &self.registers.x),

            Vmergevvm(args) => v::vmerge::vvm(args, &mut vctx),
            Vmergevxm(args) => v::vmerge::vxm(args, &mut vctx, &self.registers.x),
            Vmergevim(args) => v::vmerge::vim(args, &mut vctx),

            Vmvvv(args) => v::vmv::vv(args, &mut vctx),
            Vmvvx(args) => v::vmv::vx(args, &mut vctx, &self.registers.x),
            Vmvvi(args) => v::vmv::vi(args, &mut vctx),

            Vmseqvv(args) => v::vmseq::vv(args, &mut vctx),
            Vmseqvx(args) => v::vmseq::vx(args, &mut vctx, &self.registers.x),
            Vmseqvi(args) => v::vmseq::vi(args, &mut vctx),

            Vmsnevv(args) => v::vmsne::vv(args, &mut vctx),
            Vmsnevx(args) => v::vmsne::vx(args, &mut vctx, &self.registers.x),
            Vmsnevi(args) => v::vmsne::vi(args, &mut vctx),

            Vmsltuvv(args) => v::vmsltu::vv(args, &mut vctx),
            Vmsltuvx(args) => v::vmsltu::vx(args, &mut vctx, &self.registers.x),

            Vmsltvv(args) => v::vmslt::vv(args, &mut vctx),
            Vmsltvx(args) => v::vmslt::vx(args, &mut vctx, &self.registers.x),

            Vmsleuvv(args) => v::vmsleu::vv(args, &mut vctx),
            Vmsleuvx(args) => v::vmsleu::vx(args, &mut vctx, &self.registers.x),
            Vmsleuvi(args) => v::vmsleu::vi(args, &mut vctx),

            Vmslevv(args) => v::vmsle::vv(args, &mut vctx),
            Vmslevx(args) => v::vmsle::vx(args, &mut vctx, &self.registers.x),
            Vmslevi(args) => v::vmsle::vi(args, &mut vctx),

            Vmsgtuvx(args) => v::vmsgtu::vx(args, &mut vctx, &self.registers.x),
            Vmsgtuvi(args) => v::vmsgtu::vi(args, &mut vctx),

            Vmsgtvx(args) => v::vmsgt::vx(args, &mut vctx, &self.registers.x),
            Vmsgtvi(args) => v::vmsgt::vi(args, &mut vctx),

            Vsadduvv(args) => v::vsaddu::vv(args, &mut vctx),
            Vsadduvx(args) => v::vsaddu::vx(args, &mut vctx, &self.registers.x),
            Vsadduvi(args) => v::vsaddu::vi(args, &mut vctx),

            Vsaddvv(args) => v::vsadd::vv(args, &mut vctx),
            Vsaddvx(args) => v::vsadd::vx(args, &mut vctx, &self.registers.x),
            Vsaddvi(args) => v::vsadd::vi(args, &mut vctx),

            Vssubuvv(args) => v::vssubu::vv(args, &mut vctx),
            Vssubuvx(args) => v::vssubu::vx(args, &mut vctx, &self.registers.x),

            Vssubvv(args) => v::vssub::vv(args, &mut vctx),
            Vssubvx(args) => v::vssub::vx(args, &mut vctx, &self.registers.x),

            Vsllvv(args) => v::vsll::vv(args, &mut vctx),
            Vsllvx(args) => v::vsll::vx(args, &mut vctx, &self.registers.x),
            Vsllvi(args) => v::vsll::vi(args, &mut vctx),

            Vsmulvv(args) => v::vsmul::vv(args, &mut vctx),
            Vsmulvx(args) => v::vsmul::vx(args, &mut vctx, &self.registers.x),

            Vmv1rv(args) => v::vmv1r::v(args, &mut vctx),
            Vmv2rv(args) => v::vmv2r::v(args, &mut vctx),
            Vmv4rv(args) => v::vmv4r::v(args, &mut vctx),
            Vmv8rv(args) => v::vmv8r::v(args, &mut vctx),

            Vsrlvv(args) => v::vsrl::vv(args, &mut vctx),
            Vsrlvx(args) => v::vsrl::vx(args, &mut vctx, &self.registers.x),
            Vsrlvi(args) => v::vsrl::vi(args, &mut vctx),

            Vsravv(args) => v::vsra::vv(args, &mut vctx),
            Vsravx(args) => v::vsra::vx(args, &mut vctx, &self.registers.x),
            Vsravi(args) => v::vsra::vi(args, &mut vctx),

            Vssrlvv(args) => v::vssrl::vv(args, &mut vctx),
            Vssrlvx(args) => v::vssrl::vx(args, &mut vctx, &self.registers.x),
            Vssrlvi(args) => v::vssrl::vi(args, &mut vctx),

            Vssravv(args) => v::vssra::vv(args, &mut vctx),
            Vssravx(args) => v::vssra::vx(args, &mut vctx, &self.registers.x),
            Vssravi(args) => v::vssra::vi(args, &mut vctx),

            Vnsrlwv(args) => v::vnsrl::wv(args, &mut vctx)?,
            Vnsrlwx(args) => v::vnsrl::wx(args, &mut vctx, &self.registers.x)?,
            Vnsrlwi(args) => v::vnsrl::wi(args, &mut vctx)?,

            Vnsrawv(args) => v::vnsra::wv(args, &mut vctx)?,
            Vnsrawx(args) => v::vnsra::wx(args, &mut vctx, &self.registers.x)?,
            Vnsrawi(args) => v::vnsra::wi(args, &mut vctx)?,

            Vnclipuwv(args) => v::vnclipu::wv(args, &mut vctx),
            Vnclipuwx(args) => v::vnclipu::wx(args, &mut vctx, &self.registers.x),
            Vnclipuwi(args) => v::vnclipu::wi(args, &mut vctx),

            Vnclipwv(args) => v::vnclip::wv(args, &mut vctx),
            Vnclipwx(args) => v::vnclip::wx(args, &mut vctx, &self.registers.x),
            Vnclipwi(args) => v::vnclip::wi(args, &mut vctx),

            Vwredsumuvs(args) => v::vwredsumu::vs(args, &mut vctx)?,
            Vwredsumvs(args) => v::vwredsum::vs(args, &mut vctx)?,

            Vredsumvs(args) => v::vredsum::vs(args, &mut vctx),
            Vredandvs(args) => v::vredand::vs(args, &mut vctx),
            Vredorvs(args) => v::vredor::vs(args, &mut vctx),
            Vredxorvs(args) => v::vredxor::vs(args, &mut vctx),
            Vredminuvs(args) => v::vredminu::vs(args, &mut vctx),
            Vredminvs(args) => v::vredmin::vs(args, &mut vctx),
            Vredmaxuvs(args) => v::vredmaxu::vs(args, &mut vctx),
            Vredmaxvs(args) => v::vredmax::vs(args, &mut vctx),

            Vaadduvv(args) => v::vaaddu::vv(args, &mut vctx),
            Vaadduvx(args) => v::vaaddu::vx(args, &mut vctx, &self.registers.x),

            Vaaddvv(args) => v::vaadd::vv(args, &mut vctx),
            Vaaddvx(args) => v::vaadd::vx(args, &mut vctx, &self.registers.x),

            Vasubuvv(args) => v::vasubu::vv(args, &mut vctx),
            Vasubuvx(args) => v::vasubu::vx(args, &mut vctx, &self.registers.x),

            Vasubvv(args) => v::vasub::vv(args, &mut vctx),
            Vasubvx(args) => v::vasub::vx(args, &mut vctx, &self.registers.x),

            Vslide1upvx(args) => v::vslide1up::vx(args, &mut vctx, &self.registers.x),

            Vslide1downvx(args) => v::vslide1down::vx(args, &mut vctx, &self.registers.x),

            Vmvxs(args) => v::vmv::xs(args, &vctx, &mut self.registers.x),
            Vcpopm(args) => v::vcpop::m(args, &vctx, &mut self.registers.x),
            Vfirstm(args) => v::vfirst::m(args, &vctx, &mut self.registers.x),

            Vmvsx(args) => v::vmv::sx(args, &mut vctx, &self.registers.x),

            Vzextvf8(args) => v::vzext::vf8(args, &mut vctx)?,
            Vsextvf8(args) => v::vsext::vf8(args, &mut vctx)?,
            Vzextvf4(args) => v::vzext::vf4(args, &mut vctx)?,
            Vsextvf4(args) => v::vsext::vf4(args, &mut vctx)?,
            Vzextvf2(args) => v::vzext::vf2(args, &mut vctx)?,
            Vsextvf2(args) => v::vsext::vf2(args, &mut vctx)?,

            Vmsbfm(args) => v::vmsbf::m(args, &mut vctx),
            Vmsofm(args) => v::vmsof::m(args, &mut vctx),
            Vmsifm(args) => v::vmsif::m(args, &mut vctx),
            Viotam(args) => v::viota::m(args, &mut vctx),
            Vidv(args) => v::vid::v(args, &mut vctx),

            Vcompressvm(args) => v::vcompress::vm(args, &mut vctx),

            Vmandnmm(args) => v::vmandn::mm(args, &mut vctx),
            Vmandmm(args) => v::vmand::mm(args, &mut vctx),
            Vmormm(args) => v::vmor::mm(args, &mut vctx),
            Vmxormm(args) => v::vmxor::mm(args, &mut vctx),
            Vmornmm(args) => v::vmorn::mm(args, &mut vctx),
            Vmnandmm(args) => v::vmnand::mm(args, &mut vctx),
            Vmnormm(args) => v::vmnor::mm(args, &mut vctx),
            Vmxnormm(args) => v::vmxnor::mm(args, &mut vctx),

            Vdivuvv(args) => v::vdivu::vv(args, &mut vctx),
            Vdivuvx(args) => v::vdivu::vx(args, &mut vctx, &self.registers.x),

            Vdivvv(args) => v::vdiv::vv(args, &mut vctx),
            Vdivvx(args) => v::vdiv::vx(args, &mut vctx, &self.registers.x),

            Vremuvv(args) => v::vremu::vv(args, &mut vctx),
            Vremuvx(args) => v::vremu::vx(args, &mut vctx, &self.registers.x),

            Vremvv(args) => v::vrem::vv(args, &mut vctx),
            Vremvx(args) => v::vrem::vx(args, &mut vctx, &self.registers.x),

            Vmulhuvv(args) => v::vmulhu::vv(args, &mut vctx),
            Vmulhuvx(args) => v::vmulhu::vx(args, &mut vctx, &self.registers.x),

            Vmulvv(args) => v::vmul::vv(args, &mut vctx),
            Vmulvx(args) => v::vmul::vx(args, &mut vctx, &self.registers.x),

            Vmulhsuvv(args) => v::vmulhsu::vv(args, &mut vctx),
            Vmulhsuvx(args) => v::vmulhsu::vx(args, &mut vctx, &self.registers.x),

            Vmulhvv(args) => v::vmulh::vv(args, &mut vctx),
            Vmulhvx(args) => v::vmulh::vx(args, &mut vctx, &self.registers.x),

            Vmaddvv(args) => v::vmadd::vv(args, &mut vctx),
            Vmaddvx(args) => v::vmadd::vx(args, &mut vctx, &self.registers.x),

            Vnmsubvv(args) => v::vnmsub::vv(args, &mut vctx),
            Vnmsubvx(args) => v::vnmsub::vx(args, &mut vctx, &self.registers.x),

            Vmaccvv(args) => v::vmacc::vv(args, &mut vctx),
            Vmaccvx(args) => v::vmacc::vx(args, &mut vctx, &self.registers.x),

            Vnmsacvv(args) => v::vnmsac::vv(args, &mut vctx),
            Vnmsacvx(args) => v::vnmsac::vx(args, &mut vctx, &self.registers.x),

            Vwadduvv(args) => v::vwaddu::vv(args, &mut vctx)?,
            Vwadduvx(args) => v::vwaddu::vx(args, &mut vctx, &self.registers.x)?,

            Vwaddvv(args) => v::vwadd::vv(args, &mut vctx)?,
            Vwaddvx(args) => v::vwadd::vx(args, &mut vctx, &self.registers.x)?,

            Vwsubuvv(args) => v::vwsubu::vv(args, &mut vctx)?,
            Vwsubuvx(args) => v::vwsubu::vx(args, &mut vctx, &self.registers.x)?,

            Vwsubvv(args) => v::vwsub::vv(args, &mut vctx)?,
            Vwsubvx(args) => v::vwsub::vx(args, &mut vctx, &self.registers.x)?,

            Vwadduwv(args) => v::vwaddu::wv(args, &mut vctx)?,
            Vwadduwx(args) => v::vwaddu::wx(args, &mut vctx, &self.registers.x)?,

            Vwaddwv(args) => v::vwadd::wv(args, &mut vctx)?,
            Vwaddwx(args) => v::vwadd::wx(args, &mut vctx, &self.registers.x)?,

            Vwsubuwv(args) => v::vwsubu::wv(args, &mut vctx)?,
            Vwsubuwx(args) => v::vwsubu::wx(args, &mut vctx, &self.registers.x)?,

            Vwsubwv(args) => v::vwsub::wv(args, &mut vctx)?,
            Vwsubwx(args) => v::vwsub::wx(args, &mut vctx, &self.registers.x)?,

            Vwmuluvv(args) => v::vwmulu::vv(args, &mut vctx)?,
            Vwmuluvx(args) => v::vwmulu::vx(args, &mut vctx, &self.registers.x)?,

            Vwmulsuvv(args) => v::vwmulsu::vv(args, &mut vctx)?,
            Vwmulsuvx(args) => v::vwmulsu::vx(args, &mut vctx, &self.registers.x)?,

            Vwmulvv(args) => v::vwmul::vv(args, &mut vctx)?,
            Vwmulvx(args) => v::vwmul::vx(args, &mut vctx, &self.registers.x)?,

            Vwmaccuvv(args) => v::vwmaccu::vv(args, &mut vctx)?,
            Vwmaccuvx(args) => v::vwmaccu::vx(args, &mut vctx, &self.registers.x)?,

            Vwmaccvv(args) => v::vwmacc::vv(args, &mut vctx)?,
            Vwmaccvx(args) => v::vwmacc::vx(args, &mut vctx, &self.registers.x)?,

            Vwmaccusvx(args) => v::vwmaccus::vx(args, &mut vctx, &self.registers.x)?,

            Vwmaccsuvv(args) => v::vwmaccsu::vv(args, &mut vctx)?,
            Vwmaccsuvx(args) => v::vwmaccsu::vx(args, &mut vctx, &self.registers.x)?,

            Vfaddvv(args) => v::vfadd::vv(args, &mut vctx)?,
            Vfaddvf(args) => v::vfadd::vf(args, &mut vctx, &self.registers.f)?,

            Vfredusumvs(args) => v::vfredusum::vs(args, &mut vctx)?,

            Vfsubvv(args) => v::vfsub::vv(args, &mut vctx)?,
            Vfsubvf(args) => v::vfsub::vf(args, &mut vctx, &self.registers.f)?,

            Vfredosumvs(args) => v::vfredosum::vs(args, &mut vctx)?,

            Vfminvv(args) => v::vfmin::vv(args, &mut vctx)?,
            Vfminvf(args) => v::vfmin::vf(args, &mut vctx, &self.registers.f)?,

            Vfredminvs(args) => v::vfredmin::vs(args, &mut vctx)?,

            Vfmaxvv(args) => v::vfmax::vv(args, &mut vctx)?,
            Vfmaxvf(args) => v::vfmax::vf(args, &mut vctx, &self.registers.f)?,

            Vfredmaxvs(args) => v::vfredmax::vs(args, &mut vctx)?,

            Vfsgnjvv(args) => v::vfsgnj::vv(args, &mut vctx)?,
            Vfsgnjvf(args) => v::vfsgnj::vf(args, &mut vctx, &self.registers.f)?,

            Vfsgnjnvv(args) => v::vfsgnjn::vv(args, &mut vctx)?,
            Vfsgnjnvf(args) => v::vfsgnjn::vf(args, &mut vctx, &self.registers.f)?,

            Vfsgnjxvv(args) => v::vfsgnjx::vv(args, &mut vctx)?,
            Vfsgnjxvf(args) => v::vfsgnjx::vf(args, &mut vctx, &self.registers.f)?,

            Vfslide1upvf(args) => v::vfslide1up::vf(args, &mut vctx, &self.registers.f)?,

            Vfslide1downvf(args) => v::vfslide1down::vf(args, &mut vctx, &self.registers.f)?,

            Vfmvfs(args) => v::vfmv::fs(args, &vctx, &mut self.registers.f)?,

            Vfmvsf(args) => v::vfmv::sf(args, &mut vctx, &self.registers.f)?,

            Vfcvtxufv(args) => v::vfcvt::xufv(args, &mut vctx)?,
            Vfcvtxfv(args) => v::vfcvt::xfv(args, &mut vctx)?,
            Vfcvtfxuv(args) => v::vfcvt::fxuv(args, &mut vctx)?,
            Vfcvtfxv(args) => v::vfcvt::fxv(args, &mut vctx)?,
            VfcvtRtzxufv(args) => v::vfcvt::rtzxufv(args, &mut vctx)?,
            VfcvtRtzxfv(args) => v::vfcvt::rtzxfv(args, &mut vctx)?,

            Vfwcvtxufv(args) => v::vfwcvt::xufv(args, &mut vctx)?,
            Vfwcvtxfv(args) => v::vfwcvt::xfv(args, &mut vctx)?,
            Vfwcvtfxuv(args) => v::vfwcvt::fxuv(args, &mut vctx)?,
            Vfwcvtfxv(args) => v::vfwcvt::fxv(args, &mut vctx)?,
            Vfwcvtffv(args) => v::vfwcvt::ffv(args, &mut vctx)?,
            VfwcvtRtzxufv(args) => v::vfwcvt::rtzxufv(args, &mut vctx)?,
            VfwcvtRtzxfv(args) => v::vfwcvt::rtzxfv(args, &mut vctx)?,

            Vfncvtxufw(args) => v::vfncvt::xufw(args, &mut vctx)?,
            Vfncvtxfw(args) => v::vfncvt::xfw(args, &mut vctx)?,
            Vfncvtfxuw(args) => v::vfncvt::fxuw(args, &mut vctx)?,
            Vfncvtfxw(args) => v::vfncvt::fxw(args, &mut vctx)?,
            Vfncvtffw(args) => v::vfncvt::ffw(args, &mut vctx)?,
            VfncvtRodffw(args) => v::vfncvt::rodffw(args, &mut vctx)?,
            VfncvtRtzxufw(args) => v::vfncvt::rtzxufw(args, &mut vctx)?,
            VfncvtRtzxfw(args) => v::vfncvt::rtzxfw(args, &mut vctx)?,

            Vfsqrtv(args) => v::vfsqrt::v(args, &mut vctx)?,
            Vfrsqrt7v(args) => v::vfrsqrt7::v(args, &mut vctx)?,
            Vfrec7v(args) => v::vfrec7::v(args, &mut vctx)?,
            Vfclassv(args) => v::vfclass::v(args, &mut vctx)?,

            Vfmergevfm(args) => v::vfmerge::vfm(args, &mut vctx, &self.registers.f)?,
            Vfmvvf(args) => v::vfmv::vf(args, &mut vctx, &self.registers.f)?,

            Vmfeqvv(args) => v::vmfeq::vv(args, &mut vctx)?,
            Vmfeqvf(args) => v::vmfeq::vf(args, &mut vctx, &self.registers.f)?,

            Vmflevv(args) => v::vmfle::vv(args, &mut vctx)?,
            Vmflevf(args) => v::vmfle::vf(args, &mut vctx, &self.registers.f)?,

            Vmfltvv(args) => v::vmflt::vv(args, &mut vctx)?,
            Vmfltvf(args) => v::vmflt::vf(args, &mut vctx, &self.registers.f)?,

            Vmfnevv(args) => v::vmfne::vv(args, &mut vctx)?,
            Vmfnevf(args) => v::vmfne::vf(args, &mut vctx, &self.registers.f)?,

            Vmfgtvf(args) => v::vmfgt::vf(args, &mut vctx, &self.registers.f)?,

            Vmfgevf(args) => v::vmfge::vf(args, &mut vctx, &self.registers.f)?,

            Vfdivvv(args) => v::vfdiv::vv(args, &mut vctx)?,
            Vfdivvf(args) => v::vfdiv::vf(args, &mut vctx, &self.registers.f)?,

            Vfrdivvf(args) => v::vfrdiv::vf(args, &mut vctx, &self.registers.f)?,

            Vfmulvv(args) => v::vfmul::vv(args, &mut vctx)?,
            Vfmulvf(args) => v::vfmul::vf(args, &mut vctx, &self.registers.f)?,

            Vfrsubvf(args) => v::vfrsub::vf(args, &mut vctx, &self.registers.f)?,

            Vfmaddvv(args) => v::vfmadd::vv(args, &mut vctx)?,
            Vfmaddvf(args) => v::vfmadd::vf(args, &mut vctx, &self.registers.f)?,

            Vfnmaddvv(args) => v::vfnmadd::vv(args, &mut vctx)?,
            Vfnmaddvf(args) => v::vfnmadd::vf(args, &mut vctx, &self.registers.f)?,

            Vfmsubvv(args) => v::vfmsub::vv(args, &mut vctx)?,
            Vfmsubvf(args) => v::vfmsub::vf(args, &mut vctx, &self.registers.f)?,

            Vfnmsubvv(args) => v::vfnmsub::vv(args, &mut vctx)?,
            Vfnmsubvf(args) => v::vfnmsub::vf(args, &mut vctx, &self.registers.f)?,

            Vfmaccvv(args) => v::vfmacc::vv(args, &mut vctx)?,
            Vfmaccvf(args) => v::vfmacc::vf(args, &mut vctx, &self.registers.f)?,

            Vfnmaccvv(args) => v::vfnmacc::vv(args, &mut vctx)?,
            Vfnmaccvf(args) => v::vfnmacc::vf(args, &mut vctx, &self.registers.f)?,

            Vfmsacvv(args) => v::vfmsac::vv(args, &mut vctx)?,
            Vfmsacvf(args) => v::vfmsac::vf(args, &mut vctx, &self.registers.f)?,

            Vfnmsacvv(args) => v::vfnmsac::vv(args, &mut vctx)?,
            Vfnmsacvf(args) => v::vfnmsac::vf(args, &mut vctx, &self.registers.f)?,

            Vfwaddvv(args) => v::vfwadd::vv(args, &mut vctx)?,
            Vfwaddvf(args) => v::vfwadd::vf(args, &mut vctx, &self.registers.f)?,

            Vfwredusumvs(args) => v::vfwredusum::vs(args, &mut vctx)?,

            Vfwsubvv(args) => v::vfwsub::vv(args, &mut vctx)?,
            Vfwsubvf(args) => v::vfwsub::vf(args, &mut vctx, &self.registers.f)?,

            Vfwredosumvs(args) => v::vfwredosum::vs(args, &mut vctx)?,

            Vfwaddwv(args) => v::vfwadd::wv(args, &mut vctx)?,
            Vfwaddwf(args) => v::vfwadd::wf(args, &mut vctx, &self.registers.f)?,

            Vfwsubwv(args) => v::vfwsub::wv(args, &mut vctx)?,
            Vfwsubwf(args) => v::vfwsub::wf(args, &mut vctx, &self.registers.f)?,

            Vfwmulvv(args) => v::vfwmul::vv(args, &mut vctx)?,
            Vfwmulvf(args) => v::vfwmul::vf(args, &mut vctx, &self.registers.f)?,

            Vfwmaccvv(args) => v::vfwmacc::vv(args, &mut vctx)?,
            Vfwmaccvf(args) => v::vfwmacc::vf(args, &mut vctx, &self.registers.f)?,

            Vfwnmaccvv(args) => v::vfwnmacc::vv(args, &mut vctx)?,
            Vfwnmaccvf(args) => v::vfwnmacc::vf(args, &mut vctx, &self.registers.f)?,

            Vfwmsacvv(args) => v::vfwmsac::vv(args, &mut vctx)?,
            Vfwmsacvf(args) => v::vfwmsac::vf(args, &mut vctx, &self.registers.f)?,

            Vfwnmsacvv(args) => v::vfwnmsac::vv(args, &mut vctx)?,
            Vfwnmsacvf(args) => v::vfwnmsac::vf(args, &mut vctx, &self.registers.f)?,

            _ => unreachable!(),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rv_core::{instruction::format, RvCoreBuilder};

    use super::*;

    #[test]
    fn integration_add() {
        use Instruction::*;

        let instructions = vec![
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Addi(I {
                rd: 2,
                rs1: 0,
                imm12: 1024,
            }),
            Add(R {
                rd: 1,
                rs1: 1,
                rs2: 1,
            }),
            Vsetvli(format::Vsetvli {
                rd: 0,
                rs1: 0,
                vtypei: 0b0_0_001_001,
            }),
            Bne(S {
                rs1: 1,
                rs2: 2,
                imm12: -12,
            }),
        ];

        let mut core = RvCoreBuilder::default().instructions(instructions).build();

        for _ in core.run() {}
    }

    #[test]
    fn integration_strcmp() {
        use Instruction::*;
        let instructions = vec![
            Addi(I {
                rd: 6,
                rs1: 0,
                imm12: 0,
            }),
            Vsetvli(format::Vsetvli {
                rd: 5,
                rs1: 0,
                vtypei: 193,
            }),
            Add(R {
                rd: 10,
                rs1: 10,
                rs2: 6,
            }),
            Vlffv {
                data: Vl {
                    vd: 8,
                    rs1: 10,
                    vm: false,
                },
                eew: BaseSew::E8,
            },
            Add(R {
                rd: 11,
                rs1: 11,
                rs2: 6,
            }),
            Vlffv {
                data: Vl {
                    vd: 16,
                    rs1: 11,
                    vm: false,
                },
                eew: BaseSew::E8,
            },
            Vmseqvi(Opivi {
                vd: 0,
                imm5: 0,
                vs2: 8,
                vm: false,
            }),
            Vmsnevv(Opivv {
                vd: 1,
                vs1: 16,
                vs2: 8,
                vm: false,
            }),
            Vmormm(Opmvv {
                dest: 0,
                vs1: 1,
                vs2: 0,
                vm: false,
            }),
            Vfirstm(Opmvv {
                dest: 12,
                vs1: 0,
                vs2: 0,
                vm: false,
            }),
            Csrrs(Csrr {
                rd: 6,
                rs1: 0,
                csr: 3104,
            }),
            Blt(S {
                rs1: 12,
                rs2: 0,
                imm12: -44,
            }),
            Add(R {
                rd: 10,
                rs1: 10,
                rs2: 12,
            }),
            Lbu(I {
                rd: 13,
                rs1: 10,
                imm12: 0,
            }),
            Add(R {
                rd: 11,
                rs1: 11,
                rs2: 12,
            }),
            Lbu(I {
                rd: 14,
                rs1: 11,
                imm12: 0,
            }),
            Sub(R {
                rd: 10,
                rs1: 13,
                rs2: 14,
            }),
            Jalr(I {
                rd: 0,
                rs1: 1,
                imm12: 0,
            }),
        ];

        let _core = RvCoreBuilder::default().instructions(instructions).build();

        // for _ in core.run() {}
    }

    #[test]
    fn integration_memcpy() {
        use Instruction::*;
        let instructions = vec![
            Addi(I {
                rd: 10,
                rs1: 0,
                imm12: 400,
            }),
            Addi(I {
                rd: 11,
                rs1: 0,
                imm12: 0,
            }),
            Addi(I {
                rd: 12,
                rs1: 0,
                imm12: 128,
            }),
            Addi(I {
                rd: 13,
                rs1: 10,
                imm12: 0,
            }),
            Vsetvli(format::Vsetvli {
                rd: 5,
                rs1: 12,
                vtypei: 195,
            }),
            Vlv {
                data: Vl {
                    vd: 0,
                    rs1: 11,
                    vm: false,
                },
                eew: BaseSew::E8,
            },
            Add(R {
                rd: 11,
                rs1: 11,
                rs2: 5,
            }),
            Sub(R {
                rd: 12,
                rs1: 12,
                rs2: 5,
            }),
            Vsv {
                data: Vs {
                    vs3: 0,
                    rs1: 13,
                    vm: false,
                },
                eew: BaseSew::E8,
            },
            Add(R {
                rd: 13,
                rs1: 13,
                rs2: 5,
            }),
            Bne(S {
                rs1: 12,
                rs2: 0,
                imm12: -28,
            }),
            Jalr(I {
                rd: 0,
                rs1: 1,
                imm12: 0,
            }),
        ];

        let _core = RvCoreBuilder::default().instructions(instructions).build();

        // for _ in core.run() {}
    }

    #[test]
    fn integration_first_label() {
        use Instruction::*;
        let instructions = vec![
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Add(R {
                rd: 1,
                rs1: 1,
                rs2: 1,
            }),
            Beq(S {
                rs1: 0,
                rs2: 0,
                imm12: -12,
            }),
        ];

        let _core = RvCoreBuilder::default().instructions(instructions).build();

        // for _ in core.run() {}
    }

    #[test]
    fn integration_graceful_finish() {
        use Instruction::*;
        let instructions = vec![
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
            Addi(I {
                rd: 1,
                rs1: 0,
                imm12: 1,
            }),
        ];

        let mut core = RvCoreBuilder::default().instructions(instructions).build();

        println!("{:?}", core.step());
        println!("{:?}", core.step());
        println!("{:?}", core.step());
        println!("{:?}", core.step());
        println!("{:?}", core.step());
        println!("{:?}", core.step());
    }

    #[test]
    fn integration_load() {
        use Instruction::*;
        let instructions = vec![
            Addi(I {
                rd: 10,
                rs1: 0,
                imm12: 10,
            }),
            Vsetvli(format::Vsetvli {
                rd: 14,
                rs1: 10,
                vtypei: 211,
            }),
            Vlv {
                data: Vl {
                    vd: 0,
                    rs1: 11,
                    vm: false,
                },
                eew: BaseSew::E32,
            },
            Sub(R {
                rd: 10,
                rs1: 10,
                rs2: 14,
            }),
            Slli(I {
                rd: 14,
                rs1: 14,
                imm12: 2,
            }),
            Add(R {
                rd: 11,
                rs1: 11,
                rs2: 14,
            }),
            Vlv {
                data: Vl {
                    vd: 8,
                    rs1: 12,
                    vm: false,
                },
                eew: BaseSew::E32,
            },
            Vfmaccvf(Opfvf {
                vd: 8,
                rs1: 10,
                vs2: 0,
                vm: false,
            }),
            Vsv {
                data: Vs {
                    vs3: 8,
                    rs1: 12,
                    vm: false,
                },
                eew: BaseSew::E32,
            },
            Add(R {
                rd: 12,
                rs1: 12,
                rs2: 14,
            }),
            Bne(S {
                rs1: 10,
                rs2: 0,
                imm12: -40,
            }),
            Jalr(I {
                rd: 0,
                rs1: 1,
                imm12: 0,
            }),
        ];

        let mut core = RvCoreBuilder::default().instructions(instructions).build();

        core.step();
        core.step();
        core.step();
        core.step();
        core.step();
        core.step();
    }
}
