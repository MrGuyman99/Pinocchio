use super::registers;

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

pub struct CPU {
    pub registers: registers::Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    pub fn test() -> Self {
        CPU {
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
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

    fn step(&mut self) {
        let mut instruction_byte: u8 = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        self.pc = self.execute(prefixed, instruction_byte);
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // Move program counter by 3 since jump is 3 bytes wide
            self.pc.wrapping_add(3)
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

    pub fn execute_cb_prefixed(&mut self, instruction: u8) -> u16 {
        match instruction {
            0x00 => {
                self.registers.b = self.rotate_left_carry(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x01 => {
                self.registers.c = self.rotate_left_carry(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x02 => {
                self.registers.d = self.rotate_left_carry(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x03 => {
                self.registers.e = self.rotate_left_carry(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x04 => {
                self.registers.h = self.rotate_left_carry(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x05 => {
                self.registers.l = self.rotate_left_carry(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: RLC HL
            0x07 => {
                self.registers.a = self.rotate_left_carry(self.registers.a);
                self.pc.wrapping_add(1)
            }
            0x08 => {
                self.registers.b = self.rotate_right_carry(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x09 => {
                self.registers.c = self.rotate_right_carry(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x0A => {
                self.registers.d = self.rotate_right_carry(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x0B => {
                self.registers.e = self.rotate_right_carry(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x0C => {
                self.registers.h = self.rotate_right_carry(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x0D => {
                self.registers.l = self.rotate_right_carry(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: RRC HL
            0x0F => {
                self.registers.a = self.rotate_right_carry(self.registers.a);
                self.pc.wrapping_add(1)
            }
            0x10 => {
                self.registers.b = self.rotate_left(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x11 => {
                self.registers.c = self.rotate_left(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x12 => {
                self.registers.d = self.rotate_left(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x13 => {
                self.registers.e = self.rotate_left(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x14 => {
                self.registers.h = self.rotate_left(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x15 => {
                self.registers.l = self.rotate_left(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: RL HL
            0x17 => {
                self.registers.a = self.rotate_left(self.registers.a);
                self.pc.wrapping_add(1)
            }
            0x18 => {
                self.registers.b = self.rotate_right(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x19 => {
                self.registers.c = self.rotate_right(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x1A => {
                self.registers.d = self.rotate_right(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x1B => {
                self.registers.e = self.rotate_right(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x1C => {
                self.registers.h = self.rotate_right(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x1D => {
                self.registers.l = self.rotate_right(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: RR HL
            0x1F => {
                self.registers.a = self.rotate_right(self.registers.a);
                self.pc.wrapping_add(1)
            }
            0x30 => {
                self.registers.b = self.swap(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x31 => {
                self.registers.c = self.swap(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x32 => {
                self.registers.d = self.swap(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x33 => {
                self.registers.e = self.swap(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x34 => {
                self.registers.h = self.swap(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x35 => {
                self.registers.l = self.swap(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: Swap HL
            0x37 => {
                self.registers.a = self.swap(self.registers.a);
                self.pc.wrapping_add(1)
            }
            _ => {
                panic!("Instruction {} not implemented!", instruction);
            }
        }
    }

    pub fn execute(&mut self, prefixed: bool, instruction: u8) -> u16 {
        if prefixed {
            self.execute_cb_prefixed(instruction);
        }
        match instruction {
            0x03 => {
                let value = self.increment_16(self.registers.get_bc());
                self.registers.set_bc(value);
                self.pc.wrapping_add(1)
            }
            0x04 => {
                self.registers.b = self.increment(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x05 => {
                self.registers.b = self.decrement(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x09 => {
                let value = self.add_hl(self.registers.get_bc());
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            }
            0x13 => {
                let value = self.increment_16(self.registers.get_de());
                self.registers.set_de(value);
                self.pc.wrapping_add(1)
            }
            0x14 => {
                self.registers.d = self.increment(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x15 => {
                self.registers.d = self.decrement(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x19 => {
                let value = self.add_hl(self.registers.get_de());
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            }
            0x23 => {
                let value = self.increment_16(self.registers.get_hl());
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            }
            0x24 => {
                self.registers.h = self.increment(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x25 => {
                self.registers.h = self.decrement(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x29 => {
                let value = self.add_hl(self.registers.get_hl());
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            }
            // ADD HL, SP
            0x0B => {
                let value = self.decrement_16(self.registers.get_bc());
                self.registers.set_bc(value);
                self.pc.wrapping_add(1)
            }
            0x0C => {
                self.registers.c = self.increment(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x0D => {
                self.registers.c = self.decrement(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x1B => {
                let value = self.decrement_16(self.registers.get_de());
                self.registers.set_de(value);
                self.pc.wrapping_add(1)
            }
            0x1C => {
                self.registers.e = self.increment(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x1D => {
                self.registers.e = self.decrement(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x2B => {
                let value = self.decrement_16(self.registers.get_hl());
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            }
            0x2C => {
                self.registers.l = self.increment(self.registers.l);
                self.pc.wrapping_add(1)
            }
            0x2D => {
                self.registers.l = self.decrement(self.registers.l);
                self.pc.wrapping_add(1)
            }
            0x3C => {
                self.registers.a = self.increment(self.registers.a);
                self.pc.wrapping_add(1)
            }
            0x3D => {
                self.registers.a = self.decrement(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // ADD A, r8
            0x80 => {
                self.registers.a = self.add(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x81 => {
                self.registers.a = self.add(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x82 => {
                self.registers.a = self.add(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x83 => {
                self.registers.a = self.add(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x84 => {
                self.registers.a = self.add(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x85 => {
                self.registers.a = self.add(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: ADD A, HL (0x86)
            0x87 => {
                self.registers.a = self.add(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // ADC A, r8
            0x88 => {
                self.registers.a = self.add_carry(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x89 => {
                self.registers.a = self.add_carry(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x8A => {
                self.registers.a = self.add_carry(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x8B => {
                self.registers.a = self.add_carry(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x8C => {
                self.registers.a = self.add_carry(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x8D => {
                self.registers.a = self.add_carry(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: ADC A, HL (0x86)
            0x8F => {
                self.registers.a = self.add_carry(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // SUB A, r8
            0x90 => {
                self.registers.a = self.subtract(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x91 => {
                self.registers.a = self.subtract(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x92 => {
                self.registers.a = self.subtract(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x93 => {
                self.registers.a = self.subtract(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x94 => {
                self.registers.a = self.subtract(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x95 => {
                self.registers.l = self.subtract(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: SUB A, HL (0x96)
            0x97 => {
                self.registers.a = self.subtract(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // SBC A, r8
            0x98 => {
                self.registers.a = self.subtract_carry(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0x99 => {
                self.registers.a = self.subtract_carry(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0x9A => {
                self.registers.a = self.subtract_carry(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0x9B => {
                self.registers.a = self.subtract_carry(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0x9C => {
                self.registers.a = self.subtract_carry(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0x9D => {
                self.registers.a = self.subtract_carry(self.registers.l);
                self.pc.wrapping_add(1)
            }
            0x9F => {
                self.registers.a = self.subtract(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // AND A, r8
            0xA0 => {
                self.registers.a = self.and(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0xA1 => {
                self.registers.a = self.and(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0xA2 => {
                self.registers.a = self.and(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0xA3 => {
                self.registers.a = self.and(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0xA4 => {
                self.registers.a = self.and(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0xA5 => {
                self.registers.a = self.and(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: AND A, HL
            0xA7 => {
                self.registers.a = self.and(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // XOR A, r8
            0xA8 => {
                self.registers.a = self.xor(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0xA9 => {
                self.registers.a = self.xor(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0xAA => {
                self.registers.a = self.xor(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0xAB => {
                self.registers.a = self.xor(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0xAC => {
                self.registers.a = self.xor(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0xAD => {
                self.registers.a = self.xor(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: XOR A, HL
            0xAF => {
                self.registers.a = self.xor(self.registers.a);
                self.pc.wrapping_add(1)
            }
            // OR A, r8
            0xB0 => {
                self.registers.a = self.or(self.registers.b);
                self.pc.wrapping_add(1)
            }
            0xB1 => {
                self.registers.a = self.or(self.registers.c);
                self.pc.wrapping_add(1)
            }
            0xB2 => {
                self.registers.a = self.or(self.registers.d);
                self.pc.wrapping_add(1)
            }
            0xB3 => {
                self.registers.a = self.or(self.registers.e);
                self.pc.wrapping_add(1)
            }
            0xB4 => {
                self.registers.a = self.or(self.registers.h);
                self.pc.wrapping_add(1)
            }
            0xB5 => {
                self.registers.a = self.or(self.registers.l);
                self.pc.wrapping_add(1)
            }
            // TODO: OR A, HL
            0xB7 => {
                self.registers.a = self.or(self.registers.a);
                self.pc.wrapping_add(1)
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
