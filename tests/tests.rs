#[cfg(test)]
mod tests {
    #[test]
    fn register_test() {
        use bitterly::register_backer;
        use paste::paste;

        register_backer!(Register, u8);

        let mut reg = Register::new(0x00);

        // Test set_bit, clear_bit, toggle_bit, is_clear, is_set
        for i in 0..8 {
            reg.set_bit(i);
            assert_eq!(reg.contents(), 1 << i);
            assert_eq!(reg.is_set(i), true);
            reg.clear_bit(i);
            assert_eq!(reg.contents(), 0x00);
            assert_eq!(reg.is_clear(i), true);
            reg.toggle_bit(i);
            assert_eq!(reg.contents(), 1 << i);
            reg.toggle_bit(i);
            assert_eq!(reg.contents(), 0x00);
        }

        // Test set_range, clear_range, and mask, get_range

        // Iterate through all possible end bits
        for end_bit in 1..8 {
            // Iterate through all possible start bits
            for start_bit in 0..end_bit {
                // Bit range struct for the current start and end bits
                let br = BitRange::new(start_bit, end_bit);

                // New mask register to set bits on
                let mut mask_register = Register::new(0);

                // Iterate through all possible mask bits
                for mask_bit in start_bit..(end_bit - start_bit) {
                    // Set a bit in the mask register
                    mask_register.set_bit(mask_bit);
                    // Set the range in the test register with mask_register bits
                    reg.set_range(br, mask_register.contents());
                    assert_eq!(reg.contents(), mask_register.contents() << start_bit);
                    assert_eq!(reg.get_range(br), mask_register.contents());
                    reg.clear_range(br);
                    assert_eq!(reg.contents(), 0x00);
                }
            }
        }
    }

