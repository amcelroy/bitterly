pub fn main() {
    use bitterly::{bitfield, bitrange, bitrange_enum, peripheral, register, register_backer};
    use paste::paste;

    // Create a u8 Register
    register_backer!(Register, u8);

    peripheral!(
        Max14748,
        5,
        [
            (ChipId, 0x00),
            (ChipRev, 0x01),
            (DevStatus1, 0x02),
            (AiclStatus, 0x03),
            (DevStatus2, 0x04),
            (ChgStatus, 0x05),
            (JeitaStatus, 0x06),
            (BcStatus, 0x07),
            (Reserved1, 0x08),
            (CCStatus1, 0x9),
            (CCStatus2, 0xA),
            (DevInt1, 0xB),
            (AiclInt, 0xC),
            (DevInt2, 0xD),
            (ChgInt, 0xE),
        ]
    );

    register!(ChipId);

    register!(ChipRev);
    bitfield!(ChipRev, RevH, 7, 4, u8);
    bitfield!(ChipRev, RevL, 3, 0, u8);

    register!(DevStatus1);
    bitfield!(DevStatus1, SysFit, 7);
    bitfield!(DevStatus1, ChgInOvp, 6);
    bitfield!(DevStatus1, ILim, 5);
    bitfield!(DevStatus1, VSysReg, 4);
    bitfield!(DevStatus1, ThrmSd150, 3);
    bitfield!(DevStatus1, ThrmSd120, 2);
    bitfield!(DevStatus1, BatDet, 1);
    bitfield!(DevStatus1, WbChg, 0);

    register!(Max14748, AiclStatus);
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
    bitrange_enum!(
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
            (_3_200ma, 31),
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
    bitrange_enum!(
        ChgStatusEnum,
        u8,
        [
            (Off, 0),
            (Suspended, 1),
            (PreChg, 2),
            (FastChargeI, 3)(FastChargeV, 4),
            (MaintainInProgress, 5),
            (MaintainComplete, 6),
            (Fault, 7),
            (FaultSuspended, 8),
        ]
    );
    bitfield!(ChgStatus, DirChgFault, 7);
    bitfield!(ChgStatus, LowPow, 5);
    bitrange!(ChgStatus, ChgStat, 3, 0, ChgStatusEnum);

    register!(JeitaStatus);
    bitrange_enum!(
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
            (DetDisabled, 7),
        ]
    );
    bitfield!(JeitaStatus, ChgThrmRegCur, 4);
    bitfield!(JeitaStatus, ChgThrmRegVolt, 3);
    bitrange!(JeitaStatus, ChgThrmStat, 2, 0, ChgThrmStatEnum);

    register!(BcStatus);
    bitrange_enum!(
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
            (RFU, 7),
        ]
    );
    bitrange_enum!(
        ChgTypEnum,
        u8,
        [(NothingAttached, 0), (SDP, 1), (CDP, 2), (DCP, 3),]
    );
    bitfield!(BcStatus, VbusDet, 7);
    bitfield!(BcStatus, ChgTypRun, 6);
    bitrange!(BcStatus, PrChgTyp, 5, 3, PrChgTypEnum);
    bitfield!(BcStatus, DcdTmo, 2);
    bitrange!(BcStatus, ChgTyp, 1, 0, ChgTypEnum);

    register(CCStatus1);
    bitrange_enum!(
        CcPinStatEnum,
        u8,
        [
            (NoDetermination, 0),
            (CC1Active, 1),
            (CC2Active, 2),
            (RFU, 3),
        ]
    );
    bitrange_enum!(
        CcIStatEnum,
        u8,
        [(NotInUFPMode, 0), (_500mA, 1), (_1_5A, 2), (_3_0A, 3),]
    );
    bitrange_enum!(
        CcStatEnum,
        u8,
        [
            (NoConnection, 0),
            (UFP, 1),
            (DFP, 2),
            (AudioAccessory, 3),
            (DebugAccessory, 3),
            (Error, 3),
            (Disabled, 3),
            (RFU, 3),
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
    bitfield!(AiclInt, AiclInt, 6);

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
    bitfield!(AiclIntMask, AiclIntMask, 6);

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
    bitfield!(LedCtrl, LedCtrl, 1);
    bitfield!(LedCtrl, LedManual, 0);

    register!(ThermaCfg1);
    bitrange_enum!(
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
            (IFChg_1_0, 0b111),
        ]
    );
    bitrange_enum!(
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
            (IFChg_1_0, 0b111),
        ]
    );
    bitrange_enum!(JeitaCfgREnum, u8. [
        (MonitoringTpuSwDisabled, 0),
        (JeitaEnabledVchginGTVBdet, 1),
        (MonitoringTpuSwEnabled, 1),
        (MonitoringDisabledTpuSwEnabled, 2),
    ]);
    bitrange!(ThermaCfg1, T1T2IFchg, 7, 5, T1T2IFchgEnum);
    bitrange!(ThermaCfg1, T2T3IFchg, 4, 2, T2T3IFchgEnum);
    bitrange!(ThermaCfg1, JeitaCfgR, 1, 0, JeitaCfgREnum);

    register!(ThermaCfg2);
    bitrange_enum!(
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
            (IFChg_1_0, 0b111),
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
    bitrange_enum!(
        BatRetChgEnum,
        u8,
        [
            (_200mv, 0b00),
            (_300mv, 0b01),
            (_400mv, 0b10),
            (_500mv, 0b11),
        ]
    );
    bitrange_enum!(
        BatRegEnum,
        u8,
        [(_8_3V, 0b00), (_8_4V, 0b01), (_8_5V, 0b10), (_8_6V, 0b11),]
    );
    bitfield!(ChargerCtrl1, ChgAutoStp, 7);
    bitrange!(ChargerCtrl1, BatRetChg, 6, 5, BatRetChgEnum);
    bitfield!(ChargerCtrl1, FreshBatDis, 4);
    bitrange!(ChargerCtrl1, BatReg, 2, 1, BatRegEnum);
    bitfield!(ChargerCtrl1, ChgEn, 0);

    register!(ChargerCtrl2);
    bitrange_enum!(
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
            (_6_4V, 0b111),
        ]
    );
    bitrange_enum!(
        IPchgEnum,
        u8,
        [(_0_05x, 0b00), (_0_10x, 0b01), (_0_2x, 0b10), (_0_3x, 0b11),]
    );
    bitrange_enum!(
        ChgDoneEnum,
        u8,
        [(_0_05x, 0b00), (_0_10x, 0b01), (_0_2x, 0b10),]
    );
    bitrange!(ChargerCtrl2, VPchg, 5, 4, VPchgEnum);
    bitrange!(ChargerCtrl2, IPchg, 3, 2, IPchgEnum);
    bitrange!(ChargerCtrl2, ChgDone, 1, 0, ChgDoneEnum);

    register!(ChargerCtrl3);
    bitrange_enum!(
        MtChgTmrEnum,
        u8,
        [(_0m, 0b00), (_15m, 0b01), (_30m, 0b10), (_60m, 0b11),]
    );
    bitrange_enum!(
        FChgTmrEnum,
        u8,
        [(_75m, 0b00), (_150m, 0b01), (_300m, 0b10), (_600m, 0b11),]
    );
    bitrange_enum!(
        PChgTmr,
        u8,
        [(_30m, 0b00), (_60m, 0b01), (_120m, 0b10), (_240m, 0b11),]
    );
    bitfield!(ChargerCtrl3, ChgAutoSta, 6);
    bitrange!(ChargerCtrl3, MtChgTmr, 5, 4, MtChgTmrEnum);
    bitrange!(ChargerCtrl3, FChgTmr, 3, 2, FChgTmrEnum);
    bitrange!(ChargerCtrl3, PChgTmr, 1, 0, PChgTmr);

    register!(ChargerCtrl4);
    bitrange_enum!(
        WeakBatStatEnum,
        u8,
        [
            (Idle, 0b000),
            (BattCondCheck, 0b001),
            (WeakBatt2mCounter, 0b010),
            (GoodBattery, 0b011),
            (WeakBatt2mCounterExpired, 0b100),
        ]
    );
    bitrange!(ChargerCtrl4, WeakBatStat, 7, 5, WeakBatStatEnum);
    bitfield!(ChargerCtrl4, WeakBatEn, 2);

    register!(CurLimCtrl);
    bitrange_enum!(
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
            (_3_00A, 0b11101),
        ]
    );
    bitfield!(CurLimCtrl, CurLim1Frc, 7);
    bitfield!(CurLimCtrl, FsusMask, 6);
    bitrange!(CurLimCtrl, CurLim1Set, 4, 0, CurLim1SetEnum);

    register!(CurLimStatus);
    bitfield_enum!(
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
            (_100perc, 7),
        ]
    );
    bitfield_enum!(
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
            (_3_00A, 0b11101),
        ]
    );
    bitrange!(CurLimStatus, CurLim2Rb, 7, 5, CurLim2RbEnum);
    bitrange!(CurLimStatus, SpvChgILim, 4, 0, SpvChgILimEnum);

    register!(BbCfg1);
    bitfield_enum!(
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
            (k129_2, 0b1111),
        ]
    );
    bitrange!(BbCfg1, BoostRComp, 7, 4, BoostRCompEnum);

    register!(BbCfg2);
    bitfield_enum!(
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
            (_5_5V, 0b1111),
        ]
    );
    bitfield!(BbCfg2, BBFrcZX, 7);
    bitrange!(BbCfg2, BuckVSet, 3, 0, BuckVSetEnum);

    register!(BcCtrl1);
    bitfield_enum!(
        StOutCtrlEnum,
        u8,
        [
            (Disabled, 0b00),
            (EnabledIfValidChgIn, 0b01),
            (ConditionallyEnabled, 0b10),
            (RFU, 0b11),
        ]
    );
    bitfield!(BcCtrl1, DCD2s, 7);
    bitfield!(BcCtrl1, SfOutLvl, 6);
    bitfield!(BcCtrl1, ADC3PDet, 4);
    bitrange!(BcCtrl1, StOutCtrl, 3, 2, StOutCtrlEnum);
    bitfield!(BcCtrl1, ChgDetMan, 1);
    bitfield!(BcCtrl1, ChgDetEn, 0);

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
    bitfield_enum!(
        CCDRPPhaseEnum,
        u8,
        [
            (_35perc, 0b00),
            (_40perc, 0b01),
            (_45perc, 0b10),
            (_50perc, 0b11),
        ]
    );
    bitfield!(CcCtrl3, CcTrySnk, 3);
    bitfield!(CcCtrl3, CcPreferSnk, 2);
    bitrange!(CcCtrl3, CCDRPPhase, 1, 0, CCDRPPhaseEnum);

    register!(ChgInILim1);
    bitrange_enum!(
        ChgInILimEnum,
        u8,
        [
            (_100mA, 0b0000000),
            (_100mA, 0b0000001),
            (_100mA, 0b0000010),
            (_100mA, 0b0000011),
            (_133mA, 0b0000100),
            (_166mA, 0b0000101),
            (_200mA, 0b0000110),
            (_233mA, 0b0000111),
            (_266mA, 0b0001000),
            (_300mA, 0b0001001),
            (_333mA, 0b0001010),
            (_366mA, 0b0001011),
            (_400mA, 0b0001100),
            (_433mA, 0b0001101),
            (_466mA, 0b0001110),
            (_500mA, 0b0001111),
            (_533mA, 0b0010000),
            (_566mA, 0b0010001),
            (_600mA, 0b0010010),
            (_633mA, 0b0010011),
            (_666mA, 0b0010100),
            (_700mA, 0b0010101),
            (_733mA, 0b0010110),
            (_766mA, 0b0010111),
            (_800mA, 0b0011000),
            (_833mA, 0b0011001),
            (_866mA, 0b0011010),
            (_900mA, 0b0011011),
            (_933mA, 0b0011100),
            (_966mA, 0b0011101),
            (_1000mA, 0b0011110),
            (_1033mA, 0b0011111),
            (_1066mA, 0b0100000),
            (_1100mA, 0b0100001),
            (_1133mA, 0b0100010),
            (_1166mA, 0b0100011),
            (_1200mA, 0b0100100),
            (_1233mA, 0b0100101),
            (_1266mA, 0b0100110),
            (_1300mA, 0b0100111),
            (_1333mA, 0b0101000),
            (_1366mA, 0b0101001),
            (_1400mA, 0b0101010),
            (_1433mA, 0b0101011),
            (_1466mA, 0b0101100),
            (_1500mA, 0b0101101),
            (_1533mA, 0b0101110),
        ]
    );

    let max14748 = Max14748::new();
    max14748.DevStatus1().set();

    let bcstatus = max14748.BcStatus().VbusDet();
}
