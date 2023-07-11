#[cfg(test)]
mod tests {
    #[test]
    fn register_test() {
        use bitterly::{bitfield, bitrange, bitrange_enum, peripheral, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        for i in 0..8 {
            let mut reg = Register::new(0x00);
            reg.set_bit(i);
            assert_eq!(reg.contents(), 1 << i);
            assert_eq!(reg.is_set(i), true);
            reg.clear_bit(i);
            assert_eq!(reg.contents(), 0x00);
            assert_eq!(reg.is_clear(i), true);
        }

        let mut reg = Register::new(0x00);
        reg.set_bit(0);
        assert_eq!(reg.contents(), 0x01);
        reg.set_bit(1);
        assert_eq!(reg.contents(), 0x03);
        reg.clear_bit(0);
        assert_eq!(reg.contents(), 0x02);
        reg.clear_bit(1);
        assert_eq!(reg.contents(), 0x00);
        reg.set_all();
        assert_eq!(reg.contents(), 0xFF);
        reg.clear_all();
        assert_eq!(reg.contents(), 0x00);

        reg.set_bit(7);
        assert_eq!(reg.contents(), 0x80);
        reg.toggle_bit(7);
        assert_eq!(reg.contents(), 0x00);

        // BitRange
        reg = Register::new(0x00);
        reg.set_range(BitRange::new(0, 3), 0x0F);
        assert_eq!(reg.contents(), 0x0F);
        reg.set_range(BitRange::new(4, 7), 0x0F);
        assert_eq!(reg.contents(), 0xFF);
        reg.clear_range(BitRange::new(0, 3));
        assert_eq!(reg.contents(), 0xF0);
        reg.clear_range(BitRange::new(4, 7));
        assert_eq!(reg.contents(), 0x00);

        reg.update(0b00011000);
        assert_eq!(reg.contents(), 0x18);

        let mask = reg.mask(BitRange::new(4, 5));
        assert_eq!(mask, 0x30);
    }

    #[test]
    fn register_define_test() {
        use bitterly::{bitfield, peripheral, register, register_backer};

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            5,
            [
                (ChipId, 0x00),
                (ChipRev, 0x01),
                (DevStatus1, 0x02),
                (AiclStatus, 0x03),
                (DevStatus2, 0x04)
            ]
        );

        register!(ChipId);

        let mut max14748 = Max14748::new();

        let id = max14748.ChipId().contents();
        assert_eq!(id, 0);

        // Set contents of register
        max14748.ChipId().update(8);

        let id = max14748.ChipId().contents();
        assert_eq!(id, 8);
    }

    #[test]
    pub fn bitfield_test() {
        use bitterly::{bitfield, peripheral, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            5,
            [
                (ChipId, 0x00),
                (ChipRev, 0x01),
                (DevStatus1, 0x02),
                (AiclStatus, 0x03),
                (DevStatus2, 0x04)
            ]
        );

        let max14748 = Max14748::new();

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

        register!(AiclStatus);

        let mut batdet = max14748.DevStatus1().get_BatDet();
        assert_eq!(batdet, false);
        max14748.DevStatus1().set_BatDet(true);
        batdet = max14748.DevStatus1().get_BatDet();
        assert_eq!(batdet, true);
    }

    #[test]
    pub fn bitrange_enum_test() {
        use bitterly::{bitfield, peripheral, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            5,
            [
                (ChipId, 0x00),
                (ChipRev, 0x01),
                (DevStatus1, 0x02),
                (AiclStatus, 0x03),
                (DevStatus2, 0x04)
            ]
        );

        register!(AiclStatus);
    }

    #[test]
    pub fn bitrange_test() {
        use bitterly::{bitfield, bitrange, bitrange_enum, peripheral, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            5,
            [
                (ChipId, 0x00),
                (ChipRev, 0x01),
                (DevStatus1, 0x02),
                (AiclStatus, 0x03),
                (DevStatus2, 0x04)
            ]
        );

        let max14748 = Max14748::new();

        register!(AiclStatus);

        bitrange_enum!(
            AiclStatusEnum,
            u8,
            [
                (Off, 0),
                (Precheck, 1),
                (Increment, 2),
                (Blank, 3),
                (Idle, 4),
                (NoConnect, 5)
            ]
        );

        assert_eq!(AiclStatusEnum::Off as u8, 0);
        assert_eq!(AiclStatusEnum::Precheck as u8, 1);
        assert_eq!(AiclStatusEnum::Increment as u8, 2);
        assert_eq!(AiclStatusEnum::Blank as u8, 3);
        assert_eq!(AiclStatusEnum::Idle as u8, 4);
        assert_eq!(AiclStatusEnum::NoConnect as u8, 5);

        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::Off), 0);
        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::Precheck), 1);
        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::Increment), 2);
        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::Blank), 3);
        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::Idle), 4);
        assert_eq!(AiclStatusEnumToNum(AiclStatusEnum::NoConnect), 5);

        assert_eq!(AiclStatusEnumFromNum(0).unwrap(), AiclStatusEnum::Off);
        assert_eq!(AiclStatusEnumFromNum(1).unwrap(), AiclStatusEnum::Precheck);
        assert_eq!(AiclStatusEnumFromNum(2).unwrap(), AiclStatusEnum::Increment);
        assert_eq!(AiclStatusEnumFromNum(3).unwrap(), AiclStatusEnum::Blank);
        assert_eq!(AiclStatusEnumFromNum(4).unwrap(), AiclStatusEnum::Idle);
        assert_eq!(AiclStatusEnumFromNum(5).unwrap(), AiclStatusEnum::NoConnect);
        for i in 6..=255 {
            assert_eq!(AiclStatusEnumFromNum(i).is_none(), true);
        }
    }
}