    #[test]
    fn register_define_test() {
        use bitterly::{bitfield, peripheral, register, register_backer};

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            0x0A,
            5,
            [
                (ChipId, 0x00, 0),
                (ChipRev, 0x01, 1),
                (DevStatus1, 0x02, 2),
                (AiclStatus, 0x03, 3),
                (DevStatus2, 0x04, 4)
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
            0x0A,
            5,
            [
                (ChipId, 0x00, 0),
                (ChipRev, 0x01, 1),
                (DevStatus1, 0x02, 2),
                (AiclStatus, 0x03, 3),
                (DevStatus2, 0x04, 4)
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

        max14748
            .DevStatus1()
            .clear()
            .set_BatDet(true)
            .set_ThrmSd120(true);
        let contents = max14748.DevStatus1().contents();
        assert_eq!(contents, 0b00000110);
    }

    #[test]
    pub fn bitrange_enum_test() {
        use bitterly::{bitfield, peripheral, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            0x0A,
            5,
            [
                (ChipId, 0x00, 0),
                (ChipRev, 0x01, 1),
                (DevStatus1, 0x02, 2),
                (AiclStatus, 0x03, 3),
                (DevStatus2, 0x04, 4)
            ]
        );

        register!(AiclStatus);
    }

    #[test]
    pub fn bitrange_test() {
        use bitterly::{
            bitfield, bitrange, bitrange_enum_values, peripheral, register, register_backer,
        };
        use paste::paste;

        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            0x0A,
            5,
            [
                (ChipId, 0x00, 0),
                (ChipRev, 0x01, 1),
                (DevStatus1, 0x02, 2),
                (AiclStatus, 0x03, 3),
                (DevStatus2, 0x04, 4)
            ]
        );

        let max14748 = Max14748::new();

        register!(AiclStatus);

        bitrange_enum_values!(
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

    #[test]
    fn max14748_test() {
        use bitterly::{
            bitfield, bitrange, bitrange_enum_values, bitrange_raw, peripheral, register,
            register_backer,
        };
        use paste::paste;

        // Create a u8 Register
        register_backer!(Register, u8);

        peripheral!(
            Max14748,
            0x0A,
            8,
            [
                (ChipId, 0x00, 0),
                (ChipRev, 0x01, 1),
                (DevStatus1, 0x02, 2),
                (AiclStatus, 0x03, 3),
                (DevStatus2, 0x04, 4),
                (ChgStatus, 0x05, 5),
                (JeitaStatus, 0x06, 6),
                (BcStatus, 0x07, 7),
                (Reserved1, 0x08, 8)
            ]
        );

        register!(ChipId);

        register!(ChipRev);
        bitrange_raw!(ChipRev, RevH, 7, 4, u8);
        bitrange_raw!(ChipRev, RevL, 3, 0, u8);

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
        bitrange_enum_values!(
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
        bitrange_enum_values!(
            AiclCurrSet,
            u8,
            [
                (_100ma, 0),
                (_200ma, 1),
                (_300ma, 2),
                (_400ma, 3),
                (_500ma, 4),
                (_600ma, 5),
                (_700ma, 6),
                (_800ma, 7),
                (_900ma, 8),
                (_1_000ma, 9),
                (_1_100ma, 10),
                (_1_200ma, 11),
                (_1_300ma, 12),
                (_1_400ma, 13),
                (_1_500ma, 14),
                (_1_600ma, 15),
                (_1_700ma, 16),
                (_1_800ma, 17),
                (_1_900ma, 18),
                (_2_000ma, 19),
                (_2_100ma, 20),
                (_2_200ma, 21),
                (_2_300ma, 22),
                (_2_400ma, 23),
                (_2_500ma, 24),
                (_2_600ma, 25),
                (_2_700ma, 26),
                (_2_800ma, 27),
                (_2_900ma, 28),
                (_3_000ma, 29),
                (_3_100ma, 30),
                (_3_200ma, 31)
            ]
        );
        bitrange!(AiclStatus, Status, 7, 5, AiclStatusEnum);
        bitrange!(AiclStatus, CurSet, 4, 0, AiclCurrSet);

        register!(DevStatus2);
        bitfield!(DevStatus2, BattPreqB, 6);
        bitfield!(DevStatus2, BypUVLO, 4);
        bitfield!(DevStatus2, SysUVLOB, 3);
        bitfield!(DevStatus2, DcDcILim, 2);
        bitfield!(DevStatus2, DcDcRunAway, 1);
        bitfield!(DevStatus2, DcDcPGood, 0);

        register!(ChgStatus);
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
                (FaultSuspended, 8)
            ]
        );
        bitfield!(ChgStatus, DirChgFault, 7);
        bitfield!(ChgStatus, LowPow, 5);
        bitrange!(ChgStatus, ChgStat, 3, 0, ChgStatusEnum);

        register!(JeitaStatus);
        bitrange_enum_values!(
            ChgThrmStatEnum,
            u8,
            [
                (T_lt_T1, 0),
                (T1_lt_T_lt_T2, 1),
                (T2_lt_T_lt_T3, 2),
                (T3_lt_T_lt_T4, 3),
                (T_gt_T4, 4),
                (NoThermister, 5),
                (NTCDisabled, 6),
                (DetDisabled, 7)
            ]
        );
        bitfield!(JeitaStatus, ChgThrmRegCur, 4);
        bitfield!(JeitaStatus, ChgThrmRegVolt, 3);
        bitrange!(JeitaStatus, ChgThrmStat, 2, 0, ChgThrmStatEnum);

        register!(BcStatus);
        bitrange_enum_values!(
            PrChgTypEnum,
            u8,
            [
                (Unknown, 0),
                (Samsung_2_0A, 1),
                (Apple_0_5A, 2),
                (Apple_1_0A, 3),
                (Apple_2_0A, 4),
                (Apple_12W, 5),
                (DCP_3_0A, 6),
                (RFU, 7)
            ]
        );
        bitrange_enum_values!(
            ChgTypEnum,
            u8,
            [(NothingAttached, 0), (SDP, 1), (CDP, 2), (DCP, 3)]
        );
        bitfield!(BcStatus, VbusDet, 7);
        bitfield!(BcStatus, ChgTypRun, 6);
        bitrange!(BcStatus, PrChgTyp, 5, 3, PrChgTypEnum);
        bitfield!(BcStatus, DcdTmo, 2);
        bitrange!(BcStatus, ChgTyp, 1, 0, ChgTypEnum);

        let max14748 = Max14748::new();
        max14748.DevStatus1().set_BatDet(true);

        let bcstatus = max14748.BcStatus().get_DcdTmo();

        max14748.JeitaStatus().clear();
    }
}
