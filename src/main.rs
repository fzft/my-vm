use std::mem::MaybeUninit;

use log::{error, info, trace, warn};

use crate::token::{EvaValue, OpCode, Token};

mod token;

#[derive(Debug)]
enum Error {
    StackOverFlow,
    EmptyStack,
    ReadByte,
    UnknownOpCode(u8),
}

const STACK_LIMIT: usize = 512;

struct EvaVM {
    // Instruction pointer (aka Program counter)
    ip: *mut u8,

    // Stack pointer
    sp: usize,

    // Constant pool
    constants: Vec<EvaValue>,

    // Operands stack
    stack: [MaybeUninit<EvaValue>; STACK_LIMIT],
}

impl EvaVM {
    fn new() -> Self {
        let stack = unsafe { MaybeUninit::uninit().assume_init() };
        Self {
            ip: std::ptr::null_mut(),
            sp: 0,
            constants: Vec::new(),
            stack,
        }
    }

    // Exec a program
    fn exec(&mut self, program: &str) -> Result<EvaValue, Error> {
        // 1. parse the program
        // auto ast = parse

        // 2. Compile program to Eva bytecode
        self.constants.push(EvaValue::Number(42 as f64));

        let code: Vec<u8> = vec![OpCode::OP_CONST, 0, OpCode::OP_CONST];

        // Set instruction pointer to the beginning
        self.ip = code.as_ptr() as *mut u8;
        self.eval()
    }

    fn eval(&mut self) -> Result<EvaValue, Error> {
        loop {
            match self.read_byte() {
                Ok(OpCode::OP_HALT) => return self.pop(),
                Ok(OpCode::OP_CONST) => {
                    let const_index = self.read_byte().unwrap();
                    let constant = self.constants.remove(const_index as usize);
                    self.push(constant)?;
                }
                Ok(op_code) => {
                    error!("Unknown opcode: {}", op_code);
                    return Err(Error::UnknownOpCode(op_code));
                }
                _ => {
                    error!("read byte got error");
                    return Err(Error::ReadByte);
                }
            }
        }
    }

    // Reads the current byte in the bytecode and advances the ip pointer
    fn read_byte(&mut self) -> Result<u8, String> {
        unsafe {
            self.ip = self.ip.offset(1);
            Ok(*self.ip)
        }
    }

    // Pushes a value onto the stack
    fn push(&mut self, eva_value: EvaValue) -> Result<(), Error> {
        if self.sp == STACK_LIMIT {
            return Err(Error::StackOverFlow);
        }
        self.stack[self.sp] = MaybeUninit::new(eva_value);
        self.sp += 1;

        Ok(())
    }

    fn pop(&mut self) -> Result<EvaValue, Error> {
        if self.stack.len() == 0 {
            error!("pop(): empty stack. \n");
            return Err(Error::EmptyStack);
        }

        self.sp -= 1;
        let value_slot = &self.stack[self.sp];

        // Safety: we previously ensured that this slot contains a valid value.
        let value = unsafe {
            value_slot.assume_init_read()
        };
        Ok(value)
    }
}

fn main() {
    env_logger::init();
    let mut vm = EvaVM::new();
    vm.exec("42");
}
