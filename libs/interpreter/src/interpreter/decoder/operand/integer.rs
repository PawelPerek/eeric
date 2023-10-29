use std::collections::HashMap;

use eeric_core::prelude::*;

pub fn parse_r_format(r: &str) -> Result<format::R, String> {
    let tokens: Vec<&str> = r.split(',').map(str::trim).collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, rs1, rs2', got {} instead",
            r
        ));
    }

    let rd = parse_operand(tokens[0])?;
    let rs1 = parse_operand(tokens[1])?;
    let rs2 = parse_operand(tokens[2])?;

    Ok(format::R { rd, rs1, rs2 })
}

pub fn parse_i_format(i: &str) -> Result<format::I, String> {
    let tokens: Vec<&str> = i.split(',').map(str::trim).collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, rs1, imm', got {} instead",
            i
        ));
    }

    let rd = parse_operand(tokens[0])?;
    let rs1 = parse_operand(tokens[1])?;
    let imm = parse_immediate(tokens[2])?;

    Ok(format::I {
        rd,
        rs1,
        imm12: imm,
    })
}

pub fn parse_load_format(
    i: &str,
    memory_labels: &HashMap<String, usize>,
) -> Result<format::I, String> {
    let tokens: Vec<&str> = i.split(',').map(str::trim).collect();

    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'rd, imm(rs1)', got {} instead",
            i
        ));
    }

    let rd = parse_operand(tokens[0])?;
    let (imm, rs1) = parse_offset_addr_operand(tokens[1], memory_labels)?;

    Ok(format::I {
        rd,
        rs1,
        imm12: imm,
    })
}

pub fn parse_s_format(
    s: &str,
    memory_labels: &HashMap<String, usize>,
) -> Result<format::S, String> {
    let tokens: Vec<&str> = s.split(',').map(str::trim).collect();

    if tokens.len() != 2 {
        return Err(format!(
            "Expected format: 'rs2, imm(rs1)', got {} instead",
            s
        ));
    }

    let rs2 = parse_operand(tokens[0])?;
    let (imm, rs1) = parse_offset_addr_operand(tokens[1], memory_labels)?;

    Ok(format::S {
        rs1,
        rs2,
        imm12: imm,
    })
}

pub fn parse_branch_format(
    s: &str,
    labels: &HashMap<String, usize>,
    current_line: usize,
) -> Result<format::S, String> {
    let tokens: Vec<&str> = s.split(',').map(str::trim).collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rs1, rs2, label', got {} instead",
            s
        ));
    }

    let rs1 = parse_operand(tokens[0])?;
    let rs2 = parse_operand(tokens[1])?;
    let label_addr = parse_instruction_label(tokens[2], labels, current_line)?;

    Ok(format::S {
        rs1,
        rs2,
        imm12: label_addr,
    })
}

pub fn parse_u_format(u: &str) -> Result<format::U, String> {
    let tokens: Vec<&str> = u.split(',').map(str::trim).collect();

    if tokens.len() != 2 {
        return Err(format!("Expected format: 'rd, imm', got {} instead", u));
    }

    let rd = parse_operand(tokens[0])?;
    let imm = parse_immediate(tokens[1])?;

    Ok(format::U { rd, imm20: imm })
}

pub fn parse_offset_addr_operand(
    op: &str,
    memory_labels: &HashMap<String, usize>,
) -> Result<(i32, usize), String> {
    let Some(operand_addr) = op.find('(') else {
        return Err(format!(
            "Expected format: 'imm(rs1)' for the address with offset, got {} instead",
            op
        ));
    };

    let (imm, reg) = op.split_at(operand_addr);

    let imm = parse_immediate_or_memory_label(imm, memory_labels)?;
    let reg = parse_addr_operand(reg)?;

    Ok((imm, reg))
}

pub fn parse_addr_operand(op: &str) -> Result<usize, String> {
    if op.starts_with('(') && op.ends_with(')') {
        let inner_op = &op[1..op.len() - 1];
        parse_operand(inner_op)
    } else {
        Err(format!(
            "Address operand {} is not wrapped in parentheses",
            op
        ))
    }
}

pub fn parse_operand(op: &str) -> Result<usize, String> {
    let operand = match op {
        "x0" | "zero" => 0,
        "x1" | "ra" => 1,
        "x2" | "sp" => 2,
        "x3" | "gp" => 3,
        "x4" | "tp" => 4,
        "x5" | "t0" => 5,
        "x6" | "t1" => 6,
        "x7" | "t2" => 7,
        "x8" | "s0" | "fp" => 8,
        "x9" | "s1" => 9,
        "x10" | "a0" => 10,
        "x11" | "a1" => 11,
        "x12" | "a2" => 12,
        "x13" | "a3" => 13,
        "x14" | "a4" => 14,
        "x15" | "a5" => 15,
        "x16" | "a6" => 16,
        "x17" | "a7" => 17,
        "x18" | "s2" => 18,
        "x19" | "s3" => 19,
        "x20" | "s4" => 20,
        "x21" | "s5" => 21,
        "x22" | "s6" => 22,
        "x23" | "s7" => 23,
        "x24" | "s8" => 24,
        "x25" | "s9" => 25,
        "x26" | "s10" => 26,
        "x27" | "s11" => 27,
        "x28" | "t3" => 28,
        "x29" | "t4" => 29,
        "x30" | "t5" => 30,
        "x31" | "t6" => 31,
        _ => return Err(format!("Incorrect integer operand: {}", op)),
    };

    Ok(operand)
}

