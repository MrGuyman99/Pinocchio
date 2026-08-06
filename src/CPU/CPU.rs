use super::registers;

macro_rules! add_reg {
    ($reg_name:ident, $self:expr) => {
        let value = $self.registers.$reg_name;
        let new_value = $self.add(value);
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

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => {
                    add_reg!(a, self);
                }
                ArithmeticTarget::B => {
                    add_reg!(b, self);
                }
                ArithmeticTarget::C => {
                    add_reg!(c, self);
                }
                ArithmeticTarget::D => {
                    add_reg!(d, self);
                }
                ArithmeticTarget::E => {
                    add_reg!(e, self);
                }
                ArithmeticTarget::H => {
                    add_reg!(h, self);
                }
                ArithmeticTarget::L => {
                    add_reg!(l, self);
                }
            },
            _ => {
                todo!()
            }
        }
    }
}
