#[derive(Debug, Clone)]
pub enum Register {
    A,
    B,
}

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(u8), // like #42
    Address(u8),   // like 08
    Label(String), // like start:
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
    pub label: Option<String>,
    pub instruction: Option<Instruction>,
}
