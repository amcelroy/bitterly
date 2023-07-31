#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub fn main() {
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
        53,
        [
            (ChipId, 0x00, 0),
            (ChipRev, 0x01, 1),
            (DevStatus1, 0x02, 2),
            (AiclStatus, 0x03, 3),
            (DevStatus2, 0x04, 4),
            (ChgStatus, 0x05, 5),
            (JeitaStatus, 0x06, 6),
            (BcStatus, 0x07, 7),
            (Reserved0x08, 0x08, 8),
            (CCStatus1, 0x9, 9),
            (CCStatus2, 0xA, 10),
            (DevInt1, 0xB, 11),
            (AiclInt, 0xC, 12),
            (DevInt2, 0xD, 13),
            (ChgInt, 0xE, 14),
            (JeitaInt, 0xF, 15),
            (BcInt, 0x10, 16),
            (CcInt, 0x11, 17),
            (DevInt1Mask, 0x12, 18),
            (AiclIntMask, 0x13, 19),
            (DevInt2Mask, 0x14, 20),
            (ChgIntMask, 0x15, 21),
            (JeitaIntMask, 0x16, 22),
            (BcIntMask, 0x17, 23),
            (CcIntMask, 0x18, 24),
            (LedCtrl, 0x19, 25),
            (ThermaCfg1, 0x1A, 26),
            (ThermaCfg2, 0x1B, 27),
            (ThermaCfg3, 0x1C, 28),
            (ChargerCtrl1, 0x1D, 29),
            (ChargerCtrl2, 0x1E, 30),
            (ChargerCtrl3, 0x1F, 31),
            (ChargerCtrl4, 0x20, 32),
            (CurLimCtrl, 0x21, 33),
            (CurLimStatus, 0x22, 34),
            (BbCfg1, 0x23, 35),
            (BbCfg2, 0x24, 36),
            (BcCtrl1, 0x25, 37),
            (Reserved0x26, 0x26, 38),
            (CcCtrl1, 0x27, 39),
            (CcCtrl2, 0x28, 40),
            (CcCtrl3, 0x29, 41),
            (ChgInILim1, 0x2A, 42),
            (ChgInILim2, 0x2B, 43),
            (AiclCfg1, 0x2C, 44),
            (AiclCfg2, 0x2D, 45),
            (AiclCfg3, 0x2E, 46),
            (DpdnSw, 0x2F, 47),
            (Other, 0x30, 48),
            (Reserved0x31, 0x31, 49),
            (Reserved0x32, 0x32, 50),
            (LowPow, 0x33, 51),
            (Reserved0x34, 0x34, 52),
            (FltSel, 0x35, 53)
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
        AiclCurrSetEnum,
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
    bitrange!(AiclStatus, CurSet, 4, 0, AiclCurrSetEnum);

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
    bitfield!(ChgStatus, LowPowMode, 5);
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

    register!(Reserved0x08);

    register!(CCStatus1);
    bitrange_enum_values!(
        CcPinStatEnum,
        u8,
        [
            (NoDetermination, 0),
            (CC1Active, 1),
            (CC2Active, 2),
            (RFU, 3)
        ]
    );
    bitrange_enum_values!(
        CcIStatEnum,
        u8,
        [(NotInUFPMode, 0), (_500mA, 1), (_1_5A, 2), (_3_0A, 3)]
    );
    bitrange_enum_values!(
        CcStatEnum,
        u8,
        [
            (NoConnection, 0),
            (UFP, 1),
            (DFP, 2),
            (AudioAccessory, 3),
            (DebugAccessory, 4),
            (Error, 5),
            (Disabled, 6),
            (RFU, 7)
        ]
    );
    bitrange!(CCStatus1, CcPinStat, 7, 6, CcPinStatEnum);
    bitrange!(CCStatus1, CcIStat, 5, 4, CcIStatEnum);
    bitfield!(CCStatus1, CcVcnStat, 3);
    bitrange!(CCStatus1, CcStat, 2, 0, CcStatEnum);

    register!(CCStatus2);
    bitfield!(CCStatus2, VSafe, 3);
    bitfield!(CCStatus2, DetAbrt, 2);

    register!(DevInt1);
    bitfield!(DevInt1, SysFitInt, 7);
    bitfield!(DevInt1, ChgInOvpInt, 6);
    bitfield!(DevInt1, ILimInt, 5);
    bitfield!(DevInt1, VSysRegInt, 4);
    bitfield!(DevInt1, ThrmSd150Int, 3);
    bitfield!(DevInt1, ThrmSd120Int, 2);
    bitfield!(DevInt1, BatDetInt, 1);
    bitfield!(DevInt1, WbChgInt, 0);

    register!(AiclInt);
    bitfield!(AiclInt, AiclI, 6);

    register!(DevInt2);
    bitfield!(DevInt2, BattPreqBInt, 6);
    bitfield!(DevInt2, BypUVLOInt, 4);
    bitfield!(DevInt2, SysUVLOBInt, 3);
    bitfield!(DevInt2, DcDcILimInt, 2);
    bitfield!(DevInt2, DcDcRunAwayInt, 1);
    bitfield!(DevInt2, DcDcPGoodInt, 0);

    register!(ChgInt);
    bitfield!(ChgInt, DirChgFaultInt, 7);
    bitfield!(ChgInt, LowPowRI, 6);
    bitfield!(ChgInt, LowPowFI, 5);
    bitfield!(ChgInt, ChgStatInt, 0);

    register!(JeitaInt);
    bitfield!(JeitaInt, ChgThrmRegCurInt, 4);
    bitfield!(JeitaInt, ChgThrmRegVoltInt, 3);
    bitfield!(JeitaInt, ChgThrmStatInt, 0);

    register!(BcInt);
    bitfield!(BcInt, VbusDetInt, 7);
    bitfield!(BcInt, ChgTypRunFallingInt, 4);
    bitfield!(BcInt, ChgTypRunRisingInt, 3);
    bitfield!(BcInt, PrChgTypInt, 2);
    bitfield!(BcInt, DcdTmoInt, 1);
    bitfield!(BcInt, ChgTypInt, 0);

    register!(CcInt);
    bitfield!(CcInt, VSafe0vInt, 6);
    bitfield!(CcInt, DetAbrtInt, 5);
    bitfield!(CcInt, CcPinStatInt, 3);
    bitfield!(CcInt, CcIStatInt, 2);
    bitfield!(CcInt, CcVcnStatInt, 1);
    bitfield!(CcInt, CcStatInt, 0);

    register!(DevInt1Mask);
    bitfield!(DevInt1Mask, SysFitIntMask, 7);
    bitfield!(DevInt1Mask, ChgInOvpIntMask, 6);
    bitfield!(DevInt1Mask, ILimIntMask, 5);
    bitfield!(DevInt1Mask, VSysRegIntMask, 4);
    bitfield!(DevInt1Mask, ThrmSd150IntMask, 3);
    bitfield!(DevInt1Mask, ThrmSd120IntMask, 2);
    bitfield!(DevInt1Mask, BatDetIntMask, 1);
    bitfield!(DevInt1Mask, WbChgIntMask, 0);

    register!(AiclIntMask);
    bitfield!(AiclIntMask, AiclIntM, 6);

    register!(DevInt2Mask);
    bitfield!(DevInt2Mask, BattPreqBIntMask, 6);
    bitfield!(DevInt2Mask, BypUVLOIntMask, 4);
    bitfield!(DevInt2Mask, SysUVLOBIntMask, 3);
    bitfield!(DevInt2Mask, DcDcILimIntMask, 2);
    bitfield!(DevInt2Mask, DcDcRunAwayIntMask, 1);
    bitfield!(DevInt2Mask, DcDcPGoodIntMask, 0);

    register!(ChgIntMask);
    bitfield!(ChgIntMask, DirChgFaultIntMask, 7);
    bitfield!(ChgIntMask, LowPowRIMask, 6);
    bitfield!(ChgIntMask, LowPowFIMask, 5);
    bitfield!(ChgIntMask, ChgStatIntMask, 0);

    register!(JeitaIntMask);
    bitfield!(JeitaIntMask, ChgThrmRegCurIntMask, 4);
    bitfield!(JeitaIntMask, ChgThrmRegVoltIntMask, 3);
    bitfield!(JeitaIntMask, ChgThrmStatIntMask, 0);

    register!(BcIntMask);
    bitfield!(BcIntMask, VbusDetIntMask, 7);
    bitfield!(BcIntMask, ChgTypRunFallingIntMask, 4);
    bitfield!(BcIntMask, ChgTypRunRisingIntMask, 3);
    bitfield!(BcIntMask, PrChgTypIntMask, 2);
    bitfield!(BcIntMask, DcdTmoIntMask, 1);
    bitfield!(BcIntMask, ChgTypIntMask, 0);

    register!(CcIntMask);
    bitfield!(CcIntMask, VSafe0vIntMask, 6);
    bitfield!(CcIntMask, DetAbrtIntMask, 5);
    bitfield!(CcIntMask, CcPinStatIntMask, 3);
    bitfield!(CcIntMask, CcIStatIntMask, 2);
    bitfield!(CcIntMask, CcVcnStatIntMask, 1);
    bitfield!(CcIntMask, CcStatIntMask, 0);

    register!(LedCtrl);
    bitfield!(LedCtrl, Led, 1);
    bitfield!(LedCtrl, LedManual, 0);

    register!(ThermaCfg1);
    bitrange_enum_values!(
        T1T2IFchgEnum,
        u8,
        [
            (IFChg_0_2, 0b000),
            (IFChg_0_3, 0b001),
            (IFChg_0_4, 0b010),
            (IFChg_0_5, 0b011),
            (IFChg_0_6, 0b100),
            (IFChg_0_7, 0b101),
            (IFChg_0_8, 0b110),
            (IFChg_1_0, 0b111)
        ]
    );
    bitrange_enum_values!(
        T2T3IFchgEnum,
        u8,
        [
            (IFChg_0_2, 0b000),
            (IFChg_0_3, 0b001),
            (IFChg_0_4, 0b010),
            (IFChg_0_5, 0b011),
            (IFChg_0_6, 0b100),
            (IFChg_0_7, 0b101),
            (IFChg_0_8, 0b110),
            (IFChg_1_0, 0b111)
        ]
    );
    bitrange_enum_values!(
        JeitaCfgREnum,
        u8,
        [
            (MonitoringTpuSwDisabled, 0),
            (JeitaEnabledVchginGTVBdet, 1),
            (MonitoringTpuSwEnabled, 2),
            (MonitoringDisabledTpuSwEnabled, 3)
        ]
    );
    bitrange!(ThermaCfg1, T1T2IFchg, 7, 5, T1T2IFchgEnum);
    bitrange!(ThermaCfg1, T2T3IFchg, 4, 2, T2T3IFchgEnum);
    bitrange!(ThermaCfg1, JeitaCfgR, 1, 0, JeitaCfgREnum);

    register!(ThermaCfg2);
    bitrange_enum_values!(
        T3T4IFchgEnum,
        u8,
        [
            (IFChg_0_2, 0b000),
            (IFChg_0_3, 0b001),
            (IFChg_0_4, 0b010),
            (IFChg_0_5, 0b011),
            (IFChg_0_6, 0b100),
            (IFChg_0_7, 0b101),
            (IFChg_0_8, 0b110),
            (IFChg_1_0, 0b111)
        ]
    );
    bitrange!(ThermaCfg2, T3T4IFchg, 7, 5, T3T4IFchgEnum);
    bitfield!(ThermaCfg2, T3T4ENset, 3);
    bitfield!(ThermaCfg2, T1T2ENset, 2);
    bitfield!(ThermaCfg2, T3T4VFset, 1);
    bitfield!(ThermaCfg2, T1T2VFset, 0);

    register!(ThermaCfg3);
    bitfield!(ThermaCfg3, JeitaCtrSet, 1);
    bitfield!(ThermaCfg3, WarmCoolSet, 0);

    register!(ChargerCtrl1);
    bitrange_enum_values!(
        BatRetChgEnum,
        u8,
        [
            (_200mv, 0b00),
            (_300mv, 0b01),
            (_400mv, 0b10),
            (_500mv, 0b11)
        ]
    );
    bitrange_enum_values!(
        BatRegEnum,
        u8,
        [(_8_3V, 0b00), (_8_4V, 0b01), (_8_5V, 0b10), (_8_6V, 0b11)]
    );
    bitfield!(ChargerCtrl1, ChgAutoStp, 7);
    bitrange!(ChargerCtrl1, BatRetChg, 6, 5, BatRetChgEnum);
    bitfield!(ChargerCtrl1, FreshBatDis, 4);
    bitrange!(ChargerCtrl1, BatReg, 2, 1, BatRegEnum);
    bitfield!(ChargerCtrl1, ChgEn, 0);

    register!(ChargerCtrl2);
    bitrange_enum_values!(
        VPchgEnum,
        u8,
        [
            (_5_7V, 0b000),
            (_5_8V, 0b001),
            (_5_9V, 0b010),
            (_6_0V, 0b011),
            (_6_1V, 0b100),
            (_6_2V, 0b101),
            (_6_3V, 0b110),
            (_6_4V, 0b111)
        ]
    );
    bitrange_enum_values!(
        IPchgEnum,
        u8,
        [(_0_05x, 0b00), (_0_10x, 0b01), (_0_2x, 0b10), (_0_3x, 0b11)]
    );
    bitrange_enum_values!(
        ChgDoneEnum,
        u8,
        [(_0_05x, 0b00), (_0_10x, 0b01), (_0_2x, 0b10)]
    );
    bitrange!(ChargerCtrl2, VPchg, 5, 4, VPchgEnum);
    bitrange!(ChargerCtrl2, IPchg, 3, 2, IPchgEnum);
    bitrange!(ChargerCtrl2, ChgDone, 1, 0, ChgDoneEnum);

    register!(ChargerCtrl3);
    bitrange_enum_values!(
        MtChgTmrEnum,
        u8,
        [(_0m, 0b00), (_15m, 0b01), (_30m, 0b10), (_60m, 0b11)]
    );
    bitrange_enum_values!(
        FChgTmrEnum,
        u8,
        [(_75m, 0b00), (_150m, 0b01), (_300m, 0b10), (_600m, 0b11)]
    );
    bitrange_enum_values!(
        PChgTmrEnum,
        u8,
        [(_30m, 0b00), (_60m, 0b01), (_120m, 0b10), (_240m, 0b11)]
    );
    bitfield!(ChargerCtrl3, ChgAutoSta, 6);
    bitrange!(ChargerCtrl3, MtChgTmr, 5, 4, MtChgTmrEnum);
    bitrange!(ChargerCtrl3, FChgTmr, 3, 2, FChgTmrEnum);
    bitrange!(ChargerCtrl3, PChgTmr, 1, 0, PChgTmrEnum);

    register!(ChargerCtrl4);
    bitrange_enum_values!(
        WeakBatStatEnum,
        u8,
        [
            (Idle, 0b000),
            (BattCondCheck, 0b001),
            (WeakBatt2mCounter, 0b010),
            (GoodBattery, 0b011),
            (WeakBatt2mCounterExpired, 0b100)
        ]
    );
    bitrange!(ChargerCtrl4, WeakBatStat, 7, 5, WeakBatStatEnum);
    bitfield!(ChargerCtrl4, WeakBatEn, 2);

    register!(CurLimCtrl);
    bitrange_enum_values!(
        CurLim1SetEnum,
        u8,
        [
            (_0_10A, 0b00000),
            (_0_20A, 0b00001),
            (_0_30A, 0b00010),
            (_0_40A, 0b00011),
            (_0_50A, 0b00100),
            (_0_60A, 0b00101),
            (_0_70A, 0b00110),
            (_0_80A, 0b00111),
            (_0_90A, 0b01000),
            (_1_00A, 0b01001),
            (_1_10A, 0b01010),
            (_1_20A, 0b01011),
            (_1_30A, 0b01100),
            (_1_40A, 0b01101),
            (_1_50A, 0b01110),
            (_1_60A, 0b01111),
            (_1_70A, 0b10000),
            (_1_80A, 0b10001),
            (_1_90A, 0b10010),
            (_2_00A, 0b10011),
            (_2_10A, 0b10100),
            (_2_20A, 0b10101),
            (_2_30A, 0b10110),
            (_2_40A, 0b10111),
            (_2_50A, 0b11000),
            (_2_60A, 0b11001),
            (_2_70A, 0b11010),
            (_2_80A, 0b11011),
            (_2_90A, 0b11100),
            (_3_00A, 0b11101)
        ]
    );
    bitfield!(CurLimCtrl, CurLim1Frc, 7);
    bitfield!(CurLimCtrl, FsusMask, 6);
    bitrange!(CurLimCtrl, CurLim1Set, 4, 0, CurLim1SetEnum);

    register!(CurLimStatus);
    bitrange_enum_values!(
        CurLim2RbEnum,
        u8,
        [
            (_20perc, 0),
            (_30perc, 1),
            (_40perc, 2),
            (_50perc, 3),
            (_60perc, 4),
            (_70perc, 5),
            (_80perc, 6),
            (_100perc, 7)
        ]
    );
    bitrange_enum_values!(
        SpvChgILimEnum,
        u8,
        [
            (_0_10A, 0b00000),
            (_0_20A, 0b00001),
            (_0_30A, 0b00010),
            (_0_40A, 0b00011),
            (_0_50A, 0b00100),
            (_0_60A, 0b00101),
            (_0_70A, 0b00110),
            (_0_80A, 0b00111),
            (_0_90A, 0b01000),
            (_1_00A, 0b01001),
            (_1_10A, 0b01010),
            (_1_20A, 0b01011),
            (_1_30A, 0b01100),
            (_1_40A, 0b01101),
            (_1_50A, 0b01110),
            (_1_60A, 0b01111),
            (_1_70A, 0b10000),
            (_1_80A, 0b10001),
            (_1_90A, 0b10010),
            (_2_00A, 0b10011),
            (_2_10A, 0b10100),
            (_2_20A, 0b10101),
            (_2_30A, 0b10110),
            (_2_40A, 0b10111),
            (_2_50A, 0b11000),
            (_2_60A, 0b11001),
            (_2_70A, 0b11010),
            (_2_80A, 0b11011),
            (_2_90A, 0b11100),
            (_3_00A, 0b11101)
        ]
    );
    bitrange!(CurLimStatus, CurLim2Rb, 7, 5, CurLim2RbEnum);
    bitrange!(CurLimStatus, SpvChgILim, 4, 0, SpvChgILimEnum);

    register!(BbCfg1);
    bitrange_enum_values!(
        BoostRCompEnum,
        u8,
        [
            (k9_5, 0b000),
            (k17_3, 0b001),
            (k25_3, 0b010),
            (k33_2, 0b011),
            (k41_4, 0b100),
            (k49_2, 0b101),
            (k57_3, 0b110),
            (k65_1, 0b111),
            (k73_6, 0b1000),
            (k81_4, 0b1001),
            (k89_4, 0b1010),
            (k97_2, 0b1011),
            (k105_5, 0b1100),
            (k113_3, 0b1101),
            (k121_4, 0b1110),
            (k129_2, 0b1111)
        ]
    );
    bitrange!(BbCfg1, BoostRComp, 7, 4, BoostRCompEnum);

    register!(BbCfg2);
    bitrange_enum_values!(
        BuckVSetEnum,
        u8,
        [
            (_4_0V, 0b000),
            (_4_1V, 0b001),
            (_4_2V, 0b010),
            (_4_3V, 0b011),
            (_4_4V, 0b100),
            (_4_5V, 0b101),
            (_4_6V, 0b110),
            (_4_7V, 0b111),
            (_4_8V, 0b1000),
            (_4_9V, 0b1001),
            (_5_0V, 0b1010),
            (_5_1V, 0b1011),
            (_5_2V, 0b1100),
            (_5_3V, 0b1101),
            (_5_4V, 0b1110),
            (_5_5V, 0b1111)
        ]
    );
    bitfield!(BbCfg2, BBFrcZX, 7);
    bitrange!(BbCfg2, BuckVSet, 3, 0, BuckVSetEnum);

    register!(BcCtrl1);
    bitrange_enum_values!(
        StOutCtrlEnum,
        u8,
        [
            (Disabled, 0b00),
            (EnabledIfValidChgIn, 0b01),
            (ConditionallyEnabled, 0b10),
            (RFU, 0b11)
        ]
    );
    bitfield!(BcCtrl1, DCD2s, 7);
    bitfield!(BcCtrl1, SfOutLvl, 6);
    bitfield!(BcCtrl1, ADC3PDet, 4);
    bitrange!(BcCtrl1, StOutCtrl, 3, 2, StOutCtrlEnum);
    bitfield!(BcCtrl1, ChgDetMan, 1);
    bitfield!(BcCtrl1, ChgDetEn, 0);

    register!(Reserved0x26);

    register!(CcCtrl1);
    bitfield!(CcCtrl1, CcSrcSnk, 4);
    bitfield!(CcCtrl1, CcSrcSrc, 3);
    bitfield!(CcCtrl1, CcDbgEn, 2);
    bitfield!(CcCtrl1, CcAudEn, 1);
    bitfield!(CcCtrl1, CcDetEn, 0);

    register!(CcCtrl2);
    bitfield!(CcCtrl2, CcForceError, 7);
    bitfield!(CcCtrl2, SnkAttachedLock, 6);
    bitfield!(CcCtrl2, CcSnkSrcSwp, 5);
    bitfield!(CcCtrl2, CcSrcSnkSwp, 4);
    bitfield!(CcCtrl2, CcVcnSwp, 3);
    bitfield!(CcCtrl2, CcVcnEn, 2);
    bitfield!(CcCtrl2, CcSrcRst, 1);
    bitfield!(CcCtrl2, CcSnkRst, 0);

    register!(CcCtrl3);
    bitrange_enum_values!(
        CCDRPPhaseEnum,
        u8,
        [
            (_35perc, 0b00),
            (_40perc, 0b01),
            (_45perc, 0b10),
            (_50perc, 0b11)
        ]
    );
    bitfield!(CcCtrl3, CcTrySnk, 3);
    bitfield!(CcCtrl3, CcPreferSnk, 2);
    bitrange!(CcCtrl3, CCDRPPhase, 1, 0, CCDRPPhaseEnum);

    register!(ChgInILim1);
    bitrange_raw!(ChgInILim1, ChgInILim, 6, 0, u8);

    register!(ChgInILim2);
    bitrange_enum_values!(
        SDPMaxCurrEnum,
        u8,
        [
            (None, 0x00),
            (Sdp_500ma, 0x01),
            (Sdp_1_0A, 0x10),
            (Sdp_1_5A, 0x11)
        ]
    );
    bitfield!(ChgInILim2, ChgInILimGate, 3);
    bitrange!(ChgInILim2, SDPMaxCurr, 2, 1, SDPMaxCurrEnum);
    bitfield!(ChgInILim2, CdpMaxCur, 0);

    register!(AiclCfg1);
    bitfield!(AiclCfg1, AiclEn, 7);
    bitfield!(AiclCfg1, AiclAbort, 0);

    register!(AiclCfg2);
    bitrange_enum_values!(
        BypUvloEnum,
        u8,
        [
            (_3_8V, 0b000),
            (_3_9V, 0b001),
            (_4_0V, 0b010),
            (_4_1V, 0b011),
            (_4_2V, 0b100),
            (_4_3V, 0b101),
            (_4_4V, 0b110),
            (_4_5V, 0b111)
        ]
    );
    bitrange_enum_values!(
        AiclMaxIEnum,
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
    bitrange!(AiclCfg2, BypUvlo, 7, 5, BypUvloEnum);
    bitrange!(AiclCfg2, AiclMaxI, 4, 0, AiclMaxIEnum);

    register!(AiclCfg3);
    bitrange_enum_values!(
        AiclTBlkEnum,
        u8,
        [
            (_0_500ms, 0b00),
            (_1_0s, 0b01),
            (_1_5s, 0b10),
            (_5_0s, 0b11)
        ]
    );
    bitrange_enum_values!(
        AiclTStepEnum,
        u8,
        [
            (_100ms, 0b00),
            (_200ms, 0b01),
            (_300ms, 0b10),
            (_500ms, 0b11)
        ]
    );
    bitfield!(AiclCfg3, BypDeb, 4);
    bitrange!(AiclCfg3, AiclTBlk, 3, 2, AiclTBlkEnum);
    bitrange!(AiclCfg3, AiclTStep, 1, 0, AiclTStepEnum);

    register!(DpdnSw);
    bitrange_enum_values!(
        AnSwCntlEnum,
        u8,
        [
            (Auto, 0b00),
            (AutoInSDP_CDP, 0b01),
            (Open, 0b10),
            (Closed, 0b11)
        ]
    );
    bitrange!(DpdnSw, AnSwCntl, 1, 0, AnSwCntlEnum);

    register!(Other);
    bitfield!(Other, USBCRSet, 0);

    register!(Reserved0x31);

    register!(Reserved0x32);

    register!(LowPow);
    bitfield!(LowPow, LowPowEn, 7);
    bitfield!(LowPow, LowPowAbort, 0);

    register!(Reserved0x34);

    register!(FltSel);
    bitrange_enum_values!(
        FltSelEnum,
        u8,
        [
            (NoEffect, 0b00),
            (LowToFault, 0b01),
            (FallingEdgeResetAllRegisters, 0b10)
        ]
    );
    bitrange!(FltSel, FLTSellect, 7, 6, FltSelEnum);

    let max14748 = Max14748::new();
    let address = max14748
        .DevStatus1()
        .set_ThrmSd120(true)
        .set_ThrmSd150(true)
        .address();

    max14748.ChgStatus().get_ChgStat().unwrap();
    max14748.ChgStatus().set_ChgStat(ChgStatusEnum::Fault);

    let bcstatus = max14748.BcStatus().get_VbusDet();
}
