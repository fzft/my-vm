pub enum Token {
    OpCode(OpCode),
    EvaValue(EvaValue),
}

pub struct OpCode {}

impl OpCode {
    pub const OP_HALT: u8 = 0x00;
    pub const OP_CONST: u8 = 0x01;
}

#[derive(Copy, Clone)]
pub enum EvaValue {
    Number(f64),
}