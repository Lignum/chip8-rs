pub mod registers;
pub mod memory;

pub struct CPU {
    regs: registers::Registers,
    stack: [u16; 16]
}