pub fn parse_immediate(imm: &str) -> Result<i32, String> {
    if imm.starts_with("0x") || imm.starts_with("0X") {
        i32::from_str_radix(&imm[2..], 16)
            .map_err(|e| format!("Error parsing hexadecimal immediate: {}", e))
    } else if imm.starts_with("0o") || imm.starts_with("0O") {
        i32::from_str_radix(&imm[2..], 8)
            .map_err(|e| format!("Error parsing octal immediate: {}", e))
    } else if imm.starts_with("0b") || imm.starts_with("0B") {
        i32::from_str_radix(&imm[2..], 2)
            .map_err(|e| format!("Error parsing binary immediate: {}", e))
    } else {
        imm.parse::<i32>()
            .map_err(|e| format!("Error parsing immediate: {}", e))
    }
}

pub fn parse_immediate_or_memory_label(
    imm_or_mem: &str,
    memory_labels: &HashMap<String, usize>,
) -> Result<i32, String> {
    if imm_or_mem.is_empty() {
        return Ok(0);
    }

    match parse_immediate(imm_or_mem) {
        Ok(imm) => Ok(imm),
        Err(imm_err) => parse_memory_label(imm_or_mem, memory_labels)
            .map_err(|addr_err| format!("{} or {}", imm_err, addr_err)),
    }
}

pub fn parse_memory_label(label: &str, map: &HashMap<String, usize>) -> Result<i32, String> {
    map.get(label)
        .cloned()
        .map(|addr| addr as i32)
        .ok_or(format!("Did not find memory label {}", label))
}

pub fn parse_instruction_label(
    label: &str,
    map: &HashMap<String, usize>,
    current_line: usize,
) -> Result<i32, String> {
    map.get(label)
        .cloned()
        .map(|addr| addr.wrapping_sub(current_line) as i32)
        .ok_or(format!("Did not find instruction label {}", label))
}

pub mod pseudo {
    use std::collections::HashMap;

    pub fn parse_imm_format(imm: &str) -> Result<i32, String> {
        let tokens: Vec<&str> = imm.split(',').map(str::trim).collect();

        if tokens.len() != 1 {
            return Err(format!("Expected format: 'imm', got {} instead", imm));
        }

        let imm = super::parse_immediate(tokens[0])?;

        Ok(imm)
    }

    pub fn parse_op_imm_format(op_imm: &str) -> Result<(usize, i32), String> {
        let tokens: Vec<&str> = op_imm.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg, imm', got {} instead",
                op_imm
            ));
        }

        let reg = super::parse_operand(tokens[0])?;
        let imm = super::parse_immediate(tokens[1])?;

        Ok((reg, imm))
    }

    pub fn parse_op_format(op: &str) -> Result<usize, String> {
        let tokens: Vec<&str> = op.split(',').map(str::trim).collect();

        if tokens.len() != 1 {
            return Err(format!("Expected format: 'xreg', got {} instead", op));
        }

        let xreg = super::parse_operand(tokens[0])?;

        Ok(xreg)
    }

    pub fn parse_op_op_format(op_op: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = op_op.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg1, xreg2', got {} instead",
                op_op
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?;
        let reg2 = super::parse_operand(tokens[1])?;

        Ok((reg1, reg2))
    }

    pub fn parse_label_format(
        label: &str,
        labels: &HashMap<String, usize>,
        current_line: usize,
    ) -> Result<i32, String> {
        let tokens: Vec<&str> = label.split(',').map(str::trim).collect();

        if tokens.len() != 1 {
            return Err(format!("Expected format: 'label', got {} instead", label));
        }

        let diff = super::parse_instruction_label(tokens[0], labels, current_line)?;

        Ok(diff)
    }

    pub fn parse_op_label_format(
        op_label: &str,
        labels: &HashMap<String, usize>,
        current_line: usize,
    ) -> Result<(usize, i32), String> {
        let tokens: Vec<&str> = op_label.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg, label', got {} instead",
                op_label
            ));
        }

        let reg = super::parse_operand(tokens[0])?;
        let diff = super::parse_instruction_label(tokens[1], labels, current_line)?;

        Ok((reg, diff))
    }

    pub fn parse_op_memory_label_format(
        op_label: &str,
        memory_labels: &HashMap<String, usize>,
    ) -> Result<(usize, i32), String> {
        let tokens: Vec<&str> = op_label.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg, memory_label', got {} instead",
                op_label
            ));
        }

        let reg = super::parse_operand(tokens[0])?;
        let diff = super::parse_memory_label(tokens[1], memory_labels)?;

        Ok((reg, diff))
    }

    pub fn parse_op_op_label_format(
        op_op_label: &str,
        labels: &HashMap<String, usize>,
        current_line: usize,
    ) -> Result<(usize, usize, i32), String> {
        let tokens: Vec<&str> = op_op_label.split(',').map(str::trim).collect();

        if tokens.len() != 3 {
            return Err(format!(
                "Expected format: 'xreg1, xreg2, label', got {} instead",
                op_op_label
            ));
        }

        let xreg1 = super::parse_operand(tokens[0])?;
        let xreg2 = super::parse_operand(tokens[1])?;
        let diff = super::parse_instruction_label(tokens[2], labels, current_line)?;

        Ok((xreg1, xreg2, diff))
    }
}
