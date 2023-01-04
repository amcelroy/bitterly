#![no_std]

pub enum Bit8 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

#[derive(Copy, Clone)]
pub struct Register8 {
    val: u8,
}

impl Register8 {
    pub fn new(val: u8) -> Self {
        Self {
            val: val,
        }
    }

    pub fn value(self) -> u8 {
        self.val
    }

    pub fn set(mut self, bit: Bit8) -> Self {
        self.val |= 1 << (bit as u8);
        self
    }

    pub fn set_all(mut self) -> Self {
        self.val = u8::MAX;
        self
    }

    pub fn clear(mut self, bit: Bit8) -> Self {
        self.val &= !(1 << (bit as u8));
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.val = 0;
        self
    }

    pub fn toggle(mut self, bit: Bit8) -> Self {
        self.val ^= 1 << (bit as u8);
        self
    }
}


pub enum Bit16 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
}

#[derive(Copy, Clone)]
pub struct Register16 {
    val: u16,
}

impl Register16 {
    pub fn new(val: u16) -> Self {
        Self {
            val: val,
        }
    }

    pub fn value(self) -> u16 {
        self.val
    }

    pub fn set(mut self, bit: Bit16) -> Self {
        self.val |= 1 << (bit as u16);
        self
    }

    pub fn set_all(mut self) -> Self {
        self.val = u16::MAX;
        self
    }

    pub fn clear(mut self, bit: Bit16) -> Self {
        self.val &= !(1 << (bit as u16));
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.val = 0;
        self
    }

    pub fn toggle(mut self, bit: Bit16) -> Self {
        self.val ^= 1 << (bit as u16);
        self
    }
}

pub enum Bit32 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
    _26,
    _27,
    _28,
    _29,
    _30,
    _31,
}

#[derive(Copy, Clone)]
pub struct Register32 {
    val: u32,
}

impl Register32 {
    pub fn new(val: u32) -> Self {
        Self {
            val: val,
        }
    }

    pub fn value(self) -> u32 {
        self.val
    }

    pub fn set(mut self, bit: Bit32) -> Self {
        self.val |= 1 << (bit as u32);
        self
    }

    pub fn set_all(mut self) -> Self {
        self.val = u32::MAX;
        self
    }

    pub fn clear(mut self, bit: Bit32) -> Self {
        self.val &= !(1 << (bit as u32));
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.val = 0;
        self
    }

    pub fn toggle(mut self, bit: Bit32) -> Self {
        self.val ^= 1 << (bit as u32);
        self
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_u32() {
        use crate::{Bit32, Register32};

        let mut register = Register32::new(0);
        register = register.set(Bit32::_1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.value(), 0b00000000_00000000_00000000_0000010);

        assert_eq!(register.clear(Bit32::_1).value(), 0);

        assert_eq!(Register32::new(1).clear(Bit32::_0).value(), 0);

        assert_eq!(Register32::new(0).set_all().value(), u32::MAX);

        assert_eq!(Register32::new(2).set(Bit32::_0).value(), 3);
    }

    #[test]
    fn test_u16() {

    }

    #[test]
    fn test_u8() {

    }

}

