#![no_std]

pub use paste::paste;

#[macro_export]
macro_rules! register_maker {
    ($reg_name:ident, $address_type:ty, $reg_type:ty) => {
        #[derive(Copy, Clone)]
        pub struct $reg_name {
            address: $address_type,
            contents: $reg_type,
        }

        impl $reg_name {
            pub fn new(address: $address_type, contents: $reg_type) -> Self {
                Self {
                    contents: contents,
                    address: address,
                }
            }
        
            pub fn value(&self) -> $reg_type {
                self.contents
            }
        
            pub fn set(&mut self, bit: $reg_type) -> &mut Self {
                self.contents |= 1 << (bit as $reg_type);
                self
            }
        
            pub fn set_all(&mut self) -> &mut Self {
                self.contents = <$reg_type>::MAX;
                self
            }
        
            pub fn clear(&mut self, bit: $reg_type) -> &mut Self {
                self.contents &= !(1 << (bit as $reg_type));
                self
            }
        
            pub fn clear_all(&mut self) -> &mut Self {
                self.contents = 0;
                self
            }
        
            pub fn toggle(&mut self, bit: $reg_type) -> &mut Self {
                self.contents ^= 1 << (bit as $reg_type);
                self
            }
        
            pub fn is_set(&self, bit: $reg_type) -> bool {
                self.contents & (1 << (bit as $reg_type)) != 0
            }
        
            pub fn is_clear(&self, bit: $reg_type) -> bool {
                self.contents & (1 << (bit as $reg_type)) == 0
            }
        
            pub fn update(&mut self, new_val: $reg_type) -> &mut Self {
                self.contents = new_val;
                self
            }
        }
    }
}

#[macro_export]
macro_rules! register {
    ($name:ident, $backing_register:ty) => {
        pub fn get(&mut self) -> &mut $backing_register {
            &mut self.register
        }

        pub fn new(reg: $backing_register) -> $name {
            $name {
                register: reg,
            }
        }
    };
}

#[macro_export]
macro_rules! bitfield {
    ($name:ident, $bit:expr, $type:ty) => {
        paste! {
            /// Gets bit $name
            pub fn [<$name _get>](&self) -> bool {
                self.register.is_set($bit)
            }
        }

        paste! {
            pub fn [<$name _set>](&mut self, val: bool) -> &mut Self{
                if val {
                    self.register.set($bit);
                } else {
                    self.register.clear($bit);
                }
        
                self
            }
        }
    };
}

#[macro_export]
macro_rules! bitrange {
    ($name:ident, $end_bit:expr, $start_bit:expr, $type:ty) => {
        paste! {
            pub fn [<$name _get>](&self) -> $type {
                (self.register.value() >> $start_bit) & self.[<$name _mask>]()
            }
        }

        paste! {
            pub fn [<$name _mask>](&self) -> $type {
                ((2 as i128).pow($end_bit + 1 - $start_bit) - 1) as $type
            }
        }

        paste! {
            pub fn [<$name _clear>](&mut self) -> &mut Self {
                self.register.update(self.register.value() & (!self.[<$name _mask>]() << $start_bit));
                self
            }
        }

        paste! {
            pub fn [<$name _set>](&mut self, val: $type) -> &mut Self{
                self.[<$name _clear>](); // Clear bits
                let masked_val = self.[<$name _mask>]() & val; // Mask input
                self.register.update(self.register.value() | (masked_val << $start_bit));
                self
            }
        }
    };
}

