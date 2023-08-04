![bitterly](https://github.com/amcelroy/bitterly/actions/workflows/rust.yml/badge.svg)

# bitterly
Creates a `peripheral!` with with human readable register accessors, bit fields, 
and bit ranges. `peripheral!` registers are stored in an allocated memory
and should be used as an intermediary to interface to a peripheral. For example,
and I2C device could be modeled and implemented, with data fetched from the
I2C device, stored in a `bitterly` created `peripheral!`, manipulated in code, and
sent back to the I2C device. It is up to the user to get data into and out of
the `peripheral!` memory using a HAL or some other approach.

The goal is to reduce errors interacting with peripherals by constraing the
way that programmers interact with the registers and memory. It is hopefully
easier, safer, and more human readable to use `get_BatDet()` to determine if a 
battery is detected.

## Why?
I first tried using `tock-registers` but was limited by the requirement that
the registers be contiguous, memory-mapped, and have the same address size as
the data stored there. The Max17261, for example, has 8-bit memory addressing
but 16-bit data which can't be created with `tock-registers`. I also need to 
gain experience with the Rust macro system, and this project was a good fit.

## Testing
This library uses `no_std`. To test on a PC, do the following:
1. `rustup target list`
2. Find the target architecture you want to test on. For example, macOS should 
be x86_64-apple-darwin.
3. run `cargo test --target x86_64-apple-darwin`. Replace `x86_64-apple-darwin` 
with your systems triplet.

## Using Bitterly

Bitterly uses itself, but also the `paste` library. `paste` is used to generate
the named getters and setters. If you are creating a new Bitterly peripheral, 
the following use statement is helpful to get start:

```
use bitterly::{
    bitfield, bitrange, bitrange_enum_values, bitrange_raw, peripheral, register,
    register_backer,
};
use paste::paste;
```

## Concepts

### Backing Registers

Backing registers are created using the `register_backer`. The `register_backer` 
macro creates a simple struct with a name, and address size, and a register 
size. Backing registers have basic bit manipulation features to clear, set, 
toggle bits. Ranges of bits can be set using `BitRange` and `mask`, `clear_range`
, `set_Range`, and `get_range`.

The `register_backer` macro takes 2 arguments:
- `Name`: Name of the register backer struct
- `Address Type`: Type of the address (u8, u16, u32, u64, etc.)

An example for the Max17261 Gas Gauge which uses 8-bit addressing and 16-bit 
register data would have a backing register of:
```
fn main() {
    use bitterly::register_backer;

    register_backer!(Register, u16);
}
```

### Peripheral

A peripheral represents the device that contains the registers, like the 
Max17261 or Max14748. This is created using the `peripheral!` macro and takes
the following arguments:
- `Name`: The name of the peripheral struct to be created, for example `Max14748`
- `Number of registers`: This is used to allocate the memory used
to store all of the registers. 
- `Register Map`: This is a list of tuples that are the register name, the 
address, and the index of the register in the allocated array. 
This is used to create an `enum` that maps the registers created with 
`register!` (see below) back to an address offset and index offset. 
Many times, the register address and index will match, but if you choose not 
to implement reserved registers or have a non-zero starting address, this will 
be helpful.

__Note__: It is important that the name in the tuples matches the name of the
registers created using `register!`. For example:
```
fn main() {
    use bitterly::{register, register_backer, peripheral};

    register_backer!(I2CRegister, u8);

    peripheral!(
        Max14748,
        0x0A, // 7-bit I2C address
        2, // Number of registers implimented
        [
            // (Register name, register address, register index) 
            (ChipId, 0x00, 0),

            // (Register name, register address, register index) 
            (ChipRev, 0x01, 1) // Note: don't add a comma for the last item in list
        ]
    );

    /* ... more code ... */
    register!(chipid); // <-- nope
    register!(ChipId); // <-- yep, matches the peripheral! tuple name above
    /* bitfields / bitranges for ChipRev (if any) */

    register!(Chiprev); // <-- nope
    register!(ChipRev); // <-- yep, matches the peripheral! tuple name above
    /* bitfields / bitranges for ChipRev (if any) */
}
```

__Note 2__: The `Number of Registers` input to the macro can be less than the
tuple list, but accessing named registers will cause an out of bounds memory 
panic. The Number of Registers can also be greater than the tuple list, which 
will just allocate more registers that can't be easily accessed.


### Registers

A register is defined using the `register!` macro, and again, should be the same
name as those used in the `Register Map` tuple in the `peripheral!` macro. Once
a register is defined, the details of the register can be implemented.

Many devices have reserved registers. I choose to implement these as Reserved
followed by the address. For the Max14748, 0x08 is reserved and would be:
`register!(Reserved0x08)`.

A `register!` has:
- `contents()`: returns the value of the register in memory
- `address()`: returns the address of the register
- `update(value)`: Sets the value of a register in memory
- `clear()`: Sets the value of the register to 0 in memory

### Bitfields

The simplest way to interact with a register is a `bitfield!` which represents
a binary, single bit that has a name. 

```
fn main() {
    use bitterly::{register, register_backer, peripheral};

    register_backer!(I2CRegister, u8);

    peripheral!(
        Max14748,
        3,
        [
            (ChipId, 0x00, 0),
            (ChipRev, 0x01, 1),
            (DevStatus1, 0x02, 2) // Note: don't add a comma for the last item in list
        ]
    );

    /* ... more code ... */
    register!(ChipId);
    register!(ChipRev); 

    register!(DevStatus1);
    bitfield!(DevStatus1, SysFit, 7);
    bitfield!(DevStatus1, ChgInOvp, 6);
    bitfield!(DevStatus1, ILim, 5);
    bitfield!(DevStatus1, VSysReg, 4);
    bitfield!(DevStatus1, ThrmSd150, 3);
    bitfield!(DevStatus1, ThrmSd120, 2);
    bitfield!(DevStatus1, BatDet, 1);
    bitfield!(DevStatus1, WbChg, 0);

    let max14748 = Max14748::new();

    /* 
    Use I2C to read and update registers using the HAL and register!
    functions such as address() and update(...).
    */

    let bat_det = max14748.DevStatus1().get_BatDet();

    // Use that info as needed

}
```

### Bitrange and enumerated Bitranges

Registers often have a range of bits that represent some state. Bitterly handles
this using the `bitrange_enum_values!` macro. This macro creates an `enum` of 
a type (`u8`, `u16`, etc) and a list of tuples of the `enum` name and value.
The `bitrange_enum_values!` macro must be used before using the `bitrange!` 
macro. 

The `bitrange_enum_values!` has the following inputs:
- `Enum Name`: Name of the macro generated enum. For example, `ChgStatusEnum`.
- `type`: Type the enum represents, for example `u8`, `u16`, etc.
- `(name, value)`: List of tuples that are the named enum values and the numeric
value. 

For example, the `ChgStatus` register of the Max14748, register `0x05` would
look like:
```
    bitrange_enum_values!(
        ChgStatusEnum,
        u8,
        [
            (Off, 0),
            (Suspended, 1),
            (PreChg, 2),
            (FastChargeI, 3),
            (FastChargeV, 4),
            (MaintainInProgress, 5),
            (MaintainComplete, 6),
            (Fault, 7),
            (FaultSuspended, 8) // Note: don't add a comma for the last item in list
        ]
    );
```
This creates functions that can set a range of bits and fetch values using
typed values.
```
let value: ChgStatusEnum = max14748.ChgStatus().get_ChgStat().unwrap();
match value {
    ChgStatusEnum::Off => {},
    ChgStatusEnum::Suspended => {}
    /* etc. */
    _ => {}
}
```

The `bitrange_enum_values!` are used with the `bitrange!` macro. The `bitrange!`
macro has the following inputs:
- `Register Name`: Should match that used in the `register!` macro.
- `Name of the bitrange`: Name of the range of bits, used to name the get / set
function.
- `Upper Bit`: High bit of the range
- `Lower Bit`: Low bit of the range
- `Enum`: The `bitrange_enum_values!` for this range. 

For example:
```
    register!(AiclCfg3);
    bitrange_enum_values!(
        AiclTBlkEnum,
        u8,
        [
            (_0_500ms, 0b00),
            (_1_0s, 0b01),
            (_1_5s, 0b10),
            (_5_0s, 0b11) // Note: don't add a comma for the last item in list
        ]
    );
    bitrange_enum_values!(
        AiclTStepEnum,
        u8,
        [
            (_100ms, 0b00),
            (_200ms, 0b01),
            (_300ms, 0b10),
            (_500ms, 0b11) // Note: don't add a comma for the last item in list
        ]
    );
    bitfield!(AiclCfg3, BypDeb, 4);
    bitrange!(AiclCfg3, AiclTBlk, 3, 2, AiclTBlkEnum);
    bitrange!(AiclCfg3, AiclTStep, 1, 0, AiclTStepEnum);
```

Some bitranges aren't really fit for an enum, for example the `ChipRev` register
of the Max14748. In this case, use `bitrange_raw!` which uses a type instead of
an enum.

```
register!(ChipRev);
bitrange_raw!(ChipRev, RevH, 7, 4, u8);
bitrange_raw!(ChipRev, RevL, 3, 0, u8);
```

Some bitranges represent an actual, quantized measurement, such as voltage, current, etc.
These can be handled by `bitrange_quantized!`, which currently produces and accepts `f32`
quantization values for the `get_` and `set_` functions.

The `bitrange_quantized!` macro expects:
- `Register Name`: Should match that used in the `register!` macro.
- `Name of the bitrange`: Name of the range of bits, used to name the get / set
function.
- `Upper Bit`: High bit of the range
- `Lower Bit`: Low bit of the range
- `type`: The underlying type to quantize. This used for error checking only. For example,
if the register is 16-bit and split into two quantized 8-bit values, this should be `u8` or
`i8` to check if the `set_` input value is `type::MIN <= input value <= type::MAX`.
- `Quantization value`: `f32` value used to convert the unquantized to quantized data.  

```
// 16-bit backing register
register_backer!(Register, u16);

/** configure peripheral here **/

register!(MaxMinVolt);
bitrange_quantized!(MaxMinVolt, MaxVCell, 15, 8, u8, 0.02); // 20mv resolution
bitrange_quantized!(MaxMinVolt, MinVCell, 7, 0, u8, 0.02); // 20mv resolution
```




