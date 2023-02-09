#[cfg(test)]
mod tests {
    #[test]
    fn test_u32() {
        use bitterly::register_backer;

        register_backer!(Register32, u32, u32);

        let mut register = Register32::new(0, 0);
        register.set_bit(1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.contents(), 0b00000000_00000000_00000000_0000010);

        register.clear_bit(1);

        assert_eq!(register.contents(), 0);

        register.set_bit(0).set_bit(1).set_bit(2).set_bit(3);

        assert_eq!(register.contents(), 0xF);

        assert_eq!(Register32::new(0, 1).clear_bit(0).contents(), 0);

        assert_eq!(Register32::new(0, 0).set_all().contents(), u32::MAX);

        assert_eq!(Register32::new(0, 2).set_bit(0).contents(), 3);

        assert_eq!(Register32::new(0, 4).is_set(2), true);

        assert_eq!(Register32::new(0, 0).is_clear(0), true);

        assert_eq!(register.is_clear(5), true);

        register.toggle_bit(5);

        assert_eq!(register.is_set(5), true);

        register.toggle_bit(5);

        assert_eq!(register.is_set(5), false);

    }

    #[test]
    fn test_bitfields() {
        use paste::paste;
        use bitterly::{register, register_backer, bitfield, bitrange};

        register_backer!(Register16, u8, u16);

        pub struct TestRegister {
            register: Register16,
        }

        impl TestRegister {
            register!(TestRegister, Register16);

            bitfield!(twelve, 12);
            bitrange!(vempty, 15, 13, u16);
            bitrange!(middle_byte, 11, 4, u16);
        }

        let mut test = TestRegister::new(Register16::new(0, 0));

        assert_eq!(test.vempty_mask(), 0xE000, "Mask for bits 15, 14, 13 should be 0xE000");

        assert_eq!(test.middle_byte_mask(), 0x0FF0, "Mask for bits 11..4 should be 0x00FF");

        test.register().update(0xE000);
        assert_eq!(test.vempty_get(), 0x7, "VEmpty should be 0x7");

        test.vempty_clear();
        assert_eq!(test.register().contents(), 0, "Test register should be 0");
        assert_eq!(test.vempty_get(), 0, "VEmpty should be 0");

        test.register().set_all();
        assert_eq!(test.register().contents(), 0xFFFF, "All bits should be set");
        assert_eq!(test.vempty_clear().register().contents(), 0x1FFF, "Test should be ");
        
        assert_eq!(test.register().clear_all().contents(), 0, "Test register shoudl be 0");
        
        test.vempty_set(0x3);
        assert_eq!(test.register().contents(), 0x6000, "Test register should be 0");
        assert_eq!(test.vempty_get(), 0x3, "VEmpty should be 3");

        assert_eq!(test.twelve_get(), false, "Bit 12 should be 0 (false)");
        test.twelve_set(true);
        assert_eq!(test.twelve_get(), true, "Bit 12 should be 1 (true)");

        test.register().update(0xFFFF);
        assert_eq!(test.register().contents(), 0xFFFF, "Test register value should be 0xFFFF");

        assert_eq!(test.vempty_get(), 0x7, "VEmpty should be 0x7");
        assert_eq!(test.middle_byte_get(), 0xFF, "Middle Byte should be 0xFF");
        assert_eq!(test.twelve_get(), true, "Bit Twelve should be true");

        test.register().update(0x0);

        assert_eq!(test.twelve_set(true).vempty_set(0x3).register().contents(), 0x7000, "Example Field should be 3, por bit should be 1");

    }

    #[test]
    fn documentation_macro_example() {
        use paste::paste;
        use bitterly::{register, register_backer, bitfield, bitrange};

        register_backer!(I2CRegister, u8, u16);
    }

    #[test]
    fn documentation_macro_example_2() {
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

    #[test]
    fn documentation_macro_example_3() {
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

}