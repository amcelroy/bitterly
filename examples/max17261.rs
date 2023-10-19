#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bitterly::{
    bitfield, bitrange, bitrange_enum_values, bitrange_quantized, bitrange_raw, peripheral,
    register, register_backer,
};
use paste::paste;

const RSENSE: f32 = 0.01; // 10mOhm RSense resistor

const CAPACITY_QUANT: f32 = 5.0 / RSENSE; // Capacity in uV-hours
const CAPACITY_MIN: f32 = 0.0;
const CAPACITY_MAX: f32 = CAPACITY_QUANT * 65535.0;

const PERCENT_QUANT: f32 = 1.0 / 256.0; // Percent in 1/256
const PERCENT_MIN: f32 = 0.0;
const PERCENT_MAX: f32 = PERCENT_QUANT * 65535.0;

const VOLTAGE_QUANT: f32 = 0.00125 / 16.0; // Voltage in 12.5mV
const VOLTAGE_MIN: f32 = 0.0;
const VOLTAGE_MAX: f32 = VOLTAGE_QUANT * 65535.0;

const CURRENT_QUANT: f32 = 0.0000015625 / RSENSE;
const CURRENT_MIN: f32 = CURRENT_QUANT * -32768.0;
const CURRENT_MAX: f32 = CURRENT_QUANT * 32767.0;

const TEMPERATURE_QUANT: f32 = 1.0 / 256.0;
const TEMPERATURE_MIN: f32 = TEMPERATURE_QUANT * -32768.0;
const TEMPERATURE_MAX: f32 = TEMPERATURE_QUANT * 32767.0;

const RESISTANCE_QUANT: f32 = 1.0 / 4096.0;
const RESISTANCE_MIN: f32 = RESISTANCE_QUANT * 0.0;
const RESISTANCE_MAX: f32 = RESISTANCE_QUANT * 65535.0;

const TIME_QUANT: f32 = 5.625;
const TIME_MIN: f32 = TIME_QUANT * 0.0;
const TIME_MAX: f32 = TIME_QUANT * 65535.0;

// Create a u8 Register
register_backer!(Register, u16);

