pub mod registers;
pub mod memory;
pub mod chars;

use self::registers::Registers;
use self::memory::Memory;

pub struct CPU {
    pub regs: Registers,
    pub mem: Memory,
    pub stack: Vec<u16>
}

fn unknown_inst() {
    panic!("unknown instruction!")
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new(),
            mem: Memory::new(),
            stack: Vec::with_capacity(16)
        }
    }

    pub fn jump(&mut self, addr: u16) {
        self.regs.pc = addr;
    }

    pub fn call(&mut self, addr: u16) {
        self.stack.push(addr);
        self.jump(addr);
    }

    pub fn ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.jump(addr);
        } else {
            panic!("can't RET with an empty stack!!")
        }
    }

    pub fn execute(&mut self, opcode: u16) {
        let op = ((opcode & 0xF000) >> 12) as u8;
        let n2 = ((opcode & 0x0F00) >> 8) as usize;
        let n3 = ((opcode & 0x00F0) >> 4) as usize;
        let b2 = (opcode & 0x00FF) as usize;
        let c2 = (opcode & 0x0FFF) as u16;

        match op {
            0x0 => match b2 {
                // CLS
                0xE0 => /* TODO: clear screen */ (),
                // RET
                0xEE => self.ret(),
                // SYS, ignore
                _ => ()
            },

            // JP
            0x1 => self.jump(c2),
            // CALL
            0x2 => self.call(c2),
            // LD Vx, y
            0x6 => self.regs.set_v(n2, b2 as u8).expect("invalid V register in LD Vx, y"),
            // ADD Vx, y
            0x7 => self.regs.v(n2).and_then(|v| self.regs.set_v(n2, b2 as u8 + v)).expect("invalid V register in ADD Vx, y"),
            // LD Vx, Vy
            0x8 => self.regs.v(n3).and_then(|v| self.regs.set_v(n2, v)).expect("invalid V register in LD Vx, Vy"),

            _ => unknown_inst()
        }
    }
}