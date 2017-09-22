pub mod registers;
pub mod memory;
pub mod chars;

pub struct CPU {
    regs: registers::Registers,
    stack: [u16; 16]
}