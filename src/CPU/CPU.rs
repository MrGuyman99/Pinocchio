use super::registers;

/*
void add_one(int &ThingToAdd){
    ThingToAdd++;
}

int test = 2;
add_one(test);

*/

// For any operation working with r8's that manipulate the a register
macro_rules! operation_on_a_match {
    ($self:ident, $method:ident, $target:ident) => {
        match $target {
            ArithmeticTarget::A => {
                let value = $self.registers.a;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::B => {
                let value = $self.registers.b;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::C => {
                let value = $self.registers.c;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::D => {
                let value = $self.registers.d;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::E => {
                let value = $self.registers.e;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::H => {
                let value = $self.registers.h;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
            ArithmeticTarget::L => {
                let value = $self.registers.l;
                let new_value = $self.$method(value);
                $self.registers.a = new_value;
            }
        }
    };
}

// For any operation with r8's that manipulate only themselves
macro_rules! operation_on_self_match {
    ($self:ident, $method:ident, $target:ident) => {
        match $target {
            ArithmeticTarget::A => {
                $self.registers.a = $self.$method($self.registers.a);
            }
            ArithmeticTarget::B => {
                $self.registers.b = $self.$method($self.registers.b);
            }
            ArithmeticTarget::C => {
                $self.registers.c = $self.$method($self.registers.c);
            }
            ArithmeticTarget::D => {
                $self.registers.d = $self.$method($self.registers.d);
            }
            ArithmeticTarget::E => {
                $self.registers.e = $self.$method($self.registers.e);
            }
            ArithmeticTarget::H => {
                $self.registers.h = $self.$method($self.registers.h);
            }
            ArithmeticTarget::L => {
                $self.registers.l = $self.$method($self.registers.l);
            }
        }
    };
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum R16Registers {
    BC,
    DE,
    HL,
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    AND(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    RL(ArithmeticTarget),
    ADDHL(R16Registers),
}

pub struct CPU {
    pub registers: registers::Registers,
}

impl CPU {
    // Note: Placeholder just for testing
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: registers::FlagsRegister {
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                },
                h: 0,
                l: 0,
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // If the lower nibble of the value and register a together result in a bigger value than 0xF
        // Then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn add_carry(&mut self, value: u8) -> u8 {
        let (add_value, _did_overflow) = value.overflowing_add(u8::from(self.registers.f.carry));
        let (new_value, did_overflow) = self.registers.a.overflowing_add(add_value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (value & 0xFFF) + (value & 0xFFF) > 0xFFF;
        new_value
    }

    fn subtract(&mut self, value: u8) -> u8 {
        let (new_value, did_borrow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_borrow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.a = new_value;
        new_value
    }

    fn subtract_carry(&mut self, value: u8) -> u8 {
        let (subtract_value, _did_overflow) =
            value.overflowing_add(u8::from(self.registers.f.carry));
        let (new_value, did_borrow) = self.registers.a.overflowing_sub(subtract_value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_borrow;
        self.registers.f.half_carry =
            ((self.registers.a + u8::from(self.registers.f.carry)) & 0xF) < (value & 0xF);
        new_value
    }

    fn increment(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0xF;
        new_value
    }

    fn decrement(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0x0;
        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn swap(&mut self, value: u8) -> u8 {
        let new_value = (value << 0x4) | (value >> 0x4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn rotate_left_carry(&mut self, value: u8) -> u8 {
        let old_carry = u8::from(self.registers.f.carry);
        let new_carry = (value & 0x80) != 0;
        let new_value = (value << 1) | old_carry;

        self.registers.f.carry = new_carry;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        new_value
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => operation_on_a_match!(self, add, target),
            Instruction::SUB(target) => operation_on_a_match!(self, subtract, target),
            Instruction::AND(target) => operation_on_a_match!(self, and, target),
            Instruction::ADC(target) => operation_on_a_match!(self, add_carry, target),
            Instruction::SBC(target) => operation_on_a_match!(self, subtract_carry, target),
            Instruction::OR(target) => operation_on_a_match!(self, or, target),
            Instruction::XOR(target) => operation_on_a_match!(self, xor, target),
            Instruction::INC(target) => operation_on_self_match!(self, increment, target),
            Instruction::DEC(target) => operation_on_self_match!(self, decrement, target),
            Instruction::RL(target) => operation_on_self_match!(self, rotate_left_carry, target),
            Instruction::SWAP(target) => operation_on_self_match!(self, swap, target),
            Instruction::ADDHL(target) => match target {
                R16Registers::BC => {
                    let value = self.registers.get_bc();
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                }
                R16Registers::DE => {
                    let value = self.registers.get_de();
                    let new_value = self.add_hl(value);
                    self.registers.set_de(new_value);
                }
                R16Registers::HL => {
                    let value = self.registers.get_hl();
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_OR() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0b0101_1010;
        cpu.registers.a = 0b1010_0101;
        cpu.execute(Instruction::OR(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_XOR() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0b0101_1011;
        cpu.registers.a = 0b1010_0101;
        cpu.execute(Instruction::XOR(ArithmeticTarget::C));
        assert_eq!(0xFE, cpu.registers.a);
    }

    #[test]
    fn test_AND() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0xF;
        cpu.registers.a = 0b1111_0101;
        cpu.execute(Instruction::AND(ArithmeticTarget::C));
        assert_eq!(0x5, cpu.registers.a);
    }

    #[test]
    fn test_SWAP() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0xF;
        cpu.execute(Instruction::SWAP(ArithmeticTarget::C));
        assert_eq!(0xF0, cpu.registers.c);
    }

    #[test]
    fn test_INC_and_DEC() {
        let mut cpu = CPU::new();
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        assert_eq!(0x1, cpu.registers.c);
        cpu.execute(Instruction::DEC(ArithmeticTarget::C));
        assert_eq!(0x0, cpu.registers.c);
    }

    #[test]
    fn test_half_carry_on_INC() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0xF;
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        assert_eq!(true, cpu.registers.f.half_carry);
    }

    #[test]
    fn test_half_carry_on_DEC() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x10;
        cpu.execute(Instruction::DEC(ArithmeticTarget::C));
        assert_eq!(true, cpu.registers.f.half_carry);
    }

    #[test]
    fn test_ADC() {
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;
        cpu.execute(Instruction::ADC(ArithmeticTarget::C));
        assert_eq!(0x1, cpu.registers.a);
    }

    #[test]
    fn test_SBC() {
        let mut cpu = CPU::new();
        cpu.registers.f.carry = true;
        cpu.execute(Instruction::SBC(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_ADD() {
        let mut cpu = CPU::new();
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(0x1, cpu.registers.a);
    }

    #[test]
    fn test_ADDHL() {
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x10);
        cpu.execute(Instruction::ADDHL(R16Registers::BC));
        assert_eq!(0x10, cpu.registers.get_hl());
    }

    #[test]
    fn test_SUB() {
        let mut cpu = CPU::new();
        cpu.execute(Instruction::INC(ArithmeticTarget::C));
        cpu.execute(Instruction::SUB(ArithmeticTarget::C));
        assert_eq!(0xFF, cpu.registers.a);
    }

    #[test]
    fn test_RL() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0b1010_0101;
        cpu.execute(Instruction::RL(ArithmeticTarget::C));
        assert_eq!(true, cpu.registers.f.carry);
        assert_eq!(0b0100_1010, cpu.registers.c);
    }
}
