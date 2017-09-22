mod registers;
mod memory;

use chip8::cpu::CPU;

#[test]
pub fn ld_test() {
    let mut cpu = CPU::new();

    cpu.execute(0x6142); // LD V1, 0x42
    cpu.execute(0x6224); // LD V2, 0x24
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x24), cpu.regs.v(0x2));

    cpu.execute(0x8210); // LD V2, V1
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x42), cpu.regs.v(0x2));
}

#[test]
pub fn add_test() {
    let mut cpu = CPU::new();

    cpu.execute(0x6003); // LD V0, 0x03
    cpu.execute(0x7007); // ADD V0, 0x07
    assert_eq!(Some(0xA), cpu.regs.v(0x0));
}

#[test]
pub fn branch_test() {
    let mut cpu = CPU::new();
    cpu.mem.load_program(&[
        0x60, 0x42, // LD V0, 0x42
        0x30, 0x42, // SE V0, 0x42
        0x60, 0x24, // LD V0, 0x24

        0x61, 0x24, // LD V1, 0x24
        0x41, 0x42, // SNE V1, 0x42
        0x61, 0x42, // LD V1, 0x42
    ]).unwrap();

    cpu.run(true);

    assert_eq!(Some(0x42), cpu.regs.v(0));
    assert_eq!(Some(0x24), cpu.regs.v(1));
}

#[test]
pub fn step_test() {
    let mut cpu = CPU::new();
    cpu.mem.load_program(&[
        0x60, 0x42 // LD V0, 0x42
    ]).unwrap();

    cpu.step();

    assert_eq!(Some(0x42), cpu.regs.v(0));
}

#[test]
pub fn run_test() {
    let mut cpu = CPU::new();
    cpu.mem.load_program(&[
        0x60, 0x42, // LD V0, 0x42
        0x61, 0x24, // LD V1, 0x24
        0x62, 0x22, // LD V2, 0x22
        0x63, 0x44  // LD V3, 0x44
    ]).unwrap();

    cpu.run(true);

    assert_eq!(Some(0x42), cpu.regs.v(0));
}