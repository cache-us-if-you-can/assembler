use crate::types::*;
use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid line: {0}: {1}")]
    InvalidLine(usize, String),
    #[error("Invalid instruction: {1}, line {0}")]
    InvalidInstruction(usize, String),
    #[error("Invalid register: {1} in line {0}")]
    InvalidRegister(usize, String),
    #[error("Invalid u8: {1} in line {0}")]
    InvalidU8(usize, String),
    #[error("Cyclic reference: {0}")]
    CyclicReference(Value),
}

pub fn parse_line((index, line): (usize, &str)) -> Result<Line, ParseError> {
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
        _ => return Err(ParseError::InvalidLine(index, line.to_string())),
    };

    let instruction = if rest.is_empty() {
        None
    } else {
        Some(parse_instruction(index, rest)?)
    };

    Ok(Line { label, instruction })
}

fn parse_two<R1, R2, F1, F2>(args: &str, p1: F1, p2: F2) -> (R1, R2)
where
    F1: Fn(&str) -> R1,
    F2: Fn(&str) -> R2,
{
    let mut parts = args.split(',').map(str::trim);
    (p1(parts.next().unwrap()), p2(parts.next().unwrap()))
}

fn parse_instruction(i: usize, text: &str) -> Result<Instruction, ParseError> {
    let mut tokens = text.split_whitespace();
    let opcode = tokens.next().expect("Empty instruction");
    let args = tokens.collect::<Vec<_>>().join(" ");

    let parse_rr = |ctor: fn(Register, Register) -> Instruction| {
        let (r1, r2) = parse_two(&args, |s| parse_register(i, s), |s| parse_register(i, s));
        Ok(ctor(r1?, r2?))
    };

    let parse_rv = |ctor: fn(Register, Value) -> Instruction| -> Result<Instruction, ParseError> {
        let (r, v) = parse_two(&args, |s| parse_register(i, s), parse_value);
        Ok(ctor(r?, v))
    };

    match opcode {
        "NOP" => Ok(Instruction::Nop),
        "INPUT" => Ok(Instruction::Input),
        "OUTPUT" => Ok(Instruction::Output),
        "HALT" => Ok(Instruction::Halt),
        "INC" => Ok(Instruction::Inc(parse_register(i, &args)?)),
        "MOV" => parse_rr(Instruction::Mov),
        "ADD" => parse_rr(Instruction::Add),
        "SUB" => parse_rr(Instruction::Sub),
        "NAND" => parse_rr(Instruction::Nand),
        "OR" => parse_rr(Instruction::Or),
        "CMP" => parse_rr(Instruction::Cmp),
        "LOAD" => parse_rv(Instruction::Load),
        "STORE" => parse_rv(Instruction::Store),
        "JMP" => Ok(Instruction::Jmp(parse_value(&args))),
        "JZ" => Ok(Instruction::Jz(parse_value(&args))),
        "DB" => Ok(Instruction::Db(parse_value(&args))),
        "EQU" => Ok(Instruction::Equ(parse_value(&args))),
        "RESB" => Ok(Instruction::Resb(parse_u8(i, &args)?)),
        _ => Err(ParseError::InvalidInstruction(i, text.to_string())),
    }
}

fn parse_register(i: usize, s: &str) -> Result<Register, ParseError> {
    match s {
        "A" => Ok(Register::A),
        "B" => Ok(Register::B),
        _ => Err(ParseError::InvalidRegister(i, s.to_string())),
    }
}

fn parse_u8(i: usize, s: &str) -> Result<u8, ParseError> {
    s.trim()
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidU8(i, s.to_string()))
}

fn parse_value(s: &str) -> Value {
    match s.strip_prefix('#') {
        Some(imm) => imm.parse().map(Value::Immediate),
        None => s.parse().map(Value::Address),
    }
    .unwrap_or_else(|_| Value::Label(s.to_string()))
}

pub fn replace_constants(lines: &[Line]) -> Result<Vec<Line>, ParseError> {
    let consts: HashMap<String, Value> = lines
        .iter()
        .filter_map(|line| match (&line.label, &line.instruction) {
            (Some(name), Some(Instruction::Equ(val))) => Some((name.clone(), val.clone())),
            _ => None,
        })
        .collect();

    fn resolve(
        val: &Value,
        consts: &HashMap<String, Value>,
        visited: &mut HashSet<String>,
    ) -> Result<Value, ParseError> {
        match val {
            Value::Label(name) => {
                if !visited.insert(name.clone()) {
                    return Err(ParseError::CyclicReference(val.clone()));
                }
                match consts.get(name) {
                    Some(inner) => resolve(inner, consts, visited),
                    None => Ok(Value::Label(name.clone())),
                }
            }
            other => Ok(other.clone()),
        }
    }

    let resolved_consts: HashMap<String, Value> = consts
        .iter()
        .map(|(k, v)| {
            let mut visited = HashSet::new();
            resolve(v, &consts, &mut visited).map(|resolved| (k.clone(), resolved))
        })
        .collect::<Result<_, _>>()?;

    fn replace_val(val: &Value, consts: &HashMap<String, Value>) -> Value {
        match val {
            Value::Label(name) => consts
                .get(name)
                .cloned()
                .unwrap_or(Value::Label(name.clone())),
            v => v.clone(),
        }
    }

    let replaced_lines = lines
        .iter()
        .map(|line| Line {
            label: line.label.clone(),
            instruction: line
                .instruction
                .as_ref()
                .map(|instr| instr.map_values(|v| replace_val(v, &resolved_consts))),
        })
        .collect();

    Ok(replaced_lines)
}
