#[cfg(test)]
mod tests {
    #[test]
    fn max14748() {
        use bitterly::{bitfield, bitrange, bitrange_enum, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        const REGISTER_COUNT: usize = 3;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum RegisterId {
            ChipId = 0x00,
            ChipRev = 0x01,
            DevStatus1 = 0x02,
            AiclStatus = 0x03,
        }

        struct Max14748 {
            registers: [Register; REGISTER_COUNT],
        }

        impl Max14748 {
            pub fn new() -> Self {
                Max14748 {
                    registers: [Register { contents: 0 }; REGISTER_COUNT],
                }
            }
        }

        let max14748 = Max14748::new();

        register!(Max14748, ChipId);

        bitfield!(ChipId, Sysfit, 7);

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

        // let x = max14748.ChipId().get_Sysfit();
        // max14748.ChipId().set_Sysfit(true);
        // let y = max14748.ChipId().get_Sysfit();

        //print!("{}", y);

        // static mut REGISTER: Register = Register { contents: 0 };

        // struct test {
        //     x: *mut Register,
        // }

        // impl test {
        //     fn new() -> Self {
        //         test {
        //             x: unsafe { &mut REGISTER },
        //         }
        //     }
        // }

        // let test = test::new();
        // unsafe {
        //     test.x.as_mut().unwrap().set_bit(0);
        //     test.x.as_mut().unwrap().set_bit(1);
        // }
        //test.x.set_bit(1);

        //bitfield!(id, 7, 0,);
    }

    #[test]
    fn register_define_test() {
        use bitterly::{bitfield, register, register_backer};

        register_backer!(Register, u8);

        const REGISTER_COUNT: usize = 1;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum RegisterId {
            ChipId = 0x00,
        }

        pub struct Max14748 {
            registers: [Register; REGISTER_COUNT],
        }

        impl Max14748 {
            pub fn new() -> Self {
                Max14748 {
                    registers: [Register { contents: 0 }; REGISTER_COUNT],
                }
            }
        }

        let mut max14748 = Max14748::new();

        register!(Max14748, ChipId);

        let id = max14748.ChipId().contents();
        assert_eq!(id, 0);

        // Set contents of register
        max14748.ChipId().update(8);

        let id = max14748.ChipId().contents();
        assert_eq!(id, 8);
    }

    #[test]
    pub fn bitfield_test() {
        use bitterly::{bitfield, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        const REGISTER_COUNT: usize = 4;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum RegisterId {
            ChipId = 0x00,
            ChipRev = 0x01,
            DevStatus1 = 0x02,
            AiclStatus = 0x03,
        }

        struct Max14748 {
            registers: [Register; REGISTER_COUNT],
        }

        impl Max14748 {
            pub fn new() -> Self {
                Max14748 {
                    registers: [Register { contents: 0 }; REGISTER_COUNT],
                }
            }
        }

        let max14748 = Max14748::new();

        register!(Max14748, ChipId);
        register!(Max14748, ChipRev);

        register!(Max14748, DevStatus1);
        bitfield!(DevStatus1, SysFit, 7);
        bitfield!(DevStatus1, ChgInOvp, 6);
        bitfield!(DevStatus1, ILim, 5);
        bitfield!(DevStatus1, VSysReg, 4);
        bitfield!(DevStatus1, ThrmSd150, 3);
        bitfield!(DevStatus1, ThrmSd120, 2);
        bitfield!(DevStatus1, BatDet, 1);
        bitfield!(DevStatus1, WbChg, 0);

        register!(Max14748, AiclStatus);

        let mut batdet = max14748.DevStatus1().get_BatDet();
        assert_eq!(batdet, false);
        max14748.DevStatus1().set_BatDet(true);
        batdet = max14748.DevStatus1().get_BatDet();
        assert_eq!(batdet, true);
    }

    #[test]
    pub fn bitrange_enum_test() {
        use bitterly::{bitfield, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        const REGISTER_COUNT: usize = 4;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum RegisterId {
            ChipId = 0x00,
            ChipRev = 0x01,
            DevStatus1 = 0x02,
            AiclStatus = 0x03,
        }

        struct Max14748 {
            registers: [Register; REGISTER_COUNT],
        }

        impl Max14748 {
            pub fn new() -> Self {
                Max14748 {
                    registers: [Register { contents: 0 }; REGISTER_COUNT],
                }
            }
        }

        let max14748 = Max14748::new();

        register!(Max14748, AiclStatus);
    }

    #[test]
    pub fn bitrange_test() {
        use bitterly::{bitfield, bitrange, bitrange_enum, register, register_backer};
        use paste::paste;

        register_backer!(Register, u8);

        const REGISTER_COUNT: usize = 4;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum RegisterId {
            ChipId = 0x00,
            ChipRev = 0x01,
            DevStatus1 = 0x02,
            AiclStatus = 0x03,
        }

        struct Max14748 {
            registers: [Register; REGISTER_COUNT],
        }

        impl Max14748 {
            pub fn new() -> Self {
                Max14748 {
                    registers: [Register { contents: 0 }; REGISTER_COUNT],
                }
            }
        }

        let max14748 = Max14748::new();

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
