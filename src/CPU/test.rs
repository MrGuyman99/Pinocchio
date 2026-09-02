// All tests are done on register c
#[cfg(test)]
mod test {
    use crate::CPU::CPU::*;
    #[test]
    fn test_ADD() {
        let mut cpu = CPU::test();
        cpu.registers.c = 1;
        cpu.execute(0x81);
        assert_eq!(0x1, cpu.registers.a)
    }

    #[test]
    fn test_ADC() {
        let mut cpu = CPU::test();
        cpu.registers.f.carry = true;
        cpu.execute(0x89);
        assert_eq!(0x1, cpu.registers.a);
    }

    #[test]
    fn test_SUB() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0x1;
        cpu.execute(0x91);
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_SBC() {
        let mut cpu = CPU::test();
        cpu.registers.f.carry = true;
        cpu.registers.c = 0x1;
        cpu.execute(0x99);
        assert_eq!(0xFE, cpu.registers.a);
    }

    #[test]
    fn test_AND() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0xF;
        cpu.registers.a = 0b1111_0101;
        cpu.execute(0xA1);
        assert_eq!(0x5, cpu.registers.a);
    }

    #[test]
    fn test_XOR() {
        let mut cpu = CPU::test();
        cpu.registers.c = 0b0101_1011;
        cpu.registers.a = 0b1010_0101;
        cpu.execute(0xA9);
        assert_eq!(0xFE, cpu.registers.a);
    }
}
