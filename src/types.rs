#[derive(Debug)]
pub enum Register {
    A,
    B,
}

#[derive(Debug)]
pub enum Value {
    Immediate(u8), // like #42
    Label(String), // like start:
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Line {
    pub label: Option<String>,
    pub instruction: Option<Instruction>,
}
