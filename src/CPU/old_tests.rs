use crate::CPU::CPU::{ArithmeticTarget, CPU, Instruction, R16Registers};

#[test]
fn test_SWAP() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0xF;
    cpu.execute(Instruction::SWAP(ArithmeticTarget::C));
    assert_eq!(0xF0, cpu.registers.c);
}

#[test]
fn test_INC_and_DEC() {
    let mut cpu = CPU::test();
    cpu.execute(Instruction::INC(ArithmeticTarget::C));
    assert_eq!(0x1, cpu.registers.c);
    cpu.execute(Instruction::DEC(ArithmeticTarget::C));
    assert_eq!(0x0, cpu.registers.c);
}

#[test]
fn test_half_carry_on_INC() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0xF;
    cpu.execute(Instruction::INC(ArithmeticTarget::C));
    assert_eq!(true, cpu.registers.f.half_carry);
}

#[test]
fn test_half_carry_on_DEC() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0x10;
    cpu.execute(Instruction::DEC(ArithmeticTarget::C));
    assert_eq!(true, cpu.registers.f.half_carry);
}

#[test]
fn test_ADDHL() {
    let mut cpu = CPU::test();
    cpu.registers.set_bc(0x10);
    cpu.execute(Instruction::ADDHL(R16Registers::BC));
    assert_eq!(0x10, cpu.registers.get_hl());
}

#[test]
fn test_RL() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0b1010_0101;
    cpu.execute(Instruction::RL(ArithmeticTarget::C));
    assert_eq!(true, cpu.registers.f.carry);
    assert_eq!(0b0100_1010, cpu.registers.c);
}

#[test]
fn test_RR() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0b1010_0100;
    cpu.execute(Instruction::RR(ArithmeticTarget::C));
    assert_eq!(false, cpu.registers.f.carry);
    assert_eq!(0b0101_0010, cpu.registers.c);
}

#[test]
fn test_RRC() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0b1010_0100;
    cpu.execute(Instruction::RRC(ArithmeticTarget::C));
    assert_eq!(false, cpu.registers.f.carry);
    assert_eq!(0b0101_0010, cpu.registers.c);
}

#[test]
fn test_RLC() {
    let mut cpu = CPU::test();
    cpu.registers.c = 0b1010_0100;
    cpu.execute(Instruction::RLC(ArithmeticTarget::C));
    assert_eq!(true, cpu.registers.f.carry);
    assert_eq!(0b0100_1001, cpu.registers.c);
}

#[test]
fn test_INC16() {
    let mut cpu = CPU::test();
    cpu.execute(Instruction::INC16(R16Registers::BC));
    assert_eq!(0x1, cpu.registers.get_bc());
}

#[test]
fn test_DEC16() {
    let mut cpu = CPU::test();
    cpu.execute(Instruction::DEC16(R16Registers::BC));
    assert_eq!(0xFFFF, cpu.registers.get_bc());
}
