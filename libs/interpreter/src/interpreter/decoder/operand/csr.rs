use eeric_core::prelude::*;

use super::integer;

pub fn parse_csrr_format(csrr: &str) -> Result<format::Csrr, String> {
    let tokens: Vec<&str> = csrr.split(',').map(str::trim).collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, csr, rs1', got {} instead",
            csrr
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let csr = parse_operand(tokens[1])?;
    let rs1 = integer::parse_operand(tokens[2])?;

    Ok(format::Csrr { rd, csr, rs1 })
}

pub fn parse_csri_format(csri: &str) -> Result<format::Csri, String> {
    let tokens: Vec<&str> = csri.split(',').map(str::trim).collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, csr, imm', got {} instead",
            csri
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let csr = parse_operand(tokens[1])?;
    let uimm = integer::parse_immediate(tokens[2])?;

    Ok(format::Csri {
        rd,
        csr,
        uimm: uimm as u32 as usize,
    })
}

fn parse_operand(op: &str) -> Result<usize, String> {
    let operand = match op {
        "instret" => alias::INSTRET,
        "instreth" => return Err("instreth is RV32 only CSR".to_owned()),
        "cycle" => alias::CYCLE,
        "cycleh" => return Err("cycleh is RV32 only CSR".to_owned()),
        "time" => alias::TIME,
        "timeh" => return Err("timeh is RV32 only CSR".to_owned()),
        "marchid" => alias::MARCHID,
        "fcsr" => alias::FCSR,
        "fflags" => alias::FFLAGS,
        "frm" => alias::FRM,
        "mstatus" => alias::MSTATUS,
        "vsstatus" => alias::VSSTATUS,
        "vtype" => alias::VTYPE,
        "vl" => alias::VL,
        "vlenb" => alias::VLENB,
        "vstart" => alias::VSTART,
        "vxrm" => alias::VXRM,
        "vxsat" => alias::VXSAT,
        "vcsr" => alias::VCSR,
        _ => return Err(format!("Incorrect or unsupported CSR operand: {}", op)),
    };

    Ok(operand)
}

pub mod pseudo {
    pub fn parse_op_csr_format(op_csr: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = op_csr.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg, csr', got {} instead",
                op_csr
            ));
        }

        let reg = super::integer::parse_operand(tokens[0])?;
        let csr = super::parse_operand(tokens[1])?;

        Ok((reg, csr))
    }

    pub fn parse_csr_op_format(csr_op: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = csr_op.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'csr, xreg', got {} instead",
                csr_op
            ));
        }

        let csr = super::parse_operand(tokens[0])?;
        let reg = super::integer::parse_operand(tokens[1])?;

        Ok((csr, reg))
    }
}
