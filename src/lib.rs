#![no_std]

pub use paste::paste;

/// # Bitterly
/// 
/// Bitterly is a simple, macro based Rust library used to generate a generic `Register` struct
/// that is made of a user definable address size, and a user definable content type. For example,
/// when dealing with a max17261 chip, the registers are 8-bit addressible and contain 16-bit
/// values. A Cortex-M may have addresses of 32-bit with register sizes of 32-bits.
/// 
/// In the simplest case with just bits to get / set / clear, use `register_maker!`.
/// 
/// The `paste` macro is used to help generate the getters and setters.
/// 
/// The macro inputs are:
/// `reg_name` - Name of the backing registers
/// `address_type` - The size of the address. 
/// `reg_type` - The size of the data stored in the register.
/// 
/// # Example
/// ```
/// use bitterly::register_backer;
/// 
/// pub fn main() {
///     register_backer!(I2CRegister, u8, u16);
/// 
///     let mut status = I2CRegister::new(0, 0);
/// 
///     let mut value = status.set(0).set(2).set(4).set(6).value(); 
///     assert_eq!(value, 0x55, "Value should be 0x55");
/// 
///     value = status.clear(2).clear(4).value();
///     assert_eq!(value, 0x41, "Value should be 0x41");
/// 
///     value = status.update(0x0FF0).value();
///     assert_eq!(value, 0x0FF0, "Value should be 0x0FF0");
/// 
///     assert_eq!(status.toggle(0).value(), 0x0FF1, "Value should be 0xFF1");
///     assert_eq!(status.is_set(0), true, "Bit 0 is_set should be true");
///     assert_eq!(status.toggle(0).value(), 0x0FF0, "Value should be 0xFF0");
///     assert_eq!(status.is_clear(0), true, "Bit 0 is_clear should be true");
/// }
/// 
/// ```
#[macro_export]
macro_rules! register_backer {
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

/// Generates commonly uses functions to access the backing register.
/// 
/// The `register` macro should be used for a register struct created by `register_backer`.
/// This macro is meant to be used to extend a register struct using `bitfields` and
/// `bitrange` macros to create easy to use registers with intuitive bit names and 
/// grouping of bits
/// 
/// # Example
/// ```
/// use paste::paste;
/// use bitterly::{register_backer, register, bitfield, bitrange};
/// 
/// fn main() {
///     register_backer!(I2CRegister, u8, u16);
/// 
///     pub struct StatusRegister{
///         register: I2CRegister,
///         // Other stuff goes here
///     }
/// 
///     impl StatusRegister {
///         register!(StatusRegister, I2CRegister);
///         bitfield!(por, 1); // Power on reset - bit 1
///         bitrange!(example_field, 14, 10, u16);   
///     }
/// 
///     let register_address = 0x00;
///     let register_init = 0;    
/// 
///     let mut status = StatusRegister::new(I2CRegister::new(register_address, register_init));
/// 
///     assert_eq!(status.por_set(true).get().value(), 0x2, "Status register should be 1");
///     assert_eq!(status.por_get(), true, "Status register bit 1 should be true");
///     assert_eq!(status.por_set(false).get().value(), 0x0, "Status register should be 0");
///     assert_eq!(status.por_get(), false, "Status register bit 1 should be false");
/// 
///     assert_eq!(status.por_set(true).example_field_set(0x3).get().value(), 0xC02, "Example Field should be 3 (bit shifted 10), por bit should be 1 (bit shifted 1)");
///     assert_eq!(status.example_field_clear().get().value(), 0x2, "Only the power on reset bit should be true");
/// }
/// 
/// ```

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

/// Used to create a single named bit.
/// 
/// The macro inputs are:
/// `name` - Identity of the bitfield
/// `bit` - Bit number. Bit 0 should be 0, Bit 1 should 1, etc.
/// 
/// # Example
/// ```
/// use paste::paste;
/// use bitterly::{register_backer, register, bitfield, bitrange};
/// 
/// fn main() {
///     register_backer!(I2CRegister, u8, u16);
/// 
///     pub struct StatusRegister{
///         register: I2CRegister,
///         // Other stuff goes here
///     }
/// 
///     impl StatusRegister {
///         register!(StatusRegister, I2CRegister);
///         bitfield!(por, 1); // Power on reset - bit 1  
///         bitfield!(bst, 3); // Battery status - Bit 3
///         bitfield!(br, 15); // Battery Removal - bit 15
///     }
/// 
///     let register_address = 0x00;
///     let register_init = 0;    
/// 
///     let mut status = StatusRegister::new(I2CRegister::new(register_address, register_init));
/// 
///     assert_eq!(status.por_set(true).bst_set(true).br_set(true).get().value(), 0x800A, "Status register should be 1");
/// }
/// 
/// ```
#[macro_export]
macro_rules! bitfield {
    ($name:ident, $bit:expr) => {
        paste! {
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

/// Used to create a masked range of named bits.
/// 
/// The macro inputs are:
/// `name` - Name of the bitfield
/// `end_bit` - Highest bit of the bitfield
/// `start_bit` - Starting bit of the bitfield
/// `register type` - This should match the $reg_type of the backing register
/// 
/// # Example
/// ```
/// use paste::paste;
/// use bitterly::{register_backer, register, bitfield, bitrange};
/// 
/// fn main() {
///     register_backer!(I2CRegister, u8, u16);
/// 
///     pub struct ModelCfgRegister{
///         register: I2CRegister,
///         // Other stuff goes here
///     }
/// 
///     impl ModelCfgRegister {
///         register!(ModelCfgRegister, I2CRegister);
///         bitfield!(refresh, 15); // Refresh status bit - bit 15  
///         bitfield!(r100, 13); // Resistor (100k / 10k) - Bit 13
///         bitfield!(vchg, 10); // Voltage charge - (> 4.2V / <= 4.2V)
///         bitrange!(modelid, 7, 4, u16); // Model ID - See Max17261 for more info
///     }
/// 
///     let register_address = 0xDB;
///     let register_init = 0x8400;    
/// 
///     let mut modelcfg = ModelCfgRegister::new(I2CRegister::new(register_address, register_init));
/// 
///     assert_eq!(modelcfg.refresh_get(), true, "Refresh should be 1");
///     
///     assert_eq!(modelcfg.modelid_mask(), 0x00F0, "Model ID mask should be 0x00F0");
///     assert_eq!(modelcfg.modelid_set(0x2).get().value(), 0x8420, "ModelCfg register value should be 0x8420");
///     assert_eq!(modelcfg.modelid_clear().get().value(), 0x8400, "ModelCfg register value should be 0x8420");
/// }
/// 
/// ```
#[macro_export]
macro_rules! bitrange {
    ($name:ident, $end_bit:expr, $start_bit:expr, $type:ty) => {
        paste! {
            pub fn [<$name _get>](&self) -> $type {
                (self.register.value() & self.[<$name _mask>]()) >> $start_bit
            }
        }

        paste! {
            pub fn [<$name _mask>](&self) -> $type {
                (((2 as $type).pow($end_bit + 1 - $start_bit) - 1) << $start_bit) as $type
            }
        }

        paste! {
            pub fn [<$name _clear>](&mut self) -> &mut Self {
                self.register.update(self.register.value() & !self.[<$name _mask>]());
                self
            }
        }

        paste! {
            pub fn [<$name _set>](&mut self, val: $type) -> &mut Self{
                self.[<$name _clear>](); // Clear bits
                let masked_val = self.[<$name _mask>]() & (val << $start_bit); // Mask input
                self.register.update(self.register.value() | masked_val);
                self
            }
        }
    };
}

