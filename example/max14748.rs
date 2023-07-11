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
    register!(Max14748, AiclStatus);
    bitrange!(AiclStatus, Status, 7, 5, AiclStatusEnum);
    bitrange!(AiclStatus, CurSet, 4, 0, AiclStatusEnum);

    let max14748 = Max14748::new();
}
