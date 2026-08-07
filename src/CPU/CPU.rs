use super::registers;

macro_rules! add_or_sub_reg {
    ($reg_name:ident, $self:expr, $method:ident) => {
        let value = $self.registers.$reg_name;
        let new_value = $self.$method(value);
        $self.registers.a = new_value;
    };
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
}

struct CPU {
    registers: registers::Registers,
}

impl CPU {
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

    fn subtract(&mut self, value: u8) -> u8 {
        let (new_value, did_borrow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_borrow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.a = new_value;
        new_value
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => {
                    add_or_sub_reg!(a, self, add);
                }
                ArithmeticTarget::B => {
                    add_or_sub_reg!(b, self, add);
                }
                ArithmeticTarget::C => {
                    add_or_sub_reg!(c, self, add);
                }
                ArithmeticTarget::D => {
                    add_or_sub_reg!(d, self, add);
                }
                ArithmeticTarget::E => {
                    add_or_sub_reg!(e, self, add);
                }
                ArithmeticTarget::H => {
                    add_or_sub_reg!(h, self, add);
                }
                ArithmeticTarget::L => {
                    add_or_sub_reg!(l, self, add);
                }
            },
            Instruction::SUB(target) => match target {
                ArithmeticTarget::A => {
                    add_or_sub_reg!(a, self, subtract);
                }
                ArithmeticTarget::B => {
                    add_or_sub_reg!(b, self, subtract);
                }
                ArithmeticTarget::C => {
                    add_or_sub_reg!(c, self, subtract);
                }
                ArithmeticTarget::D => {
                    add_or_sub_reg!(d, self, subtract);
                }
                ArithmeticTarget::E => {
                    add_or_sub_reg!(e, self, subtract);
                }
                ArithmeticTarget::H => {
                    add_or_sub_reg!(h, self, subtract);
                }
                ArithmeticTarget::L => {
                    add_or_sub_reg!(l, self, subtract);
                }
            },
            _ => {
                todo!()
            }
        }
    }
}
