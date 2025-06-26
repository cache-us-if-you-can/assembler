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

impl Line {
    pub fn parse((line, index): (&str, usize)) -> Result<Line, ParseError> {
        // Remove comments and trim whitespace
        let line = line
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_uppercase();

        // If line is empty after trimming, return an empty line without label or instruction
        if line.is_empty() {
            return Ok(Line {
                index,
                label: None,
                instruction: None,
            });
        }

        // Find first colon to split label and instruction
        if let Some(colon_pos) = line.find(':') {
            let (label_part, rest_part) = line.split_at(colon_pos);
            let label = label_part.trim();

            // If label is empty, treat it as no label
            if label.is_empty() {
                return Err(ParseError::InvalidLine(index, line));
            }

            let rest = rest_part[1..].trim();
            let instruction = if rest.is_empty() {
                None
            } else {
                Some(Instruction::parse(index, rest)?)
            };

            return Ok(Line {
                index,
                label: Some(label.to_string()),
                instruction,
            });
        }
        // No colon, so whole line is instruction without label
        let instruction = Instruction::parse(index, &line)?;
        Ok(Line {
            index,
            label: None,
            instruction: Some(instruction),
        })
    }
}

impl Instruction {
    fn parse(i: usize, text: &str) -> Result<Instruction, ParseError> {
        let mut tokens = text.split_whitespace();
        let opcode = tokens.next().expect("Empty instruction");
        let args = tokens.collect::<Vec<_>>().join(" ");

        let parse_rr = |ctor: fn(Register, Register) -> Instruction| {
            let (a, b) = args
                .split_once(',')
                .ok_or(ParseError::InvalidInstruction(i, text.to_string()))?;
            let r1 = Register::parse(i, a.trim())?;
            let r2 = Register::parse(i, b.trim())?;
            Ok(ctor(r1, r2))
        };

        let parse_rv =
            |ctor: fn(Register, Value) -> Instruction| -> Result<Instruction, ParseError> {
                let (a, b) = args
                    .split_once(',')
                    .ok_or(ParseError::InvalidInstruction(i, text.to_string()))?;
                let r = Register::parse(i, a.trim())?;
                let v = Value::parse(b.trim());
                Ok(ctor(r, v))
            };

        use Instruction::*;
        match opcode {
            "NOP" => Ok(Nop),
            "INPUT" => Ok(Input),
            "OUTPUT" => Ok(Output),
            "HALT" => Ok(Halt),
            "INC" => Ok(Inc(Register::parse(i, &args)?)),
            "MOV" => parse_rr(Mov),
            "ADD" => parse_rr(Add),
            "SUB" => parse_rr(Sub),
            "NAND" => parse_rr(Nand),
            "OR" => parse_rr(Or),
            "CMP" => parse_rr(Cmp),
            "LOAD" => parse_rv(Load),
            "STORE" => parse_rv(Store),
            "JMP" => Ok(Jmp(Value::parse(&args))),
            "JZ" => Ok(Jz(Value::parse(&args))),
            "DB" => Ok(Db(Value::parse(&args))),
            "EQU" => Ok(Equ(Value::parse(&args))),
            "RESB" => Ok(Resb(parse_u8(i, &args)?)),
            _ => Err(ParseError::InvalidInstruction(i, text.to_string())),
        }
    }
}

impl Register {
    fn parse(i: usize, s: &str) -> Result<Register, ParseError> {
        match s {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            _ => Err(ParseError::InvalidRegister(i, s.to_string())),
        }
    }
}

impl Value {
    fn parse(s: &str) -> Value {
        match s.strip_prefix('#') {
            Some(imm) => imm.parse().map(Value::Immediate),
            None => s.parse().map(Value::Address),
        }
        .unwrap_or_else(|_| Value::Label(s.to_string()))
    }
}

fn parse_u8(i: usize, s: &str) -> Result<u8, ParseError> {
    s.trim()
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidU8(i, s.to_string()))
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
            ..*line
        })
        .collect();

    Ok(replaced_lines)
}
