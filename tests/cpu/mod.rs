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