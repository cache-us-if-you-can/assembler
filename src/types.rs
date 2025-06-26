use std::fmt;

#[derive(Debug, Clone)]
pub enum Register {
    A,
    B,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(u8), // like #42
    Address(u8),   // like 08
    Label(String), // like start:
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Immediate(imm) => write!(f, "#{}", imm),
            Value::Address(addr) => write!(f, "{}", addr),
            Value::Label(label) => write!(f, "{}", label),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    Input,
    Output,
    Halt,
    Inc(Register),
    Mov(Register, Register),
    Add(Register, Register),
    Jmp(Value),
    Load(Register, Value),
    Sub(Register, Register),
    Nand(Register, Register),
    Or(Register, Register),
    Cmp(Register, Register),
    Jz(Value),
    Store(Register, Value),
    Db(Value),
    Equ(Value),
    Resb(u8),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Nop => write!(f, "NOP"),
            Instruction::Input => write!(f, "INPUT"),
            Instruction::Output => write!(f, "OUTPUT"),
            Instruction::Halt => write!(f, "HALT"),
            Instruction::Inc(r) => write!(f, "INC {}", r),
            Instruction::Mov(r1, r2) => write!(f, "MOV {} {}", r1, r2),
            Instruction::Add(r1, r2) => write!(f, "ADD {} {}", r1, r2),
            Instruction::Jmp(v) => write!(f, "JMP {}", v),
            Instruction::Load(r, v) => write!(f, "LOAD {} {}", r, v),
            Instruction::Sub(r1, r2) => write!(f, "SUB {} {}", r1, r2),
            Instruction::Nand(r1, r2) => write!(f, "NAND {} {}", r1, r2),
            Instruction::Or(r1, r2) => write!(f, "OR {} {}", r1, r2),
            Instruction::Cmp(r1, r2) => write!(f, "CMP {} {}", r1, r2),
            Instruction::Jz(v) => write!(f, "JZ {}", v),
            Instruction::Store(r, v) => write!(f, "STORE {} {}", r, v),
            Instruction::Db(v) => write!(f, "DB {}", v),
            Instruction::Equ(v) => write!(f, "EQU {}", v),
            Instruction::Resb(n) => write!(f, "RESB {}", n),
        }
    }
}

impl Instruction {
    pub fn map_values<F>(&self, mut f: F) -> Instruction
    where
        F: FnMut(&Value) -> Value,
    {
        use Instruction::*;
        match self {
            Load(r, v) => Load(r.clone(), f(v)),
            Store(r, v) => Store(r.clone(), f(v)),
            Jmp(v) => Jmp(f(v)),
            Jz(v) => Jz(f(v)),
            Db(v) => Db(f(v)),
            Equ(v) => Equ(f(v)),
            // Instructions that don't have values
            _ => self.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub index: usize,
    pub label: Option<String>,
    pub instruction: Option<Instruction>,
}
