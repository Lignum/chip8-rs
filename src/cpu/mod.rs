pub mod registers;
pub mod memory;
pub mod chars;

use self::registers::Registers;
use self::memory::Memory;

use std;
use std::num::Wrapping;

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

    fn jump(&mut self, addr: u16) {
        self.regs.pc = addr - 2;
    }

    fn skip(&mut self) {
        self.regs.pc += 2;
    }

    fn call(&mut self, addr: u16) {
        self.stack.push(addr);
        self.jump(addr);
    }

    fn ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.jump(addr);
        } else {
            panic!("can't RET with an empty stack!!")
        }
    }

    fn v(&self, i: u8) -> u8 {
        self.regs.v(i as usize).expect("invalid V register")
    }

    fn set_v(&mut self, i: u8, v: u8) {
        self.regs.set_v(i as usize, v).expect("invalid V register")
    }

    pub fn execute(&mut self, opcode: u16) {
        let op = ((opcode & 0xF000) >> 12) as u8;
        let n2 = ((opcode & 0x0F00) >> 8) as u8;
        let n3 = ((opcode & 0x00F0) >> 4) as u8;
        let n4 = (opcode & 0x000F) as u8;
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
            // SE Vx, y
            0x3 => if self.v(n2) == b2 { self.skip() },
            // SNE Vx, y
            0x4 => if self.v(n2) != b2 { self.skip() },
            // SE Vx, Vy
            0x5 => if self.v(n2) == self.v(n3) { self.skip() },
            // LD Vx, y
            0x6 => self.set_v(n2, b2),
            // ADD Vx, y
            0x7 => {
                let v = self.v(n2);
                self.set_v(n2, b2 + v);
            },
            // LD Vx, Vy
            0x8 => {
                let x = self.v(n2);
                let y = self.v(n3);

                match n4 {
                    // LD Vx, Vy
                    0x0 => self.set_v(n2, y),
                    // OR Vx, Vy
                    0x1 => self.set_v(n2, x | y),
                    // AND Vx, Vy
                    0x2 => self.set_v(n2, x & y),
                    // XOR Vx, Vy
                    0x3 => self.set_v(n2, x ^ y),
                    // ADD Vx, Vy
                    0x4 => {
                        let carry = if x as u16 + y as u16 > std::u8::MAX as u16 { 1 } else { 0 };
                        self.set_v(0xF, carry);
                        self.set_v(n2, (Wrapping(x) + Wrapping(y)).0);
                    },
                    // SUB Vx, Vy / SUBN Vx, Vy
                    0x5 | 0x7 => {
                        let borrow = if n4 == 0x5 { x <= y } else { y <= x };
                        self.set_v(0xF, if !borrow { 1 } else { 0 });
                        self.set_v(n2, (Wrapping(x) - Wrapping(y)).0);
                    },
                    // SHR Vx, Vy
                    0x6 => {
                        self.set_v(0xF, y & 0x1);
                        self.set_v(n2, y >> 1);
                    },
                    // SHL Vx, Vy
                    0xE => {
                        self.set_v(0xF, (y & 0x80) >> 7);
                        self.set_v(n2, y << 1);
                    },

                    _ => unknown_inst()
                }
            },
            // SNE Vx, Vy
            0x9 => if self.v(n2) != self.v(n3) { self.skip() },
            // LD I, x
            0xA => self.regs.i = c2,
            // JP V0, x
            0xB => {
                let v0 = self.v(0x0);
                self.jump(c2 + v0 as u16)
            },
            _ => unknown_inst()
        }
    }

    fn fetch(&self) -> u16 {
        if let Some(b) = self.mem.block(self.regs.pc as usize, 2) {
            ((b[0] as u16) << 8) | (b[1] as u16)
        } else {
            panic!("Failed to fetch next instruction!! Is PC out of bounds?")
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
        self.regs.pc += 2;
    }

    pub fn run(&mut self, stop_at_0: bool) {
        while !stop_at_0 || (stop_at_0 && self.fetch() != 0x0000) {
            self.step();
        }
    }
}