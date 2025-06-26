use crate::types::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Undefined label: {1} on line {0}")]
    UndefinedLabel(usize, String),
    #[error("Unsupported instruction: {1} on line {0}")]
    UnsupportedInstruction(usize, Instruction),
}

pub fn build_symbol_table(lines: &[Line]) -> HashMap<String, usize> {
    lines
        .iter()
        .scan(0usize, |addr, line| {
            let label = line.label.clone().map(|l| (l, *addr));
            if let Some(instr) = &line.instruction {
                *addr += instr.size();
            }
            Some(label)
        })
        .flatten()
        .collect()
}

impl Instruction {
    pub fn size(&self) -> usize {
        use Instruction::*;
        match self {
            Load(_, _) => 2,
            Jmp(_) => 2,
            Jz(_) => 2,
            Store(_, _) => 2,
            _ => 1,
        }
    }
    pub fn encode(
        &self,
        index: usize,
        symbols: &HashMap<String, usize>,
    ) -> Result<Vec<u8>, EncodeError> {
        use Instruction::*;
        let resolve = |label: &str| {
            symbols
                .get(label)
                .copied()
                .ok_or(EncodeError::UndefinedLabel(index, label.to_string()))
        };
        let bytes = match self {
            Nop => vec![0x00],
            Input => vec![0x04],
            Output => vec![0x05],
            Jmp(Value::Label(label)) => vec![0x06, resolve(label)? as u8],
            Load(Register::A, val) => match val {
                Value::Immediate(v) => vec![0x09, *v],
                Value::Address(v) => vec![0x1d, *v],
                Value::Label(label) => vec![0x1d, resolve(label)? as u8],
            },
            Inc(reg) => match reg {
                Register::A => vec![0x0c],
                Register::B => vec![0x10],
            },
            Mov(Register::B, Register::A) => vec![0x0d],
            Mov(Register::A, Register::B) => vec![0x11],

            Add(Register::A, Register::B) => vec![0x0e],
            Sub(Register::A, Register::B) => vec![0x12],
            Nand(Register::A, Register::B) => vec![0x13],
            Or(Register::A, Register::B) => vec![0x14],
            Cmp(Register::A, Register::B) => vec![0x15],

            Jz(Value::Label(label)) => vec![0x16, resolve(label)? as u8],
            Store(Register::A, val) => match val {
                Value::Address(v) => vec![0x1a, *v],
                Value::Label(label) => vec![0x1a, resolve(label)? as u8],
                _ => return Err(EncodeError::UnsupportedInstruction(index, self.clone())),
            },

            Halt => vec![0x0f],

            Db(Value::Immediate(v)) => vec![*v],
            Equ(Value::Immediate(_)) => vec![],
            Resb(n) => vec![0x00; *n as usize],

            _ => return Err(EncodeError::UnsupportedInstruction(index, self.clone())),
        };
        Ok(bytes)
    }
}
