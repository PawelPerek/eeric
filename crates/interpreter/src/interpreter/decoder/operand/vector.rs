use eeric_core::prelude::*;

use super::{float, integer};

fn construct_vtype(
    (sew, lmul, tail, mask): (BaseSew, Lmul, MaskBehavior, MaskBehavior),
) -> Result<u32, String> {
    let vsew = match sew {
        BaseSew::E8 => 0b000,
        BaseSew::E16 => 0b001,
        BaseSew::E32 => 0b010,
        BaseSew::E64 => 0b011,
    };

    let vlmul = match lmul {
        Lmul::MF8 => 0b101,
        Lmul::MF4 => 0b110,
        Lmul::MF2 => 0b111,
        Lmul::M1 => 0b000,
        Lmul::M2 => 0b001,
        Lmul::M4 => 0b010,
        Lmul::M8 => 0b011,
    };

    use MaskBehavior as MB;

    let (vta, vma) = match (tail, mask) {
        (MB::Undisturbed, MB::Undisturbed) => (0, 0),
        (MB::Undisturbed, MB::Agnostic) => (0, 1),
        (MB::Agnostic, MB::Undisturbed) => (1, 0),
        (MB::Agnostic, MB::Agnostic) => (1, 1),
    };

    let result = (vma << 7) | (vta << 6) | (vsew << 3) | vlmul;

    Ok(result)
}

fn parse_vtype(vtype: &[&str]) -> Result<(BaseSew, Lmul, MaskBehavior, MaskBehavior), String> {
    let sew = match vtype[0] {
        "e8" => BaseSew::E8,
        "e16" => BaseSew::E16,
        "e32" => BaseSew::E32,
        "e64" => BaseSew::E64,
        other => return Err(format!("Unknown BaseSew value: {}", other)),
    };

    let lmul = match vtype[1] {
        "mf8" => Lmul::MF8,
        "mf4" => Lmul::MF4,
        "mf2" => Lmul::MF2,
        "m1" => Lmul::M1,
        "m2" => Lmul::M2,
        "m4" => Lmul::M4,
        "m8" => Lmul::M8,
        other => return Err(format!("Unknown Lmul value: {}", other)),
    };

    let tail = match vtype[2] {
        "ta" => MaskBehavior::Agnostic,
        "tu" => MaskBehavior::Undisturbed,
        other => return Err(format!("Unknown tail value: {}", other)),
    };

    let mask = match vtype[3] {
        "ma" => MaskBehavior::Agnostic,
        "mu" => MaskBehavior::Undisturbed,
        other => return Err(format!("Unknown mask value: {}", other)),
    };

    Ok((sew, lmul, tail, mask))
}

