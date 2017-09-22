pub struct Registers {
    pub i: u16,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    v: [u8; 16]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            v: [0; 16]
        }
    }

    pub fn valid_register_index(i: usize) -> bool {
        i <= 0xF
    }

    pub fn v(&self, x: usize) -> Option<u8> {
        if Registers::valid_register_index(x) {
            Some(self.v[x])
        } else {
            None
        }
    }

    pub fn set_v(&mut self, x: usize, v: u8) -> Option<()> {
        if Registers::valid_register_index(x) {
            self.v[x] = v;
            Some(())
        } else {
            None
        }
    }
}