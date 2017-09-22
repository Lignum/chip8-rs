mod registers;
mod memory;

use chip8::cpu::CPU;

#[test]
pub fn ld_test() {
    let mut cpu = CPU::new();

    cpu.execute(0x6142);
    cpu.execute(0x6224);
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x24), cpu.regs.v(0x2));

    cpu.execute(0x8210);
    assert_eq!(Some(0x42), cpu.regs.v(0x1));
    assert_eq!(Some(0x42), cpu.regs.v(0x2));
}