pub fn parse_vsetvli_format(vsetvli: &str) -> Result<format::Vsetvli, String> {
    let tokens: Vec<&str> = vsetvli.split(',').map(str::trim).collect();
    if tokens.len() != 6 {
        return Err(format!(
            "Expected format: 'rd, rs1, BaseSew, Lmul, ta/tu, ma/mu', got {} instead",
            vsetvli
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let rs1 = integer::parse_operand(tokens[1])?;
    let vtype = parse_vtype(&tokens[2..])?;

    Ok(format::Vsetvli {
        rd,
        rs1,
        vtypei: construct_vtype(vtype)?,
    })
}

pub fn parse_vsetivli_format(vsetivli: &str) -> Result<format::Vsetivli, String> {
    let tokens: Vec<&str> = vsetivli.split(',').map(str::trim).collect();
    if tokens.len() != 6 {
        return Err(format!(
            "Expected format: 'rd, uimm5, BaseSew, Lmul, ta/tu, ma/mu', got {} instead",
            vsetivli
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let uimm = integer::parse_immediate(tokens[1])? as u32;

    let vtype = parse_vtype(&tokens[2..])?;

    Ok(format::Vsetivli {
        rd,
        uimm,
        vtypei: construct_vtype(vtype)?,
    })
}

pub fn parse_vsetvl_format(vsetvl: &str) -> Result<format::Vsetvl, String> {
    let tokens: Vec<&str> = vsetvl.split(',').map(str::trim).collect();
    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, rs1, rs2', got {} instead",
            vsetvl
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let rs1 = integer::parse_operand(tokens[1])?;
    let rs2 = integer::parse_operand(tokens[2])?;

    Ok(format::Vsetvl { rd, rs1, rs2 })
}

pub fn parse_vl_format(vl: &str) -> Result<format::Vl, String> {
    let tokens: Vec<&str> = vl.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, (rs1), [vm]', got {} instead",
            vl
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vl { vd, rs1, vm })
}

pub fn parse_vlm_format(vlm: &str) -> Result<format::Vl, String> {
    let tokens: Vec<&str> = vlm.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, (rs1)', got {} instead", vlm));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;

    Ok(format::Vl { vd, rs1, vm: false })
}

pub fn parse_vls_format(vls: &str) -> Result<format::Vls, String> {
    let tokens: Vec<&str> = vls.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, (rs1), rs2, [vm]', got {} instead",
            vls
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let rs2 = integer::parse_operand(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vls { vd, rs1, rs2, vm })
}

pub fn parse_vlx_format(vlx: &str) -> Result<format::Vlx, String> {
    let tokens: Vec<&str> = vlx.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, (rs1), vs2, [vm]', got {} instead",
            vlx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vlx { vd, rs1, vs2, vm })
}

pub fn parse_vlr_format(vlr: &str) -> Result<format::Vlr, String> {
    let tokens: Vec<&str> = vlr.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, (rs1)', got {} instead", vlr));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;

    Ok(format::Vlr { vd, rs1 })
}

pub fn parse_vs_format(vs: &str) -> Result<format::Vs, String> {
    let tokens: Vec<&str> = vs.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vs3, (rs1), [vm]', got {} instead",
            vs
        ));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vs { vs3, rs1, vm })
}

pub fn parse_vsm_format(vsm: &str) -> Result<format::Vs, String> {
    let tokens: Vec<&str> = vsm.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vs3, (rs1)', got {} instead",
            vsm
        ));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;

    Ok(format::Vs {
        vs3,
        rs1,
        vm: false,
    })
}

pub fn parse_vss_format(vss: &str) -> Result<format::Vss, String> {
    let tokens: Vec<&str> = vss.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vs3, (rs1), rs2, [vm]', got {} instead",
            vss
        ));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let rs2 = integer::parse_operand(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vss { vs3, rs1, rs2, vm })
}

pub fn parse_vsx_format(vsx: &str) -> Result<format::Vsx, String> {
    let tokens: Vec<&str> = vsx.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vs3, (rs1), vs2, [vm]', got {} instead",
            vsx
        ));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vsx { vs3, rs1, vs2, vm })
}

pub fn parse_vsr_format(vsr: &str) -> Result<format::Vsr, String> {
    let tokens: Vec<&str> = vsr.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vs3, (rs1)', got {} instead",
            vsr
        ));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(tokens[1])?;

    Ok(format::Vsr { vs3, rs1 })
}

pub fn parse_opivv_format(opivv: &str) -> Result<format::Opivv, String> {
    let tokens: Vec<&str> = opivv.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1, [vm]', got {} instead",
            opivv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opivv { vd, vs2, vs1, vm })
}

pub fn parse_opivv_v0_format(opivv: &str) -> Result<format::Opivv, String> {
    let tokens: Vec<&str> = opivv.split(',').map(str::trim).collect();
    if tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1, v0', got {} instead",
            opivv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    if parse_operand(tokens[3])?.as_register()? != 0 {
        return Err("Expected last operand to be v0".to_owned());
    }

    Ok(format::Opivv {
        vd,
        vs2,
        vs1,
        vm: true,
    })
}

pub fn parse_opivv_maskless_format(opivv: &str) -> Result<format::Opivv, String> {
    let tokens: Vec<&str> = opivv.split(',').map(str::trim).collect();
    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1', got {} instead",
            opivv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;

    Ok(format::Opivv {
        vd,
        vs2,
        vs1,
        vm: true,
    })
}

