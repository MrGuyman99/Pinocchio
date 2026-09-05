use crate::CPU::CPU::{ArithmeticTarget, CPU, Instruction, R16Registers};

#[test]
fn test_ADDHL() {
    let mut cpu = CPU::test();
    cpu.registers.set_bc(0x10);
    cpu.execute(Instruction::ADDHL(R16Registers::BC));
    assert_eq!(0x10, cpu.registers.get_hl());
}
