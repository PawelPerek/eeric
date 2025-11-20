pub mod arbitrary_float;
pub mod instruction;
pub mod memory;
pub mod registers;
pub mod snapshot;
pub mod vector_engine;

use derive_builder::Builder;

use instruction::{executor::Executor, Instruction};
use memory::Memory;
use registers::Registers;

use self::vector_engine::VectorEngine;

#[derive(Builder, Clone, PartialEq, Debug)]
#[builder(build_fn(skip))]
pub struct RvCore {
    pub memory: Memory,
    pub instructions: Vec<Instruction>,
    #[builder(setter(skip))]
    pub registers: Registers,
    pub vec_engine: VectorEngine,
}

impl RvCore {
    pub fn step(&mut self) -> Option<Result<(), String>> {
        self.run().next()
    }

    pub fn run(&mut self) -> RunningRvCore<'_> {
        RunningRvCore { core: self }
    }
}

impl Default for RvCore {
    fn default() -> Self {
        let vec_engine = VectorEngine::default();
        let memory = Memory::default();

        Self {
            instructions: Vec::new(),
            registers: Registers::new(&vec_engine, &memory),
            memory,
            vec_engine,
        }
    }
}

impl RvCoreBuilder {
    pub fn build(&self) -> RvCore {
        let memory = self.memory.clone().unwrap_or_default();
        let instructions = self.instructions.clone().unwrap_or_default();
        let vec_engine = self.vec_engine.unwrap_or_default();
        let registers = Registers::new(&vec_engine, &memory);

        RvCore {
            memory,
            instructions,
            vec_engine,
            registers,
        }
    }
}

pub struct RunningRvCore<'core> {
    core: &'core mut RvCore,
}

impl Iterator for RunningRvCore<'_> {
    type Item = Result<(), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction_pointer = self.core.registers.pc / 4;
        let instruction = self
            .core
            .instructions
            .get(instruction_pointer as usize)?
            .clone();

        Some(
            Executor::new(
                &mut self.core.registers,
                &mut self.core.memory,
                &mut self.core.vec_engine,
            )
            .execute(instruction),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::rv_core::{
        registers::aliases::{csr::VLENB, integer::SP},
        snapshot::Snapshotable,
        vector_engine::{VectorEngineBuilder, Vlen},
    };

    use super::*;

    #[test]
    fn default_has_vector_registers() {
        let core = RvCore::default();
        assert_eq!(
            core.registers.snapshot().v.len(),
            32 * Vlen::V128.byte_length()
        );
    }

    #[test]
    fn custom_vlen_works() {
        use Instruction::*;

        let core = RvCoreBuilder::default()
            .instructions(vec![Vsetvli(instruction::format::Vsetvli {
                rd: 5,
                rs1: 12,
                vtypei: 195,
            })])
            .vec_engine(VectorEngineBuilder::default().vlen(Vlen::V256).build())
            .build();
        assert_eq!(
            core.registers.snapshot().v.len(),
            32 * Vlen::V256.byte_length()
        );
    }

    #[test]
    fn vlenb_csr_works() {
        let core = RvCoreBuilder::default()
            .vec_engine(VectorEngineBuilder::default().vlen(Vlen::V256).build())
            .build();
        assert_eq!(
            core.registers.snapshot().c[VLENB].read(),
            Vlen::V256.byte_length() as u64
        );
    }

    #[test]
    fn sp_points_to_stack() {
        let memory = Memory::new([5, 2, 1, 3, 4].into_iter());

        let core = RvCoreBuilder::default().memory(memory).build();

        assert_eq!(core.registers.snapshot().x[SP], 4);
    }
}
