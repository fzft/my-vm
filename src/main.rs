use crate::op_code::OpCode;
use log::{info, trace, warn, error};

mod op_code;

struct EvaVM {
    ip: *mut u8
}

impl EvaVM {
    fn new() -> Self {
        Self{ip: std::ptr::null_mut()}
    }

    // Exec a program
    fn exec(&mut self, program: &str) {
        // 1. parse the program
        // auto ast = parse

        // 2. Compile program to Eva bytecode
        let code =

        // Set instruction pointer to the beginning
        self.ip = program.as_ptr() as *mut u8;
        self.eval()
    }

    fn eval(&mut self) {
        loop {
            unsafe {
                match self.read_byte() {
                    Ok(OpCode::HALT)  => return,
                    Ok(op_code) => warn!("Unknown opcode: {}", op_code),
                    _ => error!("read byte got error")
                }
            }
        }
    }

    fn read_byte(&mut self) -> Result<u8, String> {
        unsafe {
            self.ip = self.ip.offset(1);
            Ok(*self.ip)
        }
    }
}

fn main() {
    env_logger::init();

}