pub fn parse_opivv_vmv_format(opivv_vmv: &str) -> Result<format::Opivv, String> {
    let tokens: Vec<&str> = opivv_vmv.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, vs1', got {} instead",
            opivv_vmv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs1 = parse_operand(tokens[1])?.as_register()?;

    Ok(format::Opivv {
        vd,
        vs2: 0,
        vs1,
        vm: false,
    })
}

pub fn parse_opivx_format(opivx: &str) -> Result<format::Opivx, String> {
    let tokens: Vec<&str> = opivx.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, rs1, [vm]', got {} instead",
            opivx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opivx { vd, vs2, rs1, vm })
}

pub fn parse_opivx_v0_format(opivx: &str) -> Result<format::Opivx, String> {
    let tokens: Vec<&str> = opivx.split(',').map(str::trim).collect();
    if tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, rs1, v0', got {} instead",
            opivx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;
    if parse_operand(tokens[3])?.as_register()? != 0 {
        return Err("Expected last operand to be v0".to_owned());
    }

    Ok(format::Opivx {
        vd,
        vs2,
        rs1,
        vm: false,
    })
}

pub fn parse_opivx_maskless_format(opivx: &str) -> Result<format::Opivx, String> {
    let tokens: Vec<&str> = opivx.split(',').map(str::trim).collect();
    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, rs1', got {} instead",
            opivx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;

    Ok(format::Opivx {
        vd,
        vs2,
        rs1,
        vm: false,
    })
}

pub fn parse_opivx_vmv_format(opivx_vmv: &str) -> Result<format::Opivx, String> {
    let tokens: Vec<&str> = opivx_vmv.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, rs1', got {} instead",
            opivx_vmv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[1])?;

    Ok(format::Opivx {
        vd,
        vs2: 0,
        rs1,
        vm: false,
    })
}

pub fn parse_opivi_format(opivi: &str) -> Result<format::Opivi, String> {
    let tokens: Vec<&str> = opivi.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, imm, [vm]', got {} instead",
            opivi
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let imm = integer::parse_immediate(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opivi {
        vd,
        vs2,
        imm5: imm,
        vm,
    })
}

pub fn parse_opivi_v0_format(opivi: &str) -> Result<format::Opivi, String> {
    let tokens: Vec<&str> = opivi.split(',').map(str::trim).collect();
    if tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, imm, v0', got {} instead",
            opivi
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let imm = integer::parse_immediate(tokens[2])?;
    if parse_operand(tokens[3])?.as_register()? != 0 {
        return Err("Expected last operand to be v0".to_owned());
    }

    Ok(format::Opivi {
        vd,
        vs2,
        imm5: imm,
        vm: false,
    })
}

pub fn parse_opivi_maskless_format(opivi: &str) -> Result<format::Opivi, String> {
    let tokens: Vec<&str> = opivi.split(',').map(str::trim).collect();
    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, imm', got {} instead",
            opivi
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let imm = integer::parse_immediate(tokens[2])?;

    Ok(format::Opivi {
        vd,
        vs2,
        imm5: imm,
        vm: false,
    })
}

pub fn parse_opivi_vmv_format(opivi_vmv: &str) -> Result<format::Opivi, String> {
    let tokens: Vec<&str> = opivi_vmv.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, imm', got {} instead",
            opivi_vmv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let imm = integer::parse_immediate(tokens[1])?;

    Ok(format::Opivi {
        vd,
        vs2: 0,
        imm5: imm,
        vm: false,
    })
}

pub fn parse_opmvv_format(opmvv: &str) -> Result<format::Opmvv, String> {
    let tokens: Vec<&str> = opmvv.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1, [vm]', got {} instead",
            opmvv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opmvv {
        dest: vd,
        vs2,
        vs1,
        vm,
    })
}

pub fn parse_opmvv_maskless_format(opmvv: &str) -> Result<format::Opmvv, String> {
    let tokens: Vec<&str> = opmvv.split(',').map(str::trim).collect();
    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1', got {} instead",
            opmvv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;

    Ok(format::Opmvv {
        dest: vd,
        vs2,
        vs1,
        vm: false,
    })
}

