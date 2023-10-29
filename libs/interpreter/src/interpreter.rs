mod decoder;

use eeric_core::prelude::*;
use std::collections::HashMap;

use decoder::{Decoder, LineClassification};

use self::decoder::{AssemblerDirective, Section};

pub struct Interpreter;

pub struct CompilationResult {
    pub instructions: Vec<Instruction>,
    pub instructions_addresses: Vec<usize>,
    pub memory: Memory,
}

impl Interpreter {
    pub fn compile(
        program: String,
        memory_size: usize,
    ) -> Result<CompilationResult, HashMap<usize, String>> {
        let mut instruction_labels = HashMap::new();
        let mut memory_labels = HashMap::new();
        let mut instructions = Vec::new();
        let mut lines_addresses = Vec::new();
        let mut program_line_address = 0;
        let mut memory_data_address = 0;

        let mut to_decode = Vec::new();
        let mut errors = HashMap::new();

        let mut lines_sections = Vec::new();
        let mut section_ctx = Section::Text;

        let mut constants = Vec::<Vec<u8>>::new();

        for (line_address, line) in program.lines().enumerate() {
            let result = Decoder::classify(line);

            match result {
                Ok(class) => match class {
                    LineClassification::PreprocDirective(_) => todo!(),
                    LineClassification::AssemblerDirective(AssemblerDirective::Section(
                        section,
                    )) => {
                        section_ctx = section;
                    }
                    LineClassification::AssemblerDirective(AssemblerDirective::Data(data)) => {
                        let vec: Vec<u8> = data.into();
                        memory_data_address += vec.len();
                        constants.push(vec);
                    }
                    LineClassification::Instruction(decodable) => {
                        program_line_address += 4;
                        to_decode.push(decodable);
                        lines_addresses.push(line_address);
                        lines_sections.push(section_ctx.clone());
                    }
                    LineClassification::Label(label) => match section_ctx {
                        Section::Text => {
                            instruction_labels.insert(label, program_line_address);
                        }
                        Section::Data => {
                            memory_labels.insert(label, memory_data_address);
                        }
                    },
                    LineClassification::Empty => {}
                },
                Err(msg) => {
                    errors.insert(line_address, msg);
                }
            }
        }

        for (decodable_line_index, decodable_line) in to_decode.into_iter().enumerate() {
            let maybe_instruction = Decoder::decode_text_section(
                &decodable_line,
                &instruction_labels,
                &memory_labels,
                decodable_line_index * 4,
            );

            match maybe_instruction {
                Ok(instruction) => instructions.push(instruction),
                Err(msg) => {
                    errors.insert(lines_addresses[decodable_line_index], msg);
                }
            };
        }

        if errors.is_empty() {
            let mut memory = Memory::new((0..memory_size).map(|_| 0));

            for constant in constants {
                memory.assign(&constant);
            }

            Ok(CompilationResult {
                instructions,
                instructions_addresses: lines_addresses,
                memory,
            })
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use eeric_core::fuse;

    use super::*;

    #[test]
    fn instruction_map_calculation() {
        let input = r#"
        .text
        addi x1, x0, 123
        loop:
        inner_loop:
            add x1, x1, x1
            bnez x1, loop
            ld a0, .hw1(x0)
            ld a0, .hw2(x0)
            ld a0, .hw3(x0)
            la a1, .hw2
        .data
        .hw1:
            .float 3.141
        .hw2:
            .string "abc"
        .hw3:
            .asciz "def"
        "#
        .trim_start();

        let compilation_result = Interpreter::compile(input.to_owned(), 12).unwrap();

        assert_eq!(
            compilation_result.instructions,
            vec![
                Instruction::Addi(format::I {
                    rd: 1,
                    rs1: 0,
                    imm12: 123
                }),
                Instruction::Add(format::R {
                    rd: 1,
                    rs1: 1,
                    rs2: 1
                }),
                Instruction::Bne(format::S {
                    rs1: 1,
                    rs2: 0,
                    imm12: -4
                }),
                Instruction::Ld(format::I {
                    rd: 10,
                    rs1: 0,
                    imm12: 0
                }),
                Instruction::Ld(format::I {
                    rd: 10,
                    rs1: 0,
                    imm12: 4
                }),
                Instruction::Ld(format::I {
                    rd: 10,
                    rs1: 0,
                    imm12: 7
                }),
                fuse![
                    Instruction::Auipc(format::U { rd: 11, imm20: 0 }),
                    Instruction::Addi(format::I {
                        rd: 11,
                        rs1: 11,
                        imm12: 4
                    })
                ]
            ]
        );

        assert_eq!(
            compilation_result.instructions_addresses,
            vec![1, 4, 5, 6, 7, 8, 9]
        );

        assert_eq!(
            compilation_result
                .memory
                .snapshot()
                .into_iter()
                .collect::<Vec<_>>(),
            vec![37, 6, 73, 64, 97, 98, 99, 100, 101, 102, 0, 0]
        );
    }
}
