#[cfg(test)]
mod test {
    use crate::CPU::CPU::{ArithmeticTarget, CPU, Instruction, R16Registers};

    #[test]
    fn test_OR() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0b0101_1010;
        cpu.registers.a = 0b1010_0101;
        cpu.execute(Instruction::OR(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_XOR() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0b0101_1011;
        cpu.registers.a = 0b1010_0101;
        cpu.execute(Instruction::XOR(ArithmeticTarget::C));
        assert_eq!(0xFE, cpu.registers.a);
    }

    #[test]
    fn test_AND() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0xF;
        cpu.registers.a = 0b1111_0101;
        cpu.execute(Instruction::AND(ArithmeticTarget::C));
        assert_eq!(0x5, cpu.registers.a);
    }

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
    fn test_ADC() {
        let mut cpu = CPU::test();
        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticTarget::C));
        assert_eq!(0x1, cpu.registers.a);
    }

    #[test]
    fn test_SBC() {
        let mut cpu = CPU::test();
        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_ADD() {
        let mut cpu = CPU::test();
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(0x1, cpu.registers.a);
    }

    #[test]
    fn test_ADDHL() {
        let mut cpu = CPU::test();
        cpu.registers.set_bc(0x10);
        cpu.execute(Instruction::ADDHL(R16Registers::BC));
        assert_eq!(0x10, cpu.registers.get_hl());
    }

    #[test]
    fn test_SUB() {
        let mut cpu = CPU::test();
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        cpu.execute(Instruction::SUB(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
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
}
