use super::chars::CHIP8_CHARACTERS;

pub struct Memory {
    mem: [u8; 4096]
}

pub const CHIP8_MEMORY_SIZE: usize = 4096;

impl Memory {
    pub fn new() -> Memory {
        let mut mem: [u8; CHIP8_MEMORY_SIZE] = [0; CHIP8_MEMORY_SIZE];

        for (i, v) in CHIP8_CHARACTERS.iter().enumerate() {
            mem[i] = *v;
        }

        Memory { mem }
    }

    pub fn in_range(&self, addr: usize) -> bool {
        addr < self.mem.len()
    }

    pub fn block_in_range(&self, addr: usize, size: usize) -> bool {
        addr < self.mem.len() && addr + size < self.mem.len()
    }

    pub fn block(&self, addr: usize, size: usize) -> Option<&[u8]> {
        if self.block_in_range(addr, size) {
            Some(&self.mem[addr..(addr+size)])
        } else {
            None
        }
    }

    pub fn block_mut(&mut self, addr: usize, size: usize) -> Option<&mut [u8]> {
        if self.block_in_range(addr, size) {
            Some(&mut self.mem[addr..(addr+size)])
        } else {
            None
        }
    }

    pub fn peek(&self, addr: usize) -> Option<u8> {
        if self.in_range(addr) {
            Some(self.mem[addr])
        } else {
            None
        }
    }

    pub fn poke(&mut self, addr: usize, v: u8) -> Option<()> {
        if self.in_range(addr) {
            self.mem[addr] = v;
            Some(())
        } else {
            None
        }
    }

    pub fn load(&mut self, addr: usize, data: &[u8]) -> Option<()> {
        let block = self.block_mut(addr, data.len());

        match block {
            Some(block) => {
                for (i, v) in data.iter().enumerate() {
                    block[i] = *v;
                }
                Some(())
            },
            None => None
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Option<()> {
        self.load(0x200, program)
    }
}