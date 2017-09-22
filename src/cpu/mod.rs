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
        let n2 = ((opcode & 0x0F00) >> 8) as u8;
        let n3 = ((opcode & 0x00F0) >> 4) as u8;
        let b1 = ((opcode & 0xFF00) >> 8) as u8;
        let b2 = (opcode & 0x00FF) as u8;
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
            0x6 => self.regs.set_v(n2 as usize, b2).expect("invalid V register in LD Vx, y"),
            // LD Vx, Vy
            0x8 => self.regs.v(n3 as usize).and_then(|v| self.regs.set_v(n2 as usize, v)).expect("invalid V register in LD Vx, Vy"),

            _ => unknown_inst()
        }
    }
}