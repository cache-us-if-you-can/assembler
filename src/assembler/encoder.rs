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
        Instruction::Nop
        | Instruction::Input
        | Instruction::Output
        | Instruction::Halt
        | Instruction::Inc(_)
        | Instruction::Mov(_, _)
        | Instruction::Add(_, _) => 1,

        Instruction::Load(_, _) => 2,
        Instruction::Jmp(_) => 2,
        // Extend as needed...
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
        _ => panic!("Unsupported instruction format: {:?}", instr),
    }
}