pub fn parse_opmvx_format(opmvx: &str) -> Result<format::Opmvx, String> {
    let tokens: Vec<&str> = opmvx.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, rs1, [vm]', got {} instead",
            opmvx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opmvx {
        dest: vd,
        vs2,
        rs1,
        vm,
    })
}

pub fn parse_opmvv_fma_format(opmvv: &str) -> Result<format::Opmvv, String> {
    let tokens: Vec<&str> = opmvv.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs1, vs2, [vm]', got {} instead",
            opmvv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs1 = parse_operand(tokens[1])?.as_register()?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opmvv {
        dest: vd,
        vs2,
        vs1,
        vm,
    })
}

pub fn parse_opmvx_fma_format(opmvx: &str) -> Result<format::Opmvx, String> {
    let tokens: Vec<&str> = opmvx.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, rs1, vs2, [vm]', got {} instead",
            opmvx
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[1])?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opmvx {
        dest: vd,
        vs2,
        rs1,
        vm,
    })
}

pub fn parse_vwxunary0_vmvxs_format(vwxunary0: &str) -> Result<format::Vwxunary0, String> {
    let tokens: Vec<&str> = vwxunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'rd, vs2', got {} instead",
            vwxunary0
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;

    Ok(format::Vwxunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm: false,
    })
}

pub fn parse_vwxunary0_format(vwxunary0: &str) -> Result<format::Vwxunary0, String> {
    let tokens: Vec<&str> = vwxunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, vs2, [vm]', got {} instead",
            vwxunary0
        ));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vwxunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm,
    })
}

pub fn parse_vrxunary0_format(vrxunary0: &str) -> Result<format::Vrxunary0, String> {
    let tokens: Vec<&str> = vrxunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, rs1', got {} instead",
            vrxunary0
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[1])?;

    Ok(format::Vrxunary0 {
        dest: vd,
        vs2: 0,
        rs1,
        vm: false,
    })
}

pub fn parse_vxunary0_format(vxunary0: &str) -> Result<format::Vxunary0, String> {
    let tokens: Vec<&str> = vxunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, [vm]', got {} instead",
            vxunary0
        ));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vxunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm,
    })
}

pub fn parse_vmunary0_vidv_format(vmunary0: &str) -> Result<format::Vmunary0, String> {
    let tokens: Vec<&str> = vmunary0.split(',').map(str::trim).collect();
    if tokens.len() != 1 && tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, [vm]', got {} instead",
            vmunary0
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vm = if tokens.len() == 2 {
        parse_operand(tokens[1])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vmunary0 {
        dest: vd,
        vs2: 0,
        vs1: 0,
        vm,
    })
}

pub fn parse_vmunary0_format(vmunary0: &str) -> Result<format::Vmunary0, String> {
    let tokens: Vec<&str> = vmunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, [vm]', got {} instead",
            vmunary0
        ));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vmunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm,
    })
}

pub fn parse_opfvv_format(opfvv: &str) -> Result<format::Opfvv, String> {
    let tokens: Vec<&str> = opfvv.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, vs1, [vm]', got {} instead",
            opfvv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opfvv {
        dest: vd,
        vs2,
        vs1,
        vm,
    })
}

pub fn parse_opfvf_format(opfvf: &str) -> Result<format::Opfvf, String> {
    let tokens: Vec<&str> = opfvf.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs2, rs1, [vm]', got {} instead",
            opfvf
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = float::parse_operand(tokens[2])?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opfvf { vd, vs2, rs1, vm })
}

pub fn parse_opfvv_fma_format(opfvv: &str) -> Result<format::Opfvv, String> {
    let tokens: Vec<&str> = opfvv.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, vs1, vs2, [vm]', got {} instead",
            opfvv
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs1 = parse_operand(tokens[1])?.as_register()?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opfvv {
        dest: vd,
        vs2,
        vs1,
        vm,
    })
}

