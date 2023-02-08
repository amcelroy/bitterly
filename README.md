# bitterly
Create registers that have an address type that can be different from the register data type. 
The registers can be created with named bits (bitfield) and ranges of bits (bitrange).

## Why?
Why not. I was working with an I2C chip that had an address space of u8 and a register
data type of u16 and there didn't seem to be anything out there.

This is a piece of my first Rust project and thought others in the embedded space might
benefit.



