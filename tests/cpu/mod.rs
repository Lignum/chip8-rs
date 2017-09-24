mod registers;
mod memory;

use chip8::io::{HeadlessInterface, IOInterface};
use chip8::cpu::CPU;

#[test]
pub fn ld_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.execute(0x6142); // LD V1, 0x42
    cpu.execute(0x6224); // LD V2, 0x24
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x24), cpu.regs.v(0x2));

    cpu.execute(0x8210); // LD V2, V1
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x42), cpu.regs.v(0x2));

    cpu.execute(0xA666); // LD I, 0x666
    assert_eq!(0x666, cpu.regs.i);
}

#[test]
#[should_panic(expected = "can't RET with an empty stack")]
pub fn invalid_ret_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x00, 0xEE
    ]);

    cpu.run(true);
}

#[test]
pub fn flow_jp_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x00, // 0200 - LD V0, 0x00
        0x70, 0x01, // 0202 - ADD V0, 0x01
        0x30, 0x0A, // 0204 - SE V0, 0x0A
        0x12, 0x02, // 0206 - JP 0x002
    ]);

    cpu.run(true);

    assert_eq!(Some(0x0A), cpu.regs.v(0x0));

    cpu.mem.load_program(&[
        0x60, 0x04, // 0200 - LD V0, 0x02
        0x61, 0x00, // 0202 - LD V1, 0x00
        0x71, 0x01, // 0204 - ADD V1, 0x01
        0x31, 0x0A, // 0206 - SE V1, 0x0A
        0xB2, 0x00, // 0208 - JP V0, 0x002
    ]);

    cpu.regs.pc = 0x200;
    cpu.run(true);
}

#[test]
pub fn add_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x03, // LD V0, 0x03
        0x70, 0x07, // ADD V0, 0x07

        0x62, 0x02, // LD V2, 0x02
        0x61, 0xFF, // LD V1, 0xFF
        0x82, 0x14, // ADD V2, V1
    ]);

    cpu.run(true);

    assert_eq!(Some(0xA), cpu.regs.v(0x0));
    assert_eq!(Some(0x1), cpu.regs.v(0x2));
    assert_eq!(Some(0x1), cpu.regs.v(0xF));
}

#[test]
pub fn subtract_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x08, // LD V0, 0x08
        0x61, 0x05, // LD V1, 0x05
        0x80, 0x15, // SUB V0, V1
    ]);

    cpu.run(true);

    assert_eq!(Some(0x03), cpu.regs.v(0x0));
    assert_eq!(Some(0x1), cpu.regs.v(0xF));

    cpu.mem.load_program(&[
        0x62, 0x04, // LD V2, 0x04
        0x63, 0x08, // LD V3, 0x08
        0x82, 0x37, // SUBN V2, V3
    ]);

    cpu.regs.pc = 0x200;
    cpu.run(true);

    assert_eq!(Some(0xFC), cpu.regs.v(0x2));
    assert_eq!(Some(0x1), cpu.regs.v(0xF));
}

#[test]
pub fn shift_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x04, // LD V0, 0x04
        0x81, 0x0E, // SHL V1, V0

        0x62, 0x08, // LD V2, 0x08
        0x83, 0x26, // SHR V3, V2
    ]);

    cpu.run(true);

    assert_eq!(Some(0x08), cpu.regs.v(0x1));
    assert_eq!(Some(0x04), cpu.regs.v(0x3));
}

#[test]
pub fn branch_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x42, // LD V0, 0x42
        0x30, 0x42, // SE V0, 0x42
        0x60, 0x24, // LD V0, 0x24

        0x61, 0x24, // LD V1, 0x24
        0x41, 0x42, // SNE V1, 0x42
        0x61, 0x42, // LD V1, 0x42,

        0x62, 0x20, // LD V2, 0x20
        0x90, 0x10, // SNE V0, V1
        0x62, 0x10, // LD V2, 0x10

        0x65, 0x32, // LD V5, 0x32
        0x63, 0x80, // LD V3, 0x80
        0x64, 0x80, // LD V4, 0x80
        0x53, 0x40, // SE V3, V4
        0x65, 0x64, // LD V5, 0x64
    ]);

    cpu.run(true);

    assert_eq!(Some(0x42), cpu.regs.v(0));
    assert_eq!(Some(0x24), cpu.regs.v(1));
    assert_eq!(Some(0x20), cpu.regs.v(2));
    assert_eq!(Some(0x32), cpu.regs.v(5));
}

#[test]
pub fn reg_mem_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x01, // LD V0, 0x01
        0x61, 0x02, // LD V1, 0x02
        0x62, 0x03, // LD V2, 0x03
        0xA0, 0x04, // LD I, 0x004
        0xF2, 0x55, // LD [I], V2

        0x60, 0x00, // LD V0, 0x00
        0x61, 0x00, // LD V1, 0x00
        0x62, 0x00, // LD V2, 0x00

        0xA0, 0x04, // LD I, 0x004
        0xF2, 0x65, // LD V2, [I]
    ]);

    cpu.run(true);

    assert_eq!(Some(0x01), cpu.mem.peek(0x004));
    assert_eq!(Some(0x02), cpu.mem.peek(0x005));
    assert_eq!(Some(0x03), cpu.mem.peek(0x006));

    assert_eq!(Some(0x01), cpu.regs.v(0));
    assert_eq!(Some(0x02), cpu.regs.v(1));
    assert_eq!(Some(0x03), cpu.regs.v(2));

    assert_eq!(0x007, cpu.regs.i);
}

struct ClearTestInterface {}

impl IOInterface for ClearTestInterface {
    fn clear_screen(&mut self) {
        panic!("ok")
    }

    fn is_key_pressed(&self, _: u8) -> bool { false }

    fn set_pixel(&mut self, _: u32, _: u32, _: bool) -> () {}

    fn pixel(&self, _: u32, _: u32) -> bool { false }
}

#[test]
#[should_panic(expected = "ok")]
pub fn clear_test() {
    let mut cpu = CPU::new(ClearTestInterface {});
    cpu.mem.load_program(&[
        0x00, 0xE0, // CLS
    ]);

    cpu.run(true);
}

#[test]
pub fn ld_char_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x02, // LD V0, 2
        0xF0, 0x29, // LD F, V0
    ]);
    
    cpu.run(true);

    assert_eq!(5 * 2, cpu.regs.i);
}

#[test]
pub fn bcd_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x63, 0x80, // LD V3, 128
        0xA4, 0x00, // LD I, 0x400
        0xF3, 0x33, // LD B, V3
    ]);

    cpu.run(true);

    if let Some(block) = cpu.mem.block(0x400, 3) {
        assert_eq!(1, block[0]);
        assert_eq!(2, block[1]);
        assert_eq!(8, block[2]);
    } else {
        panic!("cpu.mem.block(0x400, 3) failed");
    }
}

#[test]
pub fn step_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x42 // LD V0, 0x42
    ]);

    cpu.step();

    assert_eq!(Some(0x42), cpu.regs.v(0));
}

#[test]
pub fn run_test() {
    let mut cpu = CPU::new(HeadlessInterface {});
    cpu.mem.load_program(&[
        0x60, 0x42, // LD V0, 0x42
        0x61, 0x24, // LD V1, 0x24
        0x62, 0x22, // LD V2, 0x22
        0x63, 0x44  // LD V3, 0x44
    ]);

    cpu.run(true);

    assert_eq!(Some(0x42), cpu.regs.v(0));
}
