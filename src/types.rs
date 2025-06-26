use std::fmt;

#[derive(Debug, Clone)]
pub enum Register {
    A,
    B,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
        use Value::*;
        match self {
            Immediate(imm) => write!(f, "#{}", imm),
            Address(addr) => write!(f, "{}", addr),
            Label(label) => write!(f, "{}", label),
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
        use Instruction::*;
        match self {
            Nop => write!(f, "NOP"),
            Input => write!(f, "INPUT"),
            Output => write!(f, "OUTPUT"),
            Halt => write!(f, "HALT"),
            Inc(r) => write!(f, "INC {}", r),
            Mov(r1, r2) => write!(f, "MOV {} {}", r1, r2),
            Add(r1, r2) => write!(f, "ADD {} {}", r1, r2),
            Jmp(v) => write!(f, "JMP {}", v),
            Load(r, v) => write!(f, "LOAD {} {}", r, v),
            Sub(r1, r2) => write!(f, "SUB {} {}", r1, r2),
            Nand(r1, r2) => write!(f, "NAND {} {}", r1, r2),
            Or(r1, r2) => write!(f, "OR {} {}", r1, r2),
            Cmp(r1, r2) => write!(f, "CMP {} {}", r1, r2),
            Jz(v) => write!(f, "JZ {}", v),
            Store(r, v) => write!(f, "STORE {} {}", r, v),
            Db(v) => write!(f, "DB {}", v),
            Equ(v) => write!(f, "EQU {}", v),
            Resb(n) => write!(f, "RESB {}", n),
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
