# bitter
Simple Rust bit maniuplation for u8, u16, and u32 types

## Why?
Why not. I needed some really simple code for an embedded project to set
multiple bits in registers and decided to publish it. It is also my first public
Rust project.

Nothing fancy, no dependancies.

## Description
bitter contains Register8, Register16, and Register32 (for u8, u16, and u32, 
respectively). Each type has an associated enum (Bit8, Bit16, Bit32) with 
constrained bitfields that can be set, cleared, or toggled. When done, use the
.value() function to fetch the value.


## Useage example for u32
let output = Register32::new(0).set(Bit32::_0).set(Bit32::_31).value();
assert_eq!(output, 0b10000000_00000000_00000000_00000001);