pub fn parse_opfvf_fma_format(opfvf: &str) -> Result<format::Opfvf, String> {
    let tokens: Vec<&str> = opfvf.split(',').map(str::trim).collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'vd, rs1, vs2, [vm]', got {} instead",
            opfvf
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = float::parse_operand(tokens[1])?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = if tokens.len() == 4 {
        parse_operand(tokens[3])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Opfvf { vd, vs2, rs1, vm })
}

pub fn parse_vwfunary0_format(vwfunary0: &str) -> Result<format::Vwfunary0, String> {
    let tokens: Vec<&str> = vwfunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'rd, vs2', got {} instead",
            vwfunary0
        ));
    }

    let rd = float::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;

    Ok(format::Vwfunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm: false,
    })
}

pub fn parse_vrfunary0_format(vrfunary0: &str) -> Result<format::Vrfunary0, String> {
    let tokens: Vec<&str> = vrfunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'vd, rs1', got {} instead",
            vrfunary0
        ));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = float::parse_operand(tokens[1])?;

    Ok(format::Vrfunary0 {
        vd,
        vs2: 0,
        rs1,
        vm: false,
    })
}

pub fn parse_vfunary0_format(vfunary0: &str) -> Result<format::Vfunary0, String> {
    let tokens: Vec<&str> = vfunary0.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, [vm]', got {} instead",
            vfunary0
        ));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vfunary0 {
        dest: rd,
        vs2,
        vs1: 0,
        vm,
    })
}

pub fn parse_vfunary1_format(vfunary1: &str) -> Result<format::Vfunary1, String> {
    let tokens: Vec<&str> = vfunary1.split(',').map(str::trim).collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'vd, vs2, [vm]', got {} instead",
            vfunary1
        ));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = if tokens.len() == 3 {
        parse_operand(tokens[2])?.as_mask()?;
        true
    } else {
        false
    };

    Ok(format::Vfunary1 {
        dest: rd,
        vs2,
        vs1: 0,
        vm,
    })
}

#[derive(PartialEq)]
pub enum VectorOperand {
    Register(usize),
    Mask,
}

impl VectorOperand {
    fn as_register(&self) -> Result<usize, String> {
        match self {
            Self::Register(nth) => Ok(*nth),
            Self::Mask => Err("Expected register, parsed mask instead".to_owned()),
        }
    }

    fn as_mask(&self) -> Result<(), String> {
        match self {
            Self::Register(nth) => Err(format!(
                "Expected mask, parsed vector register {} instead",
                nth
            )),
            Self::Mask => Ok(()),
        }
    }
}

fn parse_operand(op: &str) -> Result<VectorOperand, String> {
    let operand = match op {
        "v0" => 0,
        "v1" => 1,
        "v2" => 2,
        "v3" => 3,
        "v4" => 4,
        "v5" => 5,
        "v6" => 6,
        "v7" => 7,
        "v8" => 8,
        "v9" => 9,
        "v10" => 10,
        "v11" => 11,
        "v12" => 12,
        "v13" => 13,
        "v14" => 14,
        "v15" => 15,
        "v16" => 16,
        "v17" => 17,
        "v18" => 18,
        "v19" => 19,
        "v20" => 20,
        "v21" => 21,
        "v22" => 22,
        "v23" => 23,
        "v24" => 24,
        "v25" => 25,
        "v26" => 26,
        "v27" => 27,
        "v28" => 28,
        "v29" => 29,
        "v30" => 30,
        "v31" => 31,
        "v0.t" => return Ok(VectorOperand::Mask),
        _ => return Err(format!("Unknown vector operand {}", op)),
    };

    Ok(VectorOperand::Register(operand))
}

