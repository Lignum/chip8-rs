pub mod registers;
pub mod memory;

use self::registers::Registers;
use self::memory::Memory;

use std;
use std::num::Wrapping;

use rand;
use rand::{ThreadRng, Rng};

#[derive(Debug)]
pub enum Interrupt {
    None,
    AwaitKey(u8)
}

pub struct CPUEnvironment {
    keyboard: [bool; 16],
    display: Vec<bool>,
    display_width: u8,
    display_height: u8
}

impl CPUEnvironment {
    pub fn new(display_width: u8, display_height: u8) -> CPUEnvironment {
        CPUEnvironment {
            keyboard: [false; 16],
            display: vec![false; display_width as usize * display_height as usize],
            display_width,
            display_height
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        match self.keyboard.get(key as usize) {
            Some(pressed) => *pressed,
            None => false
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, white: bool) {
        let x = x % self.display_width as u32;
        let y = y % self.display_height as u32;

        match self.display.get_mut((y * self.display_width as u32 + x) as usize) {
            Some(pix) => *pix = white,
            None => ()
        }
    }

    pub fn pixel(&mut self, x: u32, y: u32) -> bool {
        let x = x % self.display_width as u32;
        let y = y % self.display_height as u32;

        match self.display.get((y * self.display_width as u32 + x) as usize) {
            Some(pix) => *pix,
            None => false
        }
    }

    pub fn clear_screen(&mut self) {
        self.display = vec![false; self.display_width as usize * self.display_height as usize];
    }
}

pub struct CPU {
    pub regs: Registers,
    pub mem: Memory,
    pub env: CPUEnvironment,
    pub interrupt: Interrupt,
    stack: Vec<u16>,
    rng: ThreadRng,
}

fn unknown_inst() {
    panic!("unknown instruction!")
}

impl CPU {
    pub fn new(display_width: u8, display_height: u8) -> CPU {
        CPU {
            regs: Registers::new(),
            mem: Memory::new(),
            stack: Vec::with_capacity(16),
            rng: rand::thread_rng(),
            interrupt: Interrupt::None,
            env: CPUEnvironment::new(display_width, display_height)
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
            panic!("can't RET with an empty stack")
        }
    }

    fn v(&self, i: u8) -> u8 {
        self.regs.v(i as usize).expect("invalid V register")
    }

    fn set_v(&mut self, i: u8, v: u8) {
        self.regs.set_v(i as usize, v).expect("invalid V register")
    }

    fn draw(&mut self, gx: u8, gy: u8, addr: u16, size: u8) {
        let pixels = {
            if let Some(block) = self.mem.block(addr as usize, size as usize) {
                let mut vec = Vec::with_capacity(size as usize);
                block.clone_into(&mut vec);
                vec
            } else {
                panic!("could not draw due to I being out of range")
            }
        };

        self.set_v(0xF, 0);

        for (y, pixel) in pixels.into_iter().enumerate() {
            for x in 0..7 {
                if pixel & (0x80 >> x) != 0 {
                    let lx = gx as u32 + x as u32;
                    let ly = gy as u32 + y as u32;
                    let white = self.env.pixel(lx, ly);

                    if white {
                        self.set_v(0xF, 1);
                    }

                    self.env.set_pixel(lx, ly, !white);
                }
            }
        }
    }

    pub fn press_key(&mut self, key: u8) {
        if let Interrupt::AwaitKey(reg) = self.interrupt {
            self.set_v(reg, key);
            self.interrupt = Interrupt::None;
        }
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
                0xE0 => self.env.clear_screen(),
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
                self.jump(c2 + v0 as u16);
            },
            // RND Vx, x
            0xC => {
                let r = self.rng.gen::<u8>();
                self.set_v(n2, r & b2);
            },

            0xE => {
                match b2 {
                    // SKP Vx
                    0x9E => if self.env.is_key_pressed(self.v(n2)) { self.skip() },
                    // SKNP Vx
                    0xA1 => if !self.env.is_key_pressed(self.v(n2)) { self.skip() },

                    _ => unknown_inst()
                }
            },

            // DRW Vx, Vy, x
            0xD => {
                let x = self.v(n2);
                let y = self.v(n3);
                let i = self.regs.i;
                self.draw(x, y, i, n4)
            },

            0xF => {
                match b2 {
                    // LD Vx, DT
                    0x07 => {
                        let dt = self.regs.dt;
                        self.set_v(n2, dt);
                    },
                    // LD Vx, K
                    0x0A => self.interrupt = Interrupt::AwaitKey(n2),
                    // LD DT, Vx
                    0x15 => self.regs.dt = self.v(n2),
                    // LD ST, Vx
                    0x18 => self.regs.st = self.v(n2),
                    // ADD I, Vx
                    0x1E => self.regs.i += self.v(n2) as u16,
                    // LD F, Vx
                    0x29 => self.regs.i = 0x00 + 5 * self.v(n2) as u16,
                    // LD B, Vx
                    0x33 => {
                        let v = self.v(n2);
                        if let Some(block) = self.mem.block_mut(self.regs.i as usize, 3) {
                            block[0] = v / 100;
                            block[1] = (v / 10) % 10;
                            block[2] = (v % 100) % 10;
                        }
                    },
                    // LD [I], Vx
                    0x55 => {
                        for i in 0..n2+1 {
                            let v = self.v(i);
                            if self.mem.poke(self.regs.i as usize, v).is_none() {
                                break;
                            }
                            self.regs.i += 1;
                        }
                    },
                    // LD Vx, [I]
                    0x65 => {
                        for i in 0..n2+1 {
                            if let Some(v) = self.mem.peek(self.regs.i as usize) {
                                self.regs.set_v(i as usize, v);
                            } else {
                                break;
                            }
                            self.regs.i += 1;
                        }
                    },

                    _ => unknown_inst()
                }
            }

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

    pub fn step(&mut self) -> bool {
        match self.interrupt {
            Interrupt::None => {
                let opcode = self.fetch();
                self.execute(opcode);
                self.regs.pc += 2;
                true
            },
            _ => false
        }
    }

    pub fn run(&mut self, stop_at_0: bool) {
        while !stop_at_0 || self.fetch() != 0x0000 {
            self.step();
        }
    }
}
