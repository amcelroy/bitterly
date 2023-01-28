#![no_std]

#[derive(Copy, Clone)]
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

macro_rules! register {
    ($reg_name:ident, $reg_type:ty , $bit_type:ty) => {
        #[derive(Copy, Clone)]
        pub struct $reg_name {
            val: $reg_type,
        }

        impl $reg_name {
            pub fn new(val: $reg_type) -> Self {
                Self {
                    val: val,
                }
            }
        
            pub fn value(&self) -> $reg_type {
                self.val
            }
        
            pub fn set(&mut self, bit: $bit_type) -> &mut Self {
                self.val |= 1 << (bit as $reg_type);
                self
            }
        
            pub fn set_all(&mut self) -> &mut Self {
                self.val = <$reg_type>::MAX;
                self
            }
        
            pub fn clear(&mut self, bit: $bit_type) -> &mut Self {
                self.val &= !(1 << (bit as $reg_type));
                self
            }
        
            pub fn clear_all(&mut self) -> &mut Self {
                self.val = 0;
                self
            }
        
            pub fn toggle(&mut self, bit: $bit_type) -> &mut Self {
                self.val ^= 1 << (bit as $reg_type);
                self
            }
        
            pub fn is_set(&self, bit: $bit_type) -> bool {
                self.val & (1 << (bit as $reg_type)) != 0
            }
        
            pub fn is_clear(&self, bit: $bit_type) -> bool {
                self.val & (1 << (bit as $reg_type)) == 0
            }
        
            pub fn update(&mut self, new_val: $reg_type) -> &mut Self {
                self.val = new_val;
                self
            }
        }
    }
}

register!(Register32, u32, Bit32);
register!(Register16, u16, Bit16);
register!(Register8, u8, Bit8);

#[cfg(test)]
mod tests {
    #[test]
    fn test_u32() {
        use crate::{Bit32, Register32};

        let mut register = Register32::new(0);
        register.set(Bit32::_1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.value(), 0b00000000_00000000_00000000_0000010);

        register.clear(Bit32::_1);

        assert_eq!(register.value(), 0);

        register.set(Bit32::_0).set(Bit32::_1).set(Bit32::_2).set(Bit32::_3);

        assert_eq!(register.value(), 0xF);

        assert_eq!(Register32::new(1).clear(Bit32::_0).value(), 0);

        assert_eq!(Register32::new(0).set_all().value(), u32::MAX);

        assert_eq!(Register32::new(2).set(Bit32::_0).value(), 3);

        assert_eq!(Register32::new(4).is_set(Bit32::_2), true);

        assert_eq!(Register32::new(0).is_clear(Bit32::_0), true);

        assert_eq!(register.is_clear(Bit32::_5), true);

        register.toggle(Bit32::_5);

        assert_eq!(register.is_set(Bit32::_5), true);

        register.toggle(Bit32::_5);

        assert_eq!(register.is_set(Bit32::_5), false);

    }

    #[test]
    fn test_u16() {
        use crate::{Bit16, Register16};

        let mut register = Register16::new(0);
        let r = register.set(Bit16::_1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.value(), 0b00000000_0000010);

        register.clear(Bit16::_1);

        assert_eq!(register.value(), 0);

        register.set(Bit16::_0).set(Bit16::_1).set(Bit16::_2).set(Bit16::_3);

        assert_eq!(register.value(), 0xF);

        assert_eq!(Register16::new(1).clear(Bit16::_0).value(), 0);

        assert_eq!(Register16::new(0).set_all().value(), u16::MAX);

        assert_eq!(Register16::new(2).set(Bit16::_0).value(), 3);

        assert_eq!(Register16::new(4).is_set(Bit16::_2), true);

        assert_eq!(register.is_clear(Bit16::_5), true);

        register.toggle(Bit16::_5);

        assert_eq!(register.is_set(Bit16::_5), true);

        register.toggle(Bit16::_5);

        assert_eq!(register.is_set(Bit16::_5), false);
    }

    #[test]
    fn test_u8() {
        use crate::{Bit8, Register8};

        let mut register = Register8::new(0);
        register.set(Bit8::_1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.value(), 0b0000010);

        register.clear(Bit8::_1);

        assert_eq!(register.value(), 0);

        register.set(Bit8::_0).set(Bit8::_1).set(Bit8::_2).set(Bit8::_3);

        assert_eq!(register.value(), 0xF);

        assert_eq!(Register8::new(1).clear(Bit8::_0).value(), 0);

        assert_eq!(Register8::new(0).set_all().value(), u8::MAX);

        assert_eq!(Register8::new(2).set(Bit8::_0).value(), 3);

        assert_eq!(Register8::new(4).is_set(Bit8::_2), true);

        assert_eq!(register.is_clear(Bit8::_5), true);

        register.toggle(Bit8::_5);

        assert_eq!(register.is_set(Bit8::_5), true);

        register.toggle(Bit8::_5);

        assert_eq!(register.is_set(Bit8::_5), false);
    }

}

