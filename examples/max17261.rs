pub fn main() {
    pub enum Max17261Error {
        InvalidVoltage,
        InvalidPercentage,
        InvalidTemperature,
        InvalidCurrent,
        InvlidResistance,
        InvalidTime,
    }

    pub trait Voltage {
        fn to_voltage(&self) -> f32;
        fn from_voltage(voltage: f32) -> Result<(), Max17261Error>;
    }

    pub trait Percentage {
        fn to_percentage(&self) -> f32;
        fn from_percentage(percentage: f32) -> Result<(), Max17261Error>;
    }

    pub trait Temperature {
        fn to_temperature(&self) -> f32;
        fn from_temperature(temperature: f32) -> Result<(), Max17261Error>;
    }

    pub trait Current {
        fn to_current(&self) -> f32;
        fn from_current(current: f32) -> Result<(), Max17261Error>;
    }

    pub trait Resistance {
        fn to_resistance(&self) -> f32;
        fn from_resistance(resistance: f32) -> Result<(), Max17261Error>;
    }

    pub trait Time {
        fn to_time(&self) -> f32;
        fn from_time(time: f32) -> Result<(), Max17261Error>;
    }

    macro_rules! voltage {
        ($register:ident, $quantization:literal) => {
            paste! {
                impl Voltage for $register {
                    fn to_voltage(&self) -> f32 {
                        self.register.as_mut().unwrap().contents() as f32 * $quantization as f32;
                    }

                    fn from_voltage(&mut self, value: f32) {
                        if value < 0.0 || value > u16::MAX as f32 * $quantization as f32 {
                            Err(Max17261Error::InvalidVoltage)
                        }else{
                            self.register.as_mut().unwrap().set_contents((value / $quantization as f32) as u16);
                            Ok(())
                        }
                    }
                }
            }
        };
    }

    use bitterly::{
        bitfield, bitrange, bitrange_enum_values, bitrange_raw, peripheral, register,
        register_backer,
    };
    use paste::paste;

    // Create a u8 Register
    register_backer!(Register, u16);

    peripheral!(
        Max17261,
        0x0A,
        16,
        [
            (Status, 0x00, 0),
            (VAlrtTh, 0x01, 1),
            (TAlrtTh, 0x02, 2),
            (SAlrtTh, 0x03, 3),
            (AtRate, 0x04, 4),
            (RepCap, 0x05, 5),
            (RepSOC, 0x06, 6),
            (Age, 0x07, 7),
            (Temp, 0x08, 8),
            (VCell, 0x09, 9),
            (RSenseCurrent, 0x0A, 10),
            (AvgCurrent, 0x0B, 11),
            (QResidual, 0x0C, 12),
            (MixSOC, 0x0D, 13),
            (AvailableSOC, 0x0E, 14),
            (MixCap, 0x0F, 15)
        ]
    );

    register!(Status);
    bitfield!(Status, Br, 15);
    bitfield!(Status, Smx, 14);
    bitfield!(Status, Tmx, 13);
    bitfield!(Status, Vmx, 12);
    bitfield!(Status, Bi, 11);
    bitfield!(Status, Smn, 10);
    bitfield!(Status, Tmn, 9);
    bitfield!(Status, Vmn, 8);
    bitfield!(Status, dSOCi, 7);
    bitfield!(Status, Imx, 6);
    bitfield!(Status, Bst, 3);
    bitfield!(Status, Imn, 2);
    bitfield!(Status, POR, 1);

    register!(VAlrtTh);
    bitrange_raw!(VAlrtTh, VMax, 15, 8, u8);
    bitrange_raw!(VAlrtTh, VMin, 7, 0, u8);

    register!(TAlrtTh);
    bitrange_raw!(TAlrtTh, TMax, 15, 8, u8);
    bitrange_raw!(TAlrtTh, TMin, 7, 0, u8);

    register!(SAlrtTh);
    bitrange_raw!(SAlrtTh, SMax, 15, 8, u8);
    bitrange_raw!(SAlrtTh, SMin, 7, 0, u8);

    register!(AtRate);

    register!(RepCap);

    register!(RepSOC);

    register!(Age);

    register!(Temp);

    register!(VCell);

    register!(RSenseCurrent);

    register!(AvgCurrent);

    register!(QResidual);

    register!(MixSOC);

    register!(AvailableSOC);

    register!(MixCap);
}
