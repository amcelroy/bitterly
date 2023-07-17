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

    let max14748 = Max14748::new();
    max14748.DevStatus1().set();

    let bcstatus = max14748.BcStatus().VbusDet();
}
