# bitterly
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

### Functions (u32)

- Register8/16/32::new(u8/16/32) -> Self: Create a new Register for manipulation
- set(Bit8/16/32) -> Self: Sets a bit, can chain
- clear(Bit8/16/32 -> Self: Clears a bit, can chain
- set_all() -> Self: Sets all bits to 1, can chain
- clear_all() -> Self: Clears all bits to 0, can chain
- toggle(Bit8/16/32) -> Self: Toggles a bit, can chain
- is_set(Bit8/16/32) -> bool: Checks if bit is set
- is_clear(Bit8/16/32) -> bool: Checks if a bit is clear
- value() -> u8/16/32: Gets the value of the Register

## Useage example for u32
```
let output = Register32::new(0).set(Bit32::_0).set(Bit32::_31).value();
assert_eq!(output, 0b10000000_00000000_00000000_00000001);
```
