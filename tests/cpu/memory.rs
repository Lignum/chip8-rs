use chip8::cpu::memory::Memory;
use chip8::cpu::memory::CHIP8_MEMORY_SIZE;
use chip8::cpu::chars::CHIP8_CHARACTERS;

#[test]
pub fn memory_in_range() {
    let mem = Memory::new();
    assert!(mem.in_range(0));
    assert!(!mem.in_range(CHIP8_MEMORY_SIZE));
    assert!(mem.in_range(CHIP8_MEMORY_SIZE - 1));
}

#[test]
pub fn memory_peek_poke() {
    let mut mem = Memory::new();
    assert_eq!(Some(()), mem.poke(0x0, 8));
    assert_eq!(None, mem.poke(CHIP8_MEMORY_SIZE, 8));
    assert_eq!(Some(8), mem.peek(0x0));
    assert_eq!(None, mem.peek(CHIP8_MEMORY_SIZE));
}

#[test]
pub fn memory_characters() {
    let mem = Memory::new();
    assert!(mem.block(0x0, 5 * 16) == Some(&CHIP8_CHARACTERS));
}

#[test]
pub fn memory_peek_poke_block() {
    let mut mem = Memory::new();
    
    if let Some(block) = mem.block_mut(0x0, 3) {
        block[0] = 1;
        block[1] = 2;
        block[2] = 3;
    } else {
        panic!("write: block == None");
    }

    assert_eq!(Some(1), mem.peek(0x0));
    assert_eq!(Some(2), mem.peek(0x1));
    assert_eq!(Some(3), mem.peek(0x2));

    if let Some(block) = mem.block(0x0, 3) {
        assert_eq!(&[1, 2, 3], block);
    } else {
        panic!("read: block == None");
    }
}