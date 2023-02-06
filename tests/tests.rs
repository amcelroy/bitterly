#[cfg(test)]
mod tests {
    #[test]
    fn test_u32() {
        use paste::paste;
        use bitterly::register_maker;

        register_maker!(Register32, u32, u32);

        let mut register = Register32::new(0, 0);
        register.set(1);

        // number = crate::u32::set(number, u32::Bit32::_1);
        assert_eq!(register.value(), 0b00000000_00000000_00000000_0000010);

        register.clear(1);

        assert_eq!(register.value(), 0);

        register.set(0).set(1).set(2).set(3);

        assert_eq!(register.value(), 0xF);

        assert_eq!(Register32::new(0, 1).clear(0).value(), 0);

        assert_eq!(Register32::new(0, 0).set_all().value(), u32::MAX);

        assert_eq!(Register32::new(0, 2).set(0).value(), 3);

        assert_eq!(Register32::new(0, 4).is_set(2), true);

        assert_eq!(Register32::new(0, 0).is_clear(0), true);

        assert_eq!(register.is_clear(5), true);

        register.toggle(5);

        assert_eq!(register.is_set(5), true);

        register.toggle(5);

        assert_eq!(register.is_set(5), false);

    }

    #[test]
    fn test_bitfields() {
        use paste::paste;
        use bitterly::{register, register_maker, bitfield, bitrange};

        register_maker!(Register16, u8, u16);

        pub struct TestRegister {
            register: Register16,
        }

        impl TestRegister {
            register!(TestRegister, Register16);

            bitfield!(twelve, 12, u16);

            bitrange!(vempty, 15, 13, u16);

            bitrange!(middle_byte, 11, 4, u16);
        }

        let mut test = TestRegister::new(Register16::new(0, 0));

        assert_eq!(test.vempty_mask(), 0x0007, "Mask for bits 15, 14, 13 should be 0x7");

        assert_eq!(test.middle_byte_mask(), 0x00FF, "Mask for bits 11..4 should be 0x00FF");

        test.get().update(0xE000);
        assert_eq!(test.vempty_get(), 0x7, "VEmpty should be 0x7");

        test.vempty_clear();
        assert_eq!(test.get().value(), 0, "Test register should be 0");
        assert_eq!(test.vempty_get(), 0, "VEmpty should be 0");

        
        assert_eq!(test.get().clear_all().value(), 0, "Test register shoudl be 0");
        
        test.vempty_set(0x3);
        assert_eq!(test.get().value(), 0x6000, "Test register should be 0");
        assert_eq!(test.vempty_get(), 0x3, "VEmpty should be 3");

        assert_eq!(test.twelve_get(), false, "Bit 12 should be 0 (false)");
        test.twelve_set(true);
        assert_eq!(test.twelve_get(), true, "Bit 12 should be 1 (true)");

        test.get().update(0xFFFF);
        assert_eq!(test.get().value(), 0xFFFF, "Test register value should be 0xFFFF");

        assert_eq!(test.vempty_get(), 0x7, "VEmpty should be 0x7");
        assert_eq!(test.middle_byte_get(), 0xFF, "Middle Byte should be 0xFF");
        assert_eq!(test.twelve_get(), true, "Bit Twelve should be true");

    }

}