use super::registers;
pub struct CPU {
    pub registers: registers::Registers,
}

impl CPU {
    pub fn test() -> Self {
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

    fn increment_16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xFF) == 0xFF;
        new_value
    }

    fn decrement_16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xFF) == 0x0;
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

    fn rotate_right_carry(&mut self, value: u8) -> u8 {
        let old_carry = u8::from(self.registers.f.carry) << 7;
        let new_carry = (value & 0x1) != 0;
        let new_value = (value >> 1) | old_carry;

        self.registers.f.carry = new_carry;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn rotate_left(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) != 0;
        new_value
    }

    fn rotate_right(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x1) != 0;
        new_value
    }

    pub fn execute_cb_prefixed(&mut self, instruction: u8) {
        match instruction {
            0x00 => {
                self.registers.b = self.rotate_left_carry(self.registers.b);
            }
            0x01 => {
                self.registers.c = self.rotate_left_carry(self.registers.c);
            }
            0x02 => {
                self.registers.d = self.rotate_left_carry(self.registers.d);
            }
            0x03 => {
                self.registers.e = self.rotate_left_carry(self.registers.e);
            }
            0x04 => {
                self.registers.h = self.rotate_left_carry(self.registers.h);
            }
            0x05 => {
                self.registers.l = self.rotate_left_carry(self.registers.l);
            }
            // TODO: RLC HL
            0x07 => {
                self.registers.a = self.rotate_left_carry(self.registers.a);
            }
            0x08 => {
                self.registers.b = self.rotate_right_carry(self.registers.b);
            }
            0x09 => {
                self.registers.c = self.rotate_right_carry(self.registers.c);
            }
            0x0A => {
                self.registers.d = self.rotate_right_carry(self.registers.d);
            }
            0x0B => {
                self.registers.e = self.rotate_right_carry(self.registers.e);
            }
            0x0C => {
                self.registers.h = self.rotate_right_carry(self.registers.h);
            }
            0x0D => {
                self.registers.l = self.rotate_right_carry(self.registers.l);
            }
            // TODO: RRC HL
            0x0F => {
                self.registers.a = self.rotate_right_carry(self.registers.a);
            }
            0x10 => {
                self.registers.b = self.rotate_left(self.registers.b);
            }
            0x11 => {
                self.registers.c = self.rotate_left(self.registers.c);
            }
            0x12 => {
                self.registers.d = self.rotate_left(self.registers.d);
            }
            0x13 => {
                self.registers.e = self.rotate_left(self.registers.e);
            }
            0x14 => {
                self.registers.h = self.rotate_left(self.registers.h);
            }
            0x15 => {
                self.registers.l = self.rotate_left(self.registers.l);
            }
            // TODO: RL HL
            0x17 => {
                self.registers.a = self.rotate_left(self.registers.a);
            }
            0x18 => {
                self.registers.b = self.rotate_right(self.registers.b);
            }
            0x19 => {
                self.registers.c = self.rotate_right(self.registers.c);
            }
            0x1A => {
                self.registers.d = self.rotate_right(self.registers.d);
            }
            0x1B => {
                self.registers.e = self.rotate_right(self.registers.e);
            }
            0x1C => {
                self.registers.h = self.rotate_right(self.registers.h);
            }
            0x1D => {
                self.registers.l = self.rotate_right(self.registers.l);
            }
            // TODO: RR HL
            0x1F => {
                self.registers.a = self.rotate_right(self.registers.a);
            }
            0x30 => {
                self.registers.b = self.swap(self.registers.b);
            }
            0x31 => {
                self.registers.c = self.swap(self.registers.c);
            }
            0x32 => {
                self.registers.d = self.swap(self.registers.d);
            }
            0x33 => {
                self.registers.e = self.swap(self.registers.e);
            }
            0x34 => {
                self.registers.h = self.swap(self.registers.h);
            }
            0x35 => {
                self.registers.l = self.swap(self.registers.l);
            }
            // TODO: Swap HL
            0x37 => {
                self.registers.a = self.swap(self.registers.a);
            }
            _ => {
                println!("Instruction {} not implemented!", instruction);
            }
        }
    }
    /*
    For the future, what rboy does is it fetches the byte of the instruction in a seperate function
    then, when 0xCB is called it passes the instruction to THAT function. I'm not emulating the mmu at this current stage so if you called 0xCB in execute
    the instruction will always be an unimplemented.
    */
    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            0x03 => {
                let value = self.increment_16(self.registers.get_bc());
                self.registers.set_bc(value);
            }
            0x04 => {
                self.registers.b = self.increment(self.registers.b);
            }
            0x05 => {
                self.registers.b = self.decrement(self.registers.b);
            }
            0x13 => {
                let value = self.increment_16(self.registers.get_de());
                self.registers.set_de(value);
            }
            0x14 => {
                self.registers.d = self.increment(self.registers.d);
            }
            0x15 => {
                self.registers.d = self.decrement(self.registers.d);
            }
            0x23 => {
                let value = self.increment_16(self.registers.get_hl());
                self.registers.set_hl(value);
            }
            0x24 => {
                self.registers.h = self.increment(self.registers.h);
            }
            0x25 => {
                self.registers.h = self.decrement(self.registers.h);
            }

            0x0B => {
                let value = self.decrement_16(self.registers.get_bc());
                self.registers.set_bc(value);
            }
            0x0C => {
                self.registers.c = self.increment(self.registers.c);
            }
            0x0D => {
                self.registers.c = self.decrement(self.registers.c);
            }
            0x1B => {
                let value = self.decrement_16(self.registers.get_de());
                self.registers.set_de(value);
            }
            0x1C => {
                self.registers.e = self.increment(self.registers.e);
            }
            0x1D => {
                self.registers.e = self.decrement(self.registers.e);
            }
            0x2B => {
                let value = self.decrement_16(self.registers.get_hl());
                self.registers.set_hl(value);
            }
            0x2C => {
                self.registers.l = self.increment(self.registers.l);
            }
            0x2D => {
                self.registers.l = self.decrement(self.registers.l);
            }
            0x3C => {
                self.registers.a = self.increment(self.registers.a);
            }
            0x3D => {
                self.registers.a = self.decrement(self.registers.a);
            }
            // ADD A, r8
            0x80 => {
                self.registers.a = self.add(self.registers.b);
            }
            0x81 => {
                self.registers.a = self.add(self.registers.c);
            }
            0x82 => {
                self.registers.a = self.add(self.registers.d);
            }
            0x83 => {
                self.registers.a = self.add(self.registers.e);
            }
            0x84 => {
                self.registers.a = self.add(self.registers.h);
            }
            0x85 => {
                self.registers.a = self.add(self.registers.l);
            }
            // TODO: ADD A, HL (0x86)
            0x87 => {
                self.registers.a = self.add(self.registers.a);
            }
            // ADC A, r8
            0x88 => {
                self.registers.a = self.add_carry(self.registers.b);
            }
            0x89 => {
                self.registers.a = self.add_carry(self.registers.c);
            }
            0x8A => {
                self.registers.a = self.add_carry(self.registers.d);
            }
            0x8B => {
                self.registers.a = self.add_carry(self.registers.e);
            }
            0x8C => {
                self.registers.a = self.add_carry(self.registers.h);
            }
            0x8D => {
                self.registers.a = self.add_carry(self.registers.l);
            }
            // TODO: ADC A, HL (0x86)
            0x8F => {
                self.registers.a = self.add_carry(self.registers.a);
            }
            // SUB A, r8
            0x90 => {
                self.registers.a = self.subtract(self.registers.b);
            }
            0x91 => {
                self.registers.a = self.subtract(self.registers.c);
            }
            0x92 => {
                self.registers.a = self.subtract(self.registers.d);
            }
            0x93 => {
                self.registers.a = self.subtract(self.registers.e);
            }
            0x94 => {
                self.registers.a = self.subtract(self.registers.h);
            }
            0x95 => {
                self.registers.l = self.subtract(self.registers.l);
            }
            // TODO: SUB A, HL (0x96)
            0x97 => {
                self.registers.a = self.subtract(self.registers.a);
            }
            // SBC A, r8
            0x98 => {
                self.registers.a = self.subtract_carry(self.registers.b);
            }
            0x99 => {
                self.registers.a = self.subtract_carry(self.registers.c);
            }
            0x9A => {
                self.registers.a = self.subtract_carry(self.registers.d);
            }
            0x9B => {
                self.registers.a = self.subtract_carry(self.registers.e);
            }
            0x9C => {
                self.registers.a = self.subtract_carry(self.registers.h);
            }
            0x9D => {
                self.registers.a = self.subtract_carry(self.registers.l);
            }
            0x9F => {
                self.registers.a = self.subtract(self.registers.a);
            }
            // AND A, r8
            0xA0 => {
                self.registers.a = self.and(self.registers.b);
            }
            0xA1 => {
                self.registers.a = self.and(self.registers.c);
            }
            0xA2 => {
                self.registers.a = self.and(self.registers.d);
            }
            0xA3 => {
                self.registers.a = self.and(self.registers.e);
            }
            0xA4 => {
                self.registers.a = self.and(self.registers.h);
            }
            0xA5 => {
                self.registers.a = self.and(self.registers.l);
            }
            // TODO: AND A, HL
            0xA7 => {
                self.registers.a = self.and(self.registers.a);
            }
            // XOR A, r8
            0xA8 => {
                self.registers.a = self.xor(self.registers.b);
            }
            0xA9 => {
                self.registers.a = self.xor(self.registers.c);
            }
            0xAA => {
                self.registers.a = self.xor(self.registers.d);
            }
            0xAB => {
                self.registers.a = self.xor(self.registers.e);
            }
            0xAC => {
                self.registers.a = self.xor(self.registers.h);
            }
            0xAD => {
                self.registers.a = self.xor(self.registers.l);
            }
            // TODO: XOR A, HL
            0xAF => {
                self.registers.a = self.xor(self.registers.a);
            }
            // OR A, r8
            0xB0 => {
                self.registers.a = self.or(self.registers.b);
            }
            0xB1 => {
                self.registers.a = self.or(self.registers.c);
            }
            0xB2 => {
                self.registers.a = self.or(self.registers.d);
            }
            0xB3 => {
                self.registers.a = self.or(self.registers.e);
            }
            0xB4 => {
                self.registers.a = self.or(self.registers.h);
            }
            0xB5 => {
                self.registers.a = self.or(self.registers.l);
            }
            // TODO: OR A, HL
            0xB7 => {
                self.registers.a = self.or(self.registers.a);
            }
            0xCB => {
                self.execute_cb_prefixed(instruction);
            }
            _ => {
                panic!(
                    "A second plane has hit the tower: Tried to run OPCODE -> {}",
                    instruction
                );
            }
        }
    }
}
