/*
a is our accumlator register
bc, de, and hl are 16 bit registers made of two 8 bit registers
f is our flag register
Basis for this code can be found here -> https://rylev.github.io/DMG-01/public/book/cpu/registers.html#cpu-registers
*/

const ZERO_POSITION: u8 = 7;
const SUBTRACTION_POSITION: u8 = 6;
const HALF_CARRY_POSITION: u8 = 5;
const CARRY_POSITION: u8 = 4;

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

// If, for example, flag.zero = true we return 1 + ZERO_POSITION to store it
// in the right bit of our u8 flag
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACTION_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACTION_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CPU::registers::Registers;

    #[test]
    fn wide_registers() {
        let mut reg = Registers {
            a: 0x12,
            b: 0x34,
            c: 0x45,
            d: 0x56,
            e: 0x67,
            f: 0x33,
            h: 0x78,
            l: 0x89,
        };

        // Testing getting methods
        assert_eq!(reg.get_bc(), 0x3445);
        assert_eq!(reg.get_de(), 0x5667);
        assert_eq!(reg.get_hl(), 0x7889);

        reg.set_bc(16);
        reg.set_de(78);
        reg.set_hl(90);

        // Testing setting methods
        assert_eq!(reg.get_bc(), 16);
        assert_eq!(reg.get_de(), 78);
        assert_eq!(reg.get_hl(), 90);
    }
}
