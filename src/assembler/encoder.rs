use crate::types::*;
use std::collections::HashMap;

pub fn build_symbol_table(lines: &[Line]) -> HashMap<String, usize> {
    lines
        .iter()
        .scan(0usize, |addr, line| {
            let current = *addr;
            if let Some(instr) = &line.instruction {
                *addr += instruction_size(instr);
            }
            Some((line, current))
        })
        .filter_map(|(line, addr)| line.label.as_ref().map(|l| (l.clone(), addr)))
        .collect()
}

fn instruction_size(instr: &Instruction) -> usize {
    match instr {
        Instruction::Load(_, _) => 2,
        Instruction::Jmp(_) => 2,
        Instruction::Jz(_) => 2,
        Instruction::Store(_, _) => 2,
        _ => 1,
    }
}

pub fn assemble_instruction(instr: &Instruction, symbols: &HashMap<String, usize>) -> Vec<u8> {
    match instr {
        Instruction::Nop => vec![0x00],
        Instruction::Input => vec![0x04],
        Instruction::Output => vec![0x05],
        Instruction::Jmp(Value::Label(label)) => {
            let addr = symbols.get(label).expect("Undefined label");
            vec![0x06, *addr as u8]
        }
        Instruction::Load(Register::A, Value::Immediate(v)) => vec![0x09, *v],
        Instruction::Inc(Register::A) => vec![0x0c],
        Instruction::Mov(Register::B, Register::A) => vec![0x0d],
        Instruction::Add(Register::A, Register::B) => vec![0x0e],
        Instruction::Halt => vec![0x0f],
        Instruction::Inc(Register::B) => vec![0x10],
        Instruction::Mov(Register::A, Register::B) => vec![0x11],
        Instruction::Sub(Register::A, Register::B) => vec![0x12],
        Instruction::Nand(Register::A, Register::B) => vec![0x13],
        Instruction::Or(Register::A, Register::B) => vec![0x14],
        Instruction::Cmp(Register::A, Register::B) => vec![0x15],
        Instruction::Jz(Value::Label(label)) => {
            let addr = symbols.get(label).expect("Undefined label");
            vec![0x16, *addr as u8]
        }
        Instruction::Store(Register::A, Value::Address(v)) => vec![0x1a, *v],
        Instruction::Load(Register::A, Value::Address(v)) => vec![0x1d, *v],
        Instruction::Load(Register::A, Value::Label(label)) => {
            let addr = symbols.get(label).expect("Undefined label");
            vec![0x1d, *addr as u8]
        }
        Instruction::Db(Value::Immediate(v)) => vec![*v],
        Instruction::Equ(Value::Immediate(_)) => vec![],
        _ => panic!("Unsupported instruction format: {:?}", instr),
    }
}
