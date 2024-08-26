//! Simulate Peppermint code on Tick Talk.
//!
//! To parse a program then run it to completion:
//! ```
//! # fn main() {
//! use peppermint_simulate::TickTalk;
//!
//! let my_program = "
//!     10
//!     STORE [0x10]
//!     5
//!     ADD [0x10]
//!     STORE [0x20]
//! ";
//!
//! let parsed = peppermint::Program::parse_source(my_program).unwrap();
//! let mut sim = TickTalk::new(&parsed, 100);
//! sim.run_to_completion().unwrap();
//!
//! assert_eq!(sim.memory[0x20], 15);
//! # }
//! ```
#![warn(clippy::pedantic)]
#![deny(missing_docs)]

use std::ops::DerefMut;

use peppermint::{Address, DoubleWord, Instruction, Program, Statement};
use thiserror::Error;

/// Simulator for Peppermint on Tick Talk.
///
/// Represents the state of a Tick Talk machine as a program runs on it.
#[derive(Clone)]
pub struct TickTalk<'a, M> {
    /// Memory of the system.
    pub memory: M,
    /// Program code that the system is executing.
    pub program: &'a Program,
    /// Program counter "register".
    pub program_counter: usize,
    /// Accumulator of the system.
    pub accumulator: peppermint::DoubleWord,
}

/// Error in simulation.
#[derive(Error, Debug)]
pub enum Error {
    /// Tried to access address outside of memory.
    #[error("tried to access address outside of memory")]
    AccessOutOfBounds,
}

impl<'a> TickTalk<'a, Vec<DoubleWord>> {
    /// Create a new simulator and load a program into it.
    ///
    /// To parse into a program, see [`peppermint::Program::parse_source`].
    pub fn new(program: &'a Program, memory_size: usize) -> Self {
        Self {
            program,
            memory: vec![0; memory_size],
            program_counter: 0,
            accumulator: 0,
        }
    }
}

impl<'a, M: DerefMut<Target = [DoubleWord]>> TickTalk<'a, M> {
    /// Create a new simulator and load a program into it with an external memory buffer.
    ///
    /// To parse into a program, see [`peppermint::Program::parse_source`].
    pub fn with_external_mem(program: &'a Program, memory: M) -> Self {
        Self {
            program,
            memory,
            program_counter: 0,
            accumulator: 0,
        }
    }

    /// Step the program by a single instruction.
    ///
    /// Returns whether the program has halted.
    pub fn step(&mut self) -> Result<bool, Error> {
        if self.halted() {
            return Ok(true);
        }

        let statement = &self.program.statements()[self.program_counter];
        self.program_counter += 1;

        match statement {
            Statement::Literal(val) => self.accumulator = *val,
            Statement::InstrLine(ins) => match ins {
                Instruction::Load(addr) => self.accumulator = self.read_address(*addr)?,
                Instruction::And(addr) => self.accumulator &= self.read_address(*addr)?,
                Instruction::Xor(addr) => self.accumulator ^= self.read_address(*addr)?,
                Instruction::Or(addr) => self.accumulator |= self.read_address(*addr)?,
                Instruction::Add(addr) => self.accumulator += self.read_address(*addr)?,
                Instruction::Sub(addr) => self.accumulator -= self.read_address(*addr)?,
                Instruction::Store(addr) => {
                    if let Some(value) = self.memory.get_mut(*addr as usize) {
                        *value = self.accumulator;
                    } else {
                        return Err(Error::AccessOutOfBounds);
                    }
                }
                Instruction::Jump(pc) => self.program_counter = *pc,
            },
            Statement::Label(_) => {}
        }

        Ok(false)
    }

    /// Run the simulator until the program exits.
    ///
    /// # Warning
    ///
    /// Infinite loops are possible in Peppermint, so this function may never terminate.
    pub fn run_to_completion(&mut self) -> Result<(), Error> {
        let mut halted = false;
        while !halted {
            halted = self.step()?;
        }

        Ok(())
    }

    /// Check if the machine is halted.
    ///
    /// The machine is halted if the program counter is outside of the instruction count.
    pub fn halted(&self) -> bool {
        self.program_counter >= self.program.statements().len()
    }

    /// Read from an address in the memory.
    fn read_address(&self, addr: Address) -> Result<DoubleWord, Error> {
        self.memory
            .get(addr as usize)
            .map(|v| *v)
            .ok_or(Error::AccessOutOfBounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_jump() {
        let source = "10
        STORE [0x00]
        JUMP :skip
        100
        ADD [0x00]
        STORE [0x00]
        skip: 1
        ADD [0x00]
        STORE [0x00]";

        let program = peppermint::Program::parse_source(source).expect("parse error");
        let mut sim = TickTalk::new(&program, 10);
        sim.run_to_completion().expect("simulation error");

        assert_eq!(sim.memory[0x00], 11);
    }
}
