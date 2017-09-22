use chip8::cpu::memory::Memory;
use chip8::cpu::memory::CHIP8_MEMORY_SIZE;

#[test]
pub fn memory_in_range() {
    let mem = Memory::new();
    assert_eq!(true, mem.in_range(0));
    assert_eq!(false, mem.in_range(CHIP8_MEMORY_SIZE));
    assert_eq!(true, mem.in_range(CHIP8_MEMORY_SIZE - 1));
}

#[test]
pub fn memory_peek_poke() {
    let mut mem = Memory::new();
    assert_eq!(Some(()), mem.poke(0x0, 8));
    assert_eq!(None, mem.poke(CHIP8_MEMORY_SIZE, 8));
    assert_eq!(Some(8), mem.peek(0x0));
    assert_eq!(None, mem.peek(CHIP8_MEMORY_SIZE));
}