pub mod pseudo {
    pub fn parse_op_format(op: &str) -> Result<usize, String> {
        let tokens: Vec<&str> = op.split(',').map(str::trim).collect();

        if tokens.len() != 1 {
            return Err(format!("Expected format: 'vreg', got {} instead", op));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;

        Ok(reg1)
    }

    pub fn parse_op_op_format(op_op: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = op_op.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2', got {} instead",
                op_op
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;

        Ok((reg1, reg2))
    }

    pub fn parse_op_op_mask_format(op_op_mask: &str) -> Result<(usize, usize, bool), String> {
        let tokens: Vec<&str> = op_op_mask.split(',').map(str::trim).collect();

        if tokens.len() != 2 && tokens.len() != 3 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, [vm]', got {} instead",
                op_op_mask
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;

        let vm = if tokens.len() == 3 {
            super::parse_operand(tokens[2])?.as_mask()?;
            true
        } else {
            false
        };

        Ok((reg1, reg2, vm))
    }

    pub fn parse_op_op_xreg_format(op_op_xreg: &str) -> Result<(usize, usize, usize), String> {
        let tokens: Vec<&str> = op_op_xreg.split(',').map(str::trim).collect();

        if tokens.len() != 3 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, xreg', got {} instead",
                op_op_xreg
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;
        let reg3 = super::integer::parse_operand(tokens[2])?;

        Ok((reg1, reg2, reg3))
    }

    pub fn parse_op_op_xreg_mask_vd_nonzero_format(
        op_op_xreg_mask_vd_nonzero: &str,
    ) -> Result<(usize, usize, usize), String> {
        let tokens: Vec<&str> = op_op_xreg_mask_vd_nonzero
            .split(',')
            .map(str::trim)
            .collect();

        if tokens.len() != 4 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, xreg, v0.t', got {} instead",
                op_op_xreg_mask_vd_nonzero
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        if reg1 == 0 {
            return Err("Expected vd != v0".to_owned());
        }

        let reg2 = super::parse_operand(tokens[1])?.as_register()?;
        let reg3 = super::integer::parse_operand(tokens[2])?;
        super::parse_operand(tokens[3])?.as_mask()?;

        Ok((reg1, reg2, reg3))
    }

    pub fn parse_op_op_xreg_mask_temp_format(
        op_op_xreg_mask: &str,
    ) -> Result<(usize, usize, usize, usize), String> {
        let tokens: Vec<&str> = op_op_xreg_mask.split(',').map(str::trim).collect();

        if tokens.len() != 5 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, xreg, v0.t, vt', got {} instead",
                op_op_xreg_mask
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;
        let xreg = super::integer::parse_operand(tokens[2])?;
        super::parse_operand(tokens[3])?.as_mask()?;
        let reg3 = super::parse_operand(tokens[4])?.as_register()?;

        Ok((reg1, reg2, xreg, reg3))
    }

    pub fn parse_op_op_op_mask_format(
        op_op_op_mask: &str,
    ) -> Result<(usize, usize, usize, bool), String> {
        let tokens: Vec<&str> = op_op_op_mask.split(',').map(str::trim).collect();

        if tokens.len() != 3 && tokens.len() != 4 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, vreg3, [vm]', got {} instead",
                op_op_op_mask
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;
        let reg3 = super::parse_operand(tokens[2])?.as_register()?;

        let vm = if tokens.len() == 4 {
            super::parse_operand(tokens[3])?.as_mask()?;
            true
        } else {
            false
        };

        Ok((reg1, reg2, reg3, vm))
    }

    pub fn parse_op_op_imm_mask_format(
        op_op_imm_mask: &str,
    ) -> Result<(usize, usize, i32, bool), String> {
        let tokens: Vec<&str> = op_op_imm_mask.split(',').map(str::trim).collect();

        if tokens.len() != 3 && tokens.len() != 4 {
            return Err(format!(
                "Expected format: 'vreg1, vreg2, imm, [vm]', got {} instead",
                op_op_imm_mask
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?.as_register()?;
        let reg2 = super::parse_operand(tokens[1])?.as_register()?;
        let reg3 = super::integer::parse_immediate(tokens[2])?;

        let vm = if tokens.len() == 4 {
            super::parse_operand(tokens[3])?.as_mask()?;
            true
        } else {
            false
        };

        Ok((reg1, reg2, reg3, vm))
    }
}
