use chip8::cpu::registers::Registers;

#[test]
pub fn register_v_access() {
    let mut regs = Registers::new();
    assert_eq!(Some(0), regs.v(0x0));
    assert_eq!(Some(()), regs.set_v(0xA, 20));
    assert_eq!(None, regs.set_v(0xFF, 20));
    assert_eq!(Some(20), regs.v(0xA));
    assert_eq!(None, regs.v(0xFF));
}