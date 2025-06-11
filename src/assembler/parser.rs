use crate::types::*;

pub fn parse_line(line: &str) -> Line {
    let line = line
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_uppercase();

    // Split into label and instruction parts by first colon, if any
    let mut parts = line.split(':');
    let (label, rest) = match (parts.next(), parts.next()) {
        (Some(l), Some(r)) => (Some(l.trim().to_string()), r.trim()),
        (Some(r), None) => (None, r.trim()),
        _ => panic!("Invalid line: {}", line),
    };

    let instruction = match rest.is_empty() {
        true => None, // empty instruction for label-only lines
        false => Some(parse_instruction(rest)),
    };

    Line { label, instruction }
}

fn _parse_two<R1, R2>(args: &str, p1: fn(&str) -> R1, p2: fn(&str) -> R2) -> (R1, R2) {
    let mut parts = args.split(',').map(str::trim);
    (p1(parts.next().unwrap()), p2(parts.next().unwrap()))
}

fn parse_instruction(text: &str) -> Instruction {
    let mut tokens = text.split_whitespace();
    let opcode = tokens.next().expect("Empty instruction");
    let args = tokens.collect::<Vec<_>>().join(" ");

    let parse_rr = |ctor: fn(Register, Register) -> Instruction| {
        let (r1, r2) = _parse_two(&args, parse_register, parse_register);
        ctor(r1, r2)
    };

    let parse_rv = |ctor: fn(Register, Value) -> Instruction| {
        let (r, v) = _parse_two(&args, parse_register, parse_value);
        ctor(r, v)
    };

    match opcode {
        "NOP" => Instruction::Nop,
        "INPUT" => Instruction::Input,
        "OUTPUT" => Instruction::Output,
        "HALT" => Instruction::Halt,
        "INC" => Instruction::Inc(parse_register(args.trim())),

        "MOV" => parse_rr(Instruction::Mov),
        "ADD" => parse_rr(Instruction::Add),
        "SUB" => parse_rr(Instruction::Sub),
        "NAND" => parse_rr(Instruction::Nand),
        "OR" => parse_rr(Instruction::Or),
        "CMP" => parse_rr(Instruction::Cmp),

        "LOAD" => parse_rv(Instruction::Load),
        "STORE" => parse_rv(Instruction::Store),

        "JMP" => Instruction::Jmp(parse_value(args.trim())),
        "JZ" => Instruction::Jz(parse_value(args.trim())),

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
    match s.strip_prefix('#') {
        Some(imm) => imm.parse().map(Value::Immediate),
        None => s.parse().map(Value::Address),
    }
    .unwrap_or_else(|_| Value::Label(s.to_string()))
}
