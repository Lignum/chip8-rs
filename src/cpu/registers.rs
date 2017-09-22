pub struct Registers {
    pub i: u16,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
    v: [u8; 16]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
            v: [0; 16]
        }
    }

    pub fn valid_register_index(i: usize) -> bool {
        i <= 0xF
    }

    pub fn v(&self, i: usize) -> Option<u8> {
        if Registers::valid_register_index(i) {
            Some(self.v[i])
        } else {
            None
        }
    }

    pub fn set_v(&mut self, i: usize, v: u8) -> Option<()> {
        if Registers::valid_register_index(i) {
            self.v[i] = v;
            Some(())
        } else {
            None
        }
    }
}