peripheral!(
    Max17261,
    0x0A,
    112,
    [
        // 0x00 to 0x10
        (Status, 0x00, 0),
        (VAlrtTh, 0x01, 1),
        (TAlrtTh, 0x02, 2),
        (SAlrtTh, 0x03, 3),
        (AtRate, 0x04, 4),
        (RepCap, 0x05, 5),
        (RepSOC, 0x06, 6),
        (AgePercent, 0x07, 7),
        (Temp, 0x08, 8),
        (VCell, 0x09, 9),
        (RSenseCurrent, 0x0A, 10),
        (AvgCurrent, 0x0B, 11),
        (QResidual, 0x0C, 12),
        (MixSOC, 0x0D, 13),
        (AvailableSOC, 0x0E, 14),
        (MixCap, 0x0F, 15),
        // 0x10 to 0x1F
        (FullCapRep, 0x10, 16),
        (TimeToEmpty, 0x11, 17),
        (QRTable00, 0x12, 18),
        (FullSOCThr, 0x13, 19),
        (RCell, 0x14, 20),
        (Reserved0x15, 0x15, 21),
        (AvgTA, 0x16, 22),
        (Cycles, 0x17, 23),
        (DesignCapacity, 0x18, 24),
        (AvgVCell, 0x19, 25),
        (MinMaxTemp, 0x1A, 26),
        (MinMaxVolt, 0x1B, 27),
        (MinMaxCurr, 0x1C, 28),
        (Config, 0x1D, 29),
        (IChgTerm, 0x1E, 30),
        (AvCap, 0x1F, 31),
        // 0x20 to 0x2F
        (TimeToFull, 0x20, 32),
        (DevName, 0x21, 33),
        (QRTable10, 0x22, 34),
        (FullCapNom, 0x23, 35),
        (Reserved0x24, 0x24, 36),
        (Reserved0x25, 0x25, 37),
        (Reserved0x26, 0x26, 38),
        (Ain, 0x27, 39),
        (LearnCfg, 0x28, 40),
        (FilterCfg, 0x29, 41),
        (RelaxCfg, 0x2A, 42),
        (MiscCfg, 0x2B, 43),
        (TGain, 0x2C, 44),
        (TOff, 0x2D, 45),
        (CGain, 0x2E, 46),
        (COff, 0x2F, 47),
        // 0x30 to 0x3F
        (Reserved0x30, 0x30, 48),
        (Reserved0x31, 0x31, 49),
        (QRTable20, 0x32, 50),
        (Reserved0x33, 0x33, 51),
        (DieTemp, 0x34, 52),
        (FullCap, 0x35, 53),
        (Reserved0x36, 0x36, 54),
        (Reserved0x37, 0x37, 55),
        (RComp0, 0x38, 56),
        (TempCo, 0x39, 57),
        (VEmpty, 0x3A, 58),
        (Reserved0x3B, 0x3B, 59),
        (Reserved0x3C, 0x3C, 60),
        (FStat, 0x3D, 61),
        (TimerLSB, 0x3E, 62),
        (ShdnTimer, 0x3F, 63),
        // 0x40 to 0x4F
        (Reserved0x40, 0x40, 64),
        (Reserved0x41, 0x41, 65),
        (QRTable30, 0x42, 66),
        (RGain, 0x43, 67),
        (Reserved0x44, 0x44, 68),
        (DQAcc, 0x45, 69),
        (DPAcc, 0x46, 70),
        (Reserved0x47, 0x47, 71),
        (Reserved0x48, 0x48, 72),
        (ConvgCfg, 0x49, 73),
        (VFRemCap, 0x4A, 74),
        (Reserved0x4B, 0x4B, 75),
        (Reserved0x4C, 0x4C, 76),
        (QH, 0x4D, 77),
        (Reserved0x4E, 0x4E, 78),
        (Reserved0x4F, 0x4F, 79),
        // 0xB0 to 0xBF
        (Status2, 0xB0, 80),
        (Power, 0xB1, 81),
        (Id, 0xB2, 82),
        (AvgPower, 0xB3, 83),
        (IAlertTh, 0xB4, 84),
        (TtfCfg, 0xB5, 85),
        (CvMixCap, 0xB6, 86),
        (CvHalfTime, 0xB7, 87),
        (CgTempCo, 0xB8, 88),
        (Curve, 0xB9, 89),
        (HibCfg, 0xBA, 90),
        (Config2, 0xBB, 91),
        (VRipple, 0xBC, 92),
        (RippleCfg, 0xBD, 93),
        (TimerHSB, 0xBE, 94),
        (Reserved0xBF, 0xBF, 95),
        // 0xD0 to 0xDF
        (RSense, 0xD0, 96),
        (ScOcvLim, 0xD1, 97),
        (VGain, 0xD2, 98),
        (SOCHold, 0xD3, 99),
        (MaxPeakPower, 0xD4, 100),
        (SusPeakPower, 0xD5, 101),
        (PackResistance, 0xD6, 102),
        (SysResistance, 0xD7, 103),
        (MinSysVoltage, 0xD8, 104),
        (MPPCurrent, 0xD9, 105),
        (SPPCurrent, 0xDA, 106),
        (ModelCfg, 0xDB, 107),
        (AtQResidual, 0xDC, 108),
        (AtTTE, 0xDD, 109),
        (AtAvSOC, 0xDE, 110),
        (AtAvCap, 0xDF, 111)
    ]
);

register!(Status);
bitfield!(Status, br, 15);
bitfield!(Status, smx, 14);
bitfield!(Status, tmx, 13);
bitfield!(Status, vmx, 12);
bitfield!(Status, bi, 11);
bitfield!(Status, smn, 10);
bitfield!(Status, tmn, 9);
bitfield!(Status, vmn, 8);
bitfield!(Status, dsoci, 7);
bitfield!(Status, imx, 6);
bitfield!(Status, bst, 3);
bitfield!(Status, imn, 2);
bitfield!(Status, por, 1);

