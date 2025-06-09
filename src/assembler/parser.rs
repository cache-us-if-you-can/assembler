use crate::types::*;

pub fn parse_line(line: &str) -> Line {
    let line = {
        let line = line.split(';').next().unwrap();
        line.trim().to_uppercase()
    };

    let mut parts = line.split(':');
    let (label, rest) = match (parts.next(), parts.next()) {
        (Some(label), Some(instr)) => (Some(label.trim().to_string()), instr.trim()),
        (Some(instr), None) => (None, instr.trim()),
        _ => panic!("Invalid line: {}", line),
    };

    let instruction = Some(parse_instruction(rest));
    Line { label, instruction }
}

fn parse_instruction(text: &str) -> Instruction {
    let tokens: Vec<&str> = text.split_whitespace().collect();

    match tokens.as_slice() {
        ["NOP"] => Instruction::Nop,
        ["INPUT"] => Instruction::Input,
        ["OUTPUT"] => Instruction::Output,
        ["HALT"] => Instruction::Halt,
        ["INC", reg] => Instruction::Inc(parse_register(reg)),
        ["MOV", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Mov(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["ADD", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Add(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["JMP", addr] => Instruction::Jmp(parse_value(addr)),
        ["LOAD", args @ ..] => {
            let joined = args.join(" ");
            let parts: Vec<&str> = joined.split(',').map(str::trim).collect();
            Instruction::Load(parse_register(parts[0]), parse_value(parts[1]))
        }
        ["SUB", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Sub(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["NAND", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Nand(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["OR", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Or(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["CMP", args] => {
            let regs: Vec<&str> = args.split(',').map(str::trim).collect();
            Instruction::Cmp(parse_register(regs[0]), parse_register(regs[1]))
        }
        ["JZ", addr] => Instruction::Jz(parse_value(addr)),
        _ => panic!("Unknown instruction: {}", text),
    }
}

fn parse_register(s: &str) -> Register {
    match s {
        "A" => Register::A,
        "B" => Register::B,
        _ => panic!("Unknown register: {}", s),
    }
}

fn parse_value(s: &str) -> Value {
    if let Some(imm) = s.strip_prefix("#") {
        Value::Immediate(imm.parse().unwrap())
    } else {
        Value::Label(s.to_string())
    }
}
