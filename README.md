![bitterly](https://github.com/amcelroy/bitterly/actions/workflows/rust.yml/badge.svg)

# bitterly
Create registers that have an address type that can be different from the register data type. The registers can be created with named bits (`bitfield`) and ranges of bits (`bitrange`).

## Why?
Why not. I was working with an I2C chip that had an address space of u8 and a register data type of u16 and there didn't seem to be anything out there.

This is a piece of my first Rust project and thought others in the embedded space might benefit.

## Testing
This library uses `no_std`. To test on a PC, do the following:
1. `rustup target list`
2. Find the target architecture you want to test on. For example, macOS should 
be x86_64-apple-darwin.
3. run `cargo test --target x86_64-apple-darwin`. Replace `x86_64-apple-darwin` 
with your systems triplet.

## Concepts

###Backing Registers

Backing registers are created using the `register_backer`. The `register_backer` macro creates a simple struct with a name, and address size, and a register size. Backing registers have basic bit manipulation features to clear, set, toggle bits, which may be all that is needed. 

The `register_backer` macro takes 3 arguments:
- Name - Name of the register backer struct
- Address Type - Type of the address (u8, u16, u32, u64, etc.)
- Data Type - Type of the data stored in the register (u8, u16, u32, u64, etc)

An example for the Max17261 Gas Gauge which uses 8-bit addressing and 16-bit register data would have a backing register of:
```
fn main() {
    use bitterly::register_backer;

    register_backer!(I2CRegister, u8, u16);
}

```
which generates:
```
pub struct I2CRegister {
    address: u8,
    contents: u16,
}
impl I2CRegister {
    pub fn new(address: u8, contents: u16) -> Self {
        Self {
            contents: contents,
            address: address,
        }
    }
    pub fn contents(&self) -> u16 {
        self.contents
    }
    pub fn set_bit(&mut self, bit: u16) -> &mut Self {
        self.contents |= 1 << (bit as u16);
        self
    }
    pub fn set_all(&mut self) -> &mut Self {
        self.contents = <u16>::MAX;
        self
    }
    pub fn clear_bit(&mut self, bit: u16) -> &mut Self {
        self.contents &= !(1 << (bit as u16));
        self
    }
    pub fn clear_all(&mut self) -> &mut Self {
        self.contents = 0;
        self
    }
    pub fn toggle_bit(&mut self, bit: u16) -> &mut Self {
        self.contents ^= 1 << (bit as u16);
        self
    }
    pub fn is_set(&self, bit: u16) -> bool {
        self.contents & (1 << (bit as u16)) != 0
    }
    pub fn is_clear(&self, bit: u16) -> bool {
        self.contents & (1 << (bit as u16)) == 0
    }
    pub fn update(&mut self, new_val: u16) -> &mut Self {
        self.contents = new_val;
        self
    }
}
```

### Register and Bitfield
The Max17261 has a lot of registers that have named bits and ranges of bits that would be more intuitive to work with as humans. The status register, Address 0x00h has 13 bits that can be get / set. 

When creating a new register `struct`, it is _critical_ that the new struct contain a `register` field of the type created using the `register_backer` macro.

The `register` macro has 2 arguments:
- Name - Name of the register, must match the `struct` name with the `register` field
- Type of the Backing Register - The type of the backing register, for example `I2CRegister`

Let's create a `StatusRegister` using our backing register created above:

```
fn main() {
    use paste::paste;
    use bitterly::{register, register_backer, bitfield};

    register_backer!(I2CRegister, u8, u16);

    pub struct StatusRegister {
        register: I2CRegister, // Our new register MUST have a register field

        /* add other fields here, though not used by bitterly */
    }

    impl StatusRegister {
        register!(StatusRegister, I2CRegister); //
    }
}

```
This is the least useful, but minimum viable, implemntation of a bitterly register. We can do better using human readable `bitfield` which will provide easy to understand functions for bits in a register.

The `bitfield` macro has 2 arguments:
- Name - Name of the bit
- Bit location - 0 indexed bit associated with the name. For example, a 16-bit register would have viable values of 0 to 15.

Improving on the above example:

```
fn main() {
    use paste::paste;
    use bitterly::{register, register_backer, bitfield};

    register_backer!(I2CRegister, u8, u16);

    pub struct StatusRegister {
        register: I2CRegister, // Our new register MUST have a register field

        /* add other fields here, though not used by bitterly */
    }

    impl StatusRegister {
        register!(StatusRegister, I2CRegister); //
        bitfield!(por, 1); // Power on reset
        /*...*/
        bitfield!(br, 15); // Battery removal
    }
}
```
We added 2 `bitfield`s, let's take a look at the macro expansion:
```
impl StatusRegister {
    pub fn register(&mut self) -> &mut I2CRegister {
        &mut self.register
    }
    pub fn new(reg: I2CRegister) -> StatusRegister {
        StatusRegister { register: reg }
    }
    pub fn por_get(&self) -> bool {
        self.register.is_set(1)
    }
    pub fn por_set(&mut self, val: bool) -> &mut Self {
        if val {
            self.register.set_bit(1);
        } else {
            self.register.clear_bit(1);
        }
        self
    }
    pub fn br_get(&self) -> bool {
        self.register.is_set(15)
    }
    pub fn br_set(&mut self, val: bool) -> &mut Self {
        if val {
            self.register.set_bit(15);
        } else {
            self.register.clear_bit(15);
        }
        self
    }
}
```
The `register` macro provides functions to get the backing register and instantiate a new `StatusRegister`. The `bitfield` macros provide functions to get / set to interact with the named bits.

### Bitrange
The Max17261 also has fields of bits that would be easier to work with using human readable names. It would also be nice if bits were masked for easy clearing and setting of these values. 

The `bitrange` macro takes 4 arguments:
- Name - name of the bitrange
- Upper Bit - Upper bit of the range (0 indexed)
- Lower Bit - Lower bit of the range (0 indexed)
- Type - The type used when getting / setting. This should match the Data Type argument used with `register_backer`.

The RelaxCfg register (0x2A) has 3 named ranges of bits, lets implement it:
```
fn main() {
    use paste::paste;
    use bitterly::{register, register_backer, bitfield, bitrange};

    register_backer!(I2CRegister, u8, u16);

    pub struct RelaxCfgRegister {
        register: I2CRegister, // Our new register MUST have a register field

        /* add other fields here, though not used by bitterly */
    }

    impl RelaxCfgRegister {
        register!(RelaxCfgRegister, I2CRegister); //
        bitrange!(load, 15, 9, u16); // Load
        bitrange!(dv, 8, 4, u16); // Delta voltage
        bitrange!(dt, 3, 0, u16); // Delta time
    }
}
```