register!(VAlrtTh);
bitrange_quantized!(VAlrtTh, VMax, 15, 8, u8, 0.02, 0.0, 5.1);
bitrange_quantized!(VAlrtTh, VMin, 7, 0, u8, 0.02, 0.0, 5.1);

register!(TAlrtTh);
bitrange_quantized!(TAlrtTh, TMax, 15, 8, u8, 1.0, -128.0, 128.0);
bitrange_quantized!(TAlrtTh, TMin, 7, 0, u8, 1.0, -128.0, 128.0);

register!(SAlrtTh);
bitrange_raw!(SAlrtTh, SMax, 15, 8, u8);
bitrange_raw!(SAlrtTh, SMin, 7, 0, u8);

register!(AtRate);
bitrange_quantized!(
    AtRate,
    rate,
    15,
    0,
    i16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(RepCap); // Capacity in mAH
bitrange_quantized!(
    RepCap,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(RepSOC); // State of Charge in %, typically used for a GUI
bitrange_quantized!(
    RepSOC,
    soc,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(AgePercent); // Age of the battery in %, based on (FullCapRep / DesignCap)*100
bitrange_quantized!(
    AgePercent,
    age,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(Temp); // Temperature of the Max17261 in degrees C
bitrange_quantized!(
    Temp,
    temperature,
    15,
    0,
    i16,
    TEMPERATURE_QUANT,
    TEMPERATURE_MIN,
    TEMPERATURE_MAX
);

register!(VCell); // Voltage cell of the battery pack
bitrange_quantized!(
    VCell,
    voltage,
    15,
    0,
    u16,
    VOLTAGE_QUANT,
    VOLTAGE_MIN,
    VOLTAGE_MAX
);

register!(RSenseCurrent); // Voltge across the sense resistor, .01mOhm for our design
bitrange_quantized!(
    RSenseCurrent,
    current,
    15,
    0,
    i16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(AvgCurrent); // Average current of RSenseCurrent
bitrange_quantized!(
    AvgCurrent,
    current,
    15,
    0,
    i16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(QResidual); // Calculated charge in mAh that can't be accessed
bitrange_quantized!(
    QResidual,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(MixSOC); // State of charge prior to compensation algorithms
bitrange_quantized!(
    MixSOC,
    soc,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(AvailableSOC); // Available State of Charge after compensation algorithms
bitrange_quantized!(
    AvailableSOC,
    soc,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(MixCap); // Capacity prior to compensation algorithms
bitrange_quantized!(
    MixCap,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

// Start of 0x10 to 0x19 registers

register!(FullCapRep); // Full capacity of the battery pack, typically reported to GUI
bitrange_quantized!(
    FullCapRep,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(TimeToEmpty); // Time to empty in seconds
bitrange_quantized!(
    TimeToEmpty,
    time,
    15,
    0,
    u16,
    TIME_QUANT,
    TIME_MIN,
    TIME_MAX
);

register!(QRTable00);

register!(FullSOCThr); // Full SOC threshold - defaults to 95%
bitrange_quantized!(
    FullSOCThr,
    threshold,
    15,
    3,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(RCell); // Resistance of the cell
bitrange_quantized!(
    RCell,
    resistance,
    15,
    0,
    u16,
    RESISTANCE_QUANT,
    RESISTANCE_MIN,
    RESISTANCE_MAX
);

register!(AvgTA); // Average Temperature from the Temp register
bitrange_quantized!(
    AvgTA,
    temperature,
    15,
    0,
    i16,
    TEMPERATURE_QUANT,
    TEMPERATURE_MIN,
    TEMPERATURE_MAX
);

register!(Cycles); // Battery cycles
bitrange_quantized!(Cycles, cycles, 15, 0, u16, 0.01, 0.0, 655.35);

register!(DesignCapacity); // Design capacity of the battery pack
bitrange_quantized!(
    DesignCapacity,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(AvgVCell); // Avg voltage of the cell
bitrange_quantized!(
    AvgVCell,
    voltage,
    15,
    0,
    u16,
    VOLTAGE_QUANT,
    VOLTAGE_MIN,
    VOLTAGE_MAX
);

register!(MinMaxTemp);
bitrange_quantized!(
    MinMaxTemp,
    max,
    15,
    8,
    i8,
    TEMPERATURE_QUANT,
    TEMPERATURE_MIN,
    TEMPERATURE_MAX
);
bitrange_quantized!(
    MinMaxTemp,
    min,
    7,
    0,
    i8,
    TEMPERATURE_QUANT,
    TEMPERATURE_MIN,
    TEMPERATURE_MAX
);

const MIN_MAX_VOLT_QUANT: f32 = 0.02; // 20mV
register!(MinMaxVolt);
bitrange_quantized!(
    MinMaxVolt,
    max,
    15,
    8,
    u8,
    MIN_MAX_VOLT_QUANT,
    0.0,
    i8::MAX as f32 * MIN_MAX_VOLT_QUANT
);
bitrange_quantized!(
    MinMaxVolt,
    min,
    7,
    0,
    u8,
    MIN_MAX_VOLT_QUANT,
    0.0,
    i8::MAX as f32 * MIN_MAX_VOLT_QUANT
);

const MIN_MAX_CURR_QUANT: f32 = 0.0004 / RSENSE; // .4mV / RSENSE
register!(MinMaxCurr);
bitrange_quantized!(
    MinMaxCurr,
    max,
    15,
    8,
    i8,
    MIN_MAX_CURR_QUANT,
    i8::MIN as f32 * MIN_MAX_CURR_QUANT,
    i8::MAX as f32 * MIN_MAX_CURR_QUANT
);
bitrange_quantized!(
    MinMaxCurr,
    min,
    7,
    0,
    i8,
    MIN_MAX_CURR_QUANT,
    i8::MIN as f32 * MIN_MAX_CURR_QUANT,
    i8::MAX as f32 * MIN_MAX_CURR_QUANT
);

register!(Config);
bitfield!(Config, tsel, 15); // 0 use internal temp, 1 external thermister
bitfield!(Config, ss, 14); // SOC: 1 alerts only cleared through software, 0 alerts auto cleared when measurment drops below threshold
bitfield!(Config, ts, 13); // Temp: 1 alerts only cleared through software, 0 alerts auto cleared when measurment drops below threshold
bitfield!(Config, vs, 12); // Voltage: 1 alerts only cleared through software, 0 alerts auto cleared when measurment drops below threshold
bitfield!(Config, is, 11); // Current: 1 alerts only cleared through software, 0 alerts auto cleared when measurment drops below threshold
bitfield!(Config, thsh, 10); // Set to 1 to auto shutdown device when TH pin is not connected
bitfield!(Config, ten, 9); // Temp channel, see datasheet
bitfield!(Config, tex, 8); // Set to 0 to use on chip temps
bitfield!(Config, shdn, 7); // Set to 1 to shutdown chip after ShdnTimer expires
bitfield!(Config, commsh, 6); // Set to 1 to shutdown chip after ShdnTimer expires if no I2C traffic
bitfield!(Config, ethrm, 4); // Set to 1 to measure TH pin
bitfield!(Config, fthrm, 3); // Set to 1 to enable thermister bias
bitfield!(Config, aen, 2); // Set to 1 to enable alerts to propigate to ALRT pin
bitfield!(Config, bei, 1); // Set to 1 to enable alert when battery is inserted (see TH pin)
bitfield!(Config, ber, 0); // Set to 1 to enable alert when battery is removed (see TH pin)

register!(IChgTerm); // Charge termination current (see datasheet)
bitrange_quantized!(
    IChgTerm,
    current,
    15,
    0,
    u16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(AvCap); // Available capacity of the battery pack
bitrange_quantized!(
    AvCap,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

// 0x20 to 0x2F
register!(TimeToFull); // Time to Full in seconds
bitrange_quantized!(TimeToFull, time, 15, 0, u16, TIME_QUANT, TIME_MIN, TIME_MAX);

register!(DevName); // Device name, should be 0x4033 for the MAX17261

register!(QRTable10);

register!(FullCapNom); // Full discharge capacity with compensation under present conditions
bitrange_quantized!(
    FullCapNom,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(Ain); // External thermister value compared to Batt pin voltage

register!(LearnCfg);
bitrange_raw!(LearnCfg, ls, 6, 4, u8);

register!(FilterCfg);
bitrange_raw!(FilterCfg, temp, 13, 11, u8);
bitrange_raw!(FilterCfg, mix, 10, 7, u8);
bitrange_raw!(FilterCfg, volt, 6, 4, u8);
bitrange_raw!(FilterCfg, curr, 3, 0, u8);

register!(RelaxCfg);
bitrange_raw!(RelaxCfg, load, 15, 9, u8);
bitrange_raw!(RelaxCfg, dv, 8, 4, u8);
bitrange_raw!(RelaxCfg, dt, 3, 0, u8);

register!(MiscCfg);
bitrange_enum_values!(
    SOCAlertConfigEnum,
    u16,
    [(RepSOC, 0), (AvSOC, 1), (MixSOC, 2), (VfSOC, 3)]
);
bitrange_raw!(MiscCfg, fus, 15, 12, u8);
bitrange_raw!(MiscCfg, mr, 9, 5, u8);
bitrange!(MiscCfg, sacfg, 1, 0, SOCAlertConfigEnum);

register!(TGain); // Thermister gain (see datasheet)

register!(TOff); // Thermister offset (see datasheet)

register!(CGain); // Current gain (see datasheet)

register!(COff); // Current offset (see datasheet

// Registers 0x30 to 0x3F
register!(QRTable20);

register!(DieTemp); // Internal die temp
bitrange_quantized!(
    DieTemp,
    Temperature,
    15,
    0,
    i16,
    TEMPERATURE_QUANT,
    TEMPERATURE_MIN,
    TEMPERATURE_MAX
);

register!(FullCap); // Full battery capacity, compensated, at present conditions
bitrange_quantized!(
    FullCap,
    Capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(RComp0); // Special values for open voltage calculations

register!(TempCo); // Temperate compensation values for RComp0

register!(VEmpty);
const VEMPTY_VE_QUANT: f32 = 0.01; // 1mV
bitrange_quantized!(
    VEmpty,
    ve,
    15,
    7,
    u16,
    VEMPTY_VE_QUANT,
    0.0,
    512.0 * VEMPTY_VE_QUANT // 2^9 * VEMPTY_VR_QUANT
);
const VEMPTY_VR_QUANT: f32 = 0.04; // 40mV
bitrange_quantized!(
    VEmpty,
    vr,
    6,
    0,
    u16,
    VEMPTY_VR_QUANT,
    0.0,
    128.0 * VEMPTY_VR_QUANT // 2^7 * VEMPTY_VR_QUANT
);

register!(FStat);
bitfield!(FStat, reldt, 9); // If 1, cell is fully relaxed
bitfield!(FStat, edet, 8); //  If 1, cell is empty, see VEmpty
bitfield!(FStat, fq, 7); // If 1, all charge termination conditions met
bitfield!(FStat, reldt2, 6); // If 1, cell has been relaxed for 48 to 96 minutes, or longer
bitfield!(FStat, dnr, 0); // If 1, battery is inserted and registers may be invalid

register!(TimerLSB); // Least significant bytes of the timer register
const TIMER_LSB_QUANT: f32 = 0.1758; //175.8ms
bitrange_quantized!(
    TimerLSB,
    Time,
    15,
    0,
    u16,
    TIMER_LSB_QUANT,
    0.0,
    65535.0 * TIMER_LSB_QUANT
);

register!(ShdnTimer); // Shutdown timer
const SHDN_CTR_QUANT: f32 = 1.4; // 1.4s
bitrange_enum_values!(TimeoutPeriodEnum, u16, [(_45s, 0), (_1_6h, 0b111)]);
bitrange!(ShdnTimer, thr, 15, 13, TimeoutPeriodEnum);
bitrange_quantized!(
    ShdnTimer,
    ctr,
    12,
    0,
    u16,
    SHDN_CTR_QUANT,
    0.0,
    8191.0 * SHDN_CTR_QUANT
);

register!(QRTable30);

register!(RGain);

register!(DQAcc); // Charge between relaxation points, debug register
bitrange_quantized!(
    DQAcc,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(DPAcc);
bitrange_quantized!(
    DPAcc,
    percent,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(ConvgCfg); // Configures operation of the converge-to-empty feature

register!(VFRemCap); // Remaining capacity WITHOUT compensation / adjustments
bitrange_quantized!(
    VFRemCap,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(QH); // Raw coulomb counter value, useful for debugging
bitrange_quantized!(
    QH,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(Status2);
bitfield!(Status2, at_rate_ready, 13); // 1 if AtRate registers are ready to be read
bitfield!(Status2, dp_ready, 12); // 1 if Dynamic Power registers are ready
bitfield!(Status2, sn_ready, 8); // 1 if the SN can be read over i2c
bitfield!(Status2, full_det, 5); // 1 if full detected
bitfield!(Status2, hib, 1); // 1 if hibernate, 0 otherwise

register!(Power);
const POWER_QUANT: f32 = 0.000008 / RSENSE; // 8uV^2 / RSENSE
bitrange_quantized!(
    Power,
    power,
    15,
    0,
    u16,
    POWER_QUANT,
    0.0,
    65535.0 * POWER_QUANT
);

register!(Id);

register!(AvgPower);
bitrange_quantized!(
    AvgPower,
    avg_power,
    15,
    0,
    u16,
    POWER_QUANT,
    0.0,
    65535.0 * POWER_QUANT
);

register!(IAlertTh); // Optional current alert threshold
const IALERT_QUANT: f32 = 0.0004 / RSENSE; // .4mV / RSENSE
bitrange_quantized!(
    IAlertTh,
    imax,
    15,
    8,
    i8,
    IALERT_QUANT,
    i8::MIN as f32 * IALERT_QUANT,
    i8::MAX as f32 * IALERT_QUANT
);
bitrange_quantized!(
    IAlertTh,
    imin,
    7,
    0,
    i8,
    IALERT_QUANT,
    i8::MIN as f32 * IALERT_QUANT,
    i8::MAX as f32 * IALERT_QUANT
);

register!(TtfCfg); // No documentation, time to full?

register!(CvMixCap); // No documentation, capacity?

register!(CvHalfTime); // No documentation, time?

register!(CgTempCo); // Copper temperature compensation
                     //Should be set to 0x20C8 (.4%) at boot if copper is used
const CGTEMP_MAX: f32 = 3.1224;
const CGTEMP_QUANT: f32 = CGTEMP_MAX / 65536.0; // % per degree C, see datasheet
bitrange_quantized!(CgTempCo, cgtemp, 15, 0, u16, CGTEMP_QUANT, 0.0, CGTEMP_MAX);

register!(Curve); // Not used when measuring internal temperature
                  // TODO: This is a special register but doesn't have quanitzation values
                  // and has been left as a raw u16 for now

register!(HibCfg); // See datasheet for mapping bitranges to times
bitfield!(HibCfg, en_hib, 15);
bitrange_raw!(HibCfg, hib_enter_time, 14, 12, u16);
bitrange_raw!(HibCfg, hib_threshold, 11, 8, u16);
bitrange_raw!(HibCfg, hib_exit_time, 4, 3, u16);
bitrange_raw!(HibCfg, hib_scalar, 2, 0, u16);

register!(Config2);
bitfield!(Config2, at_rate_en, 13); // When 0, AtRate calculation are disabled
bitfield!(Config2, dp_en, 12); // When 0, dynamic power calculations are 0
bitrange_raw!(Config2, powr, 11, 8, u16);
bitfield!(Config2, d_soc_en, 7); // When 1, soc alerts propigate to ALRT pin
bitfield!(Config2, t_alert_en, 6); // When 1, temp alerts propigate to ALRT pin
bitfield!(Config2, load_model, 5); // When 1, asks chip to load a new model, goes to 0 when finished
bitrange_enum_values!(
    Config2DRCfgEnum,
    u16,
    [(_1_6h, 0b00), (_3_2h, 0b01), (_6_4h, 0b10), (_12_8h, 0b11)]
);
bitrange!(Config2, dr_cfg, 3, 2, Config2DRCfgEnum);
bitfield!(Config2, cp_mode, 1); // When 1, chip is in constant power mode

register!(VRipple); // Ripple value of VCell register
const VRIPPLE_QUANT: f32 = 0.00125 / 128.0; // 1.25mV / 128
bitrange_quantized!(
    VRipple,
    ripple,
    15,
    0,
    u16,
    VRIPPLE_QUANT,
    0.0,
    65535.0 * VRIPPLE_QUANT
);

register!(RippleCfg); // Not recommended to change

register!(TimerHSB);
const TIMER_HSB_QUANT: f32 = 11520.0; // The TimerLSB is 0 to 3.2 hours, so the resolution on the HSB is 3.2 hours or 115200 seconds.
bitrange_quantized!(
    TimerHSB,
    Time,
    15,
    0,
    u16,
    TIMER_HSB_QUANT,
    0.0,
    65535.0 * TIMER_HSB_QUANT
);

register!(RSense); // Not documented, resister value?
bitrange_quantized!(
    RSense,
    Resistance,
    15,
    0,
    u16,
    RESISTANCE_QUANT,
    RESISTANCE_MIN,
    RESISTANCE_MAX
);

register!(ScOcvLim); // Not used for Li-Ion, used for LiFePO4
const OCV_LOW_LIM_QUANT: f32 = 0.005; // 5mV
const OCV_LOW_LIM_MIN: f32 = 2.65; // 2.65V
const OCV_LOW_LIM_MAX: f32 = 5.12; // 5.12V
bitrange_quantized!(
    ScOcvLim,
    ocv_low_lim,
    15,
    7,
    u8,
    OCV_LOW_LIM_QUANT,
    OCV_LOW_LIM_MIN,
    OCV_LOW_LIM_MAX
);
const OCV_DELTA_QUANT: f32 = 0.0025; // 2.5mV
const OCV_DELTA_MIN: f32 = 0.0; // 0V
const OCV_DELTA_MAX: f32 = 128.0 * OCV_DELTA_QUANT; // 2^7 * 2.5mV
bitrange_quantized!(
    ScOcvLim,
    ocv_delta,
    6,
    0,
    u8,
    OCV_DELTA_QUANT,
    OCV_DELTA_MIN,
    OCV_DELTA_MAX
);

register!(VGain); // Not documented, voltage gain?

register!(SOCHold);
bitfield!(SOCHold, hold_en_99p, 12); // If 1, Holds RepSOC @ 99% until Full Qualified is reached
const SOC_HOLD_EMPTY_VOLT_QUANT: f32 = 0.01; // 10mV
bitrange_quantized!(
    SOCHold,
    empty,
    11,
    5,
    u16,
    SOC_HOLD_EMPTY_VOLT_QUANT,
    0.0,
    127.0 * SOC_HOLD_EMPTY_VOLT_QUANT
);
const SOC_HOLD_EMPTY_SOC_QUANT: f32 = 0.5; // 0.5%
bitrange_quantized!(
    SOCHold,
    empty_soc,
    4,
    0,
    u16,
    SOC_HOLD_EMPTY_SOC_QUANT,
    0.0,
    31.0 * SOC_HOLD_EMPTY_SOC_QUANT
);

register!(MaxPeakPower); // Estimated peak power discharge
const MAX_PEAK_POWER_QUANT: f32 = 0.0008; // .8mW
bitrange_quantized!(
    MaxPeakPower,
    power,
    15,
    0,
    u16,
    MAX_PEAK_POWER_QUANT,
    0.0,
    65535.0 * MAX_PEAK_POWER_QUANT
);

register!(SusPeakPower);
const SUS_PEAK_POWER_QUANT: f32 = 0.0008; // .8mW
bitrange_quantized!(
    SusPeakPower,
    power,
    15,
    0,
    u16,
    SUS_PEAK_POWER_QUANT,
    0.0,
    65535.0 * SUS_PEAK_POWER_QUANT
);

register!(PackResistance); // Pack resistance, should include Fuses, interconnects, sense reistor, etc.
const PACK_RESISTANCE_QUANT: f32 = 0.000244140625; // in Ohm
bitrange_quantized!(
    PackResistance,
    resistance,
    15,
    0,
    u16,
    PACK_RESISTANCE_QUANT,
    0.0,
    65535.0 * PACK_RESISTANCE_QUANT
);

register!(SysResistance); // System resistance, estimated when battery is inserted
const SYS_RESISTANCE_QUANT: f32 = 0.000244140625; // in Ohm
bitrange_quantized!(
    SysResistance,
    resistance,
    15,
    0,
    u16,
    PACK_RESISTANCE_QUANT,
    0.0,
    65535.0 * PACK_RESISTANCE_QUANT
);

register!(MinSysVoltage); // Minimum system voltage, set to 3.0V by default
bitrange_quantized!(
    MinSysVoltage,
    voltage,
    15,
    0,
    u16,
    VOLTAGE_QUANT,
    VOLTAGE_MIN,
    VOLTAGE_MAX
);

register!(MPPCurrent); // Max instantaneous peak current in mA
bitrange_quantized!(
    MPPCurrent,
    current,
    15,
    0,
    u16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(SPPCurrent); // Max sustaint current
bitrange_quantized!(
    SPPCurrent,
    current,
    15,
    0,
    u16,
    CURRENT_QUANT,
    CURRENT_MIN,
    CURRENT_MAX
);

register!(ModelCfg); // Model configuration
bitrange_enum_values!(
    ModelCfgIdEnum,
    u16,
    [
        (LithiumCobaltOxide, 0b0000),
        (LithiumNcrNca, 0b0010),
        (LithiumIronPhosphate, 0b0110)
    ]
);
bitfield!(ModelCfg, refresh, 15); // 1 - Refresh the model, 0 model is refreshed
bitfield!(ModelCfg, r100, 13); // 1 if 100kOhm NTC, 0 if using 10kOhm NTC
bitfield!(ModelCfg, v_chg, 10); // 1 if VChg is higher than 4.25V, 0 if VChg is 4.2V
bitrange!(ModelCfg, id, 7, 4, ModelCfgIdEnum);

register!(AtQResidual); // Theoretical charge that is inaccessible
bitrange_quantized!(
    AtQResidual,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);

register!(AtTTE); // Estimate time to empty based on AtRate entry, this is THEORETICAL and does not impact the gauge
bitrange_quantized!(AtTTE, time, 15, 0, u16, TIME_QUANT, TIME_MIN, TIME_MAX);

register!(AtAvSOC); // Estimate of the SOC based on a a theorteical load in the AtRate register
bitrange_quantized!(
    AtAvSOC,
    soc,
    15,
    0,
    u16,
    PERCENT_QUANT,
    PERCENT_MIN,
    PERCENT_MAX
);

register!(AtAvCap); // Estimate of the capacity based on a theoretical load in the AtRate register
bitrange_quantized!(
    AtAvCap,
    capacity,
    15,
    0,
    u16,
    CAPACITY_QUANT,
    CAPACITY_MIN,
    CAPACITY_MAX
);
