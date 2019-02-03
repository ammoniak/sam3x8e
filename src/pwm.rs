#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub clk: CLK,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: ENA,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: DIS,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: SR,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub ier1: IER1,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub idr1: IDR1,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub imr1: IMR1,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub isr1: ISR1,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub scm: SCM,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub scuc: SCUC,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub scup: SCUP,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub scupupd: SCUPUPD,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub ier2: IER2,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub idr2: IDR2,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub imr2: IMR2,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub isr2: ISR2,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub oov: OOV,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub os: OS,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub oss: OSS,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub osc: OSC,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub ossupd: OSSUPD,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub oscupd: OSCUPD,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub fmr: FMR,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub fsr: FSR,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub fcr: FCR,
    #[doc = "0x68 - PWM Fault Protection Value Register"]
    pub fpv: FPV,
    #[doc = "0x6c - PWM Fault Protection Enable Register 1"]
    pub fpe1: FPE1,
    #[doc = "0x70 - PWM Fault Protection Enable Register 2"]
    pub fpe2: FPE2,
    _reserved28: [u8; 8usize],
    #[doc = "0x7c - PWM Event Line 0 Mode Register"]
    pub elmr: [ELMR; 2],
    _reserved29: [u8; 44usize],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: SMMR,
    _reserved30: [u8; 48usize],
    #[doc = "0xe4 - PWM Write Protect Control Register"]
    pub wpcr: WPCR,
    #[doc = "0xe8 - PWM Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved32: [u8; 28usize],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    _reserved34: [u8; 8usize],
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: TNPR,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: TNCR,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
    _reserved38: [u8; 8usize],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub cmpv0: CMPV0,
    #[doc = "0x134 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd0: CMPVUPD0,
    #[doc = "0x138 - PWM Comparison 0 Mode Register"]
    pub cmpm0: CMPM0,
    #[doc = "0x13c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd0: CMPMUPD0,
    #[doc = "0x140 - PWM Comparison 1 Value Register"]
    pub cmpv1: CMPV1,
    #[doc = "0x144 - PWM Comparison 1 Value Update Register"]
    pub cmpvupd1: CMPVUPD1,
    #[doc = "0x148 - PWM Comparison 1 Mode Register"]
    pub cmpm1: CMPM1,
    #[doc = "0x14c - PWM Comparison 1 Mode Update Register"]
    pub cmpmupd1: CMPMUPD1,
    #[doc = "0x150 - PWM Comparison 2 Value Register"]
    pub cmpv2: CMPV2,
    #[doc = "0x154 - PWM Comparison 2 Value Update Register"]
    pub cmpvupd2: CMPVUPD2,
    #[doc = "0x158 - PWM Comparison 2 Mode Register"]
    pub cmpm2: CMPM2,
    #[doc = "0x15c - PWM Comparison 2 Mode Update Register"]
    pub cmpmupd2: CMPMUPD2,
    #[doc = "0x160 - PWM Comparison 3 Value Register"]
    pub cmpv3: CMPV3,
    #[doc = "0x164 - PWM Comparison 3 Value Update Register"]
    pub cmpvupd3: CMPVUPD3,
    #[doc = "0x168 - PWM Comparison 3 Mode Register"]
    pub cmpm3: CMPM3,
    #[doc = "0x16c - PWM Comparison 3 Mode Update Register"]
    pub cmpmupd3: CMPMUPD3,
    #[doc = "0x170 - PWM Comparison 4 Value Register"]
    pub cmpv4: CMPV4,
    #[doc = "0x174 - PWM Comparison 4 Value Update Register"]
    pub cmpvupd4: CMPVUPD4,
    #[doc = "0x178 - PWM Comparison 4 Mode Register"]
    pub cmpm4: CMPM4,
    #[doc = "0x17c - PWM Comparison 4 Mode Update Register"]
    pub cmpmupd4: CMPMUPD4,
    #[doc = "0x180 - PWM Comparison 5 Value Register"]
    pub cmpv5: CMPV5,
    #[doc = "0x184 - PWM Comparison 5 Value Update Register"]
    pub cmpvupd5: CMPVUPD5,
    #[doc = "0x188 - PWM Comparison 5 Mode Register"]
    pub cmpm5: CMPM5,
    #[doc = "0x18c - PWM Comparison 5 Mode Update Register"]
    pub cmpmupd5: CMPMUPD5,
    #[doc = "0x190 - PWM Comparison 6 Value Register"]
    pub cmpv6: CMPV6,
    #[doc = "0x194 - PWM Comparison 6 Value Update Register"]
    pub cmpvupd6: CMPVUPD6,
    #[doc = "0x198 - PWM Comparison 6 Mode Register"]
    pub cmpm6: CMPM6,
    #[doc = "0x19c - PWM Comparison 6 Mode Update Register"]
    pub cmpmupd6: CMPMUPD6,
    #[doc = "0x1a0 - PWM Comparison 7 Value Register"]
    pub cmpv7: CMPV7,
    #[doc = "0x1a4 - PWM Comparison 7 Value Update Register"]
    pub cmpvupd7: CMPVUPD7,
    #[doc = "0x1a8 - PWM Comparison 7 Mode Register"]
    pub cmpm7: CMPM7,
    #[doc = "0x1ac - PWM Comparison 7 Mode Update Register"]
    pub cmpmupd7: CMPMUPD7,
    _reserved70: [u8; 80usize],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr0: CMR0,
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty0: CDTY0,
    #[doc = "0x208 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub cdtyupd0: CDTYUPD0,
    #[doc = "0x20c - PWM Channel Period Register (ch_num = 0)"]
    pub cprd0: CPRD0,
    #[doc = "0x210 - PWM Channel Period Update Register (ch_num = 0)"]
    pub cprdupd0: CPRDUPD0,
    #[doc = "0x214 - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt0: CCNT0,
    #[doc = "0x218 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub dt0: DT0,
    #[doc = "0x21c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub dtupd0: DTUPD0,
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    pub cmr1: CMR1,
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    pub cdty1: CDTY1,
    #[doc = "0x228 - PWM Channel Duty Cycle Update Register (ch_num = 1)"]
    pub cdtyupd1: CDTYUPD1,
    #[doc = "0x22c - PWM Channel Period Register (ch_num = 1)"]
    pub cprd1: CPRD1,
    #[doc = "0x230 - PWM Channel Period Update Register (ch_num = 1)"]
    pub cprdupd1: CPRDUPD1,
    #[doc = "0x234 - PWM Channel Counter Register (ch_num = 1)"]
    pub ccnt1: CCNT1,
    #[doc = "0x238 - PWM Channel Dead Time Register (ch_num = 1)"]
    pub dt1: DT1,
    #[doc = "0x23c - PWM Channel Dead Time Update Register (ch_num = 1)"]
    pub dtupd1: DTUPD1,
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    pub cmr2: CMR2,
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    pub cdty2: CDTY2,
    #[doc = "0x248 - PWM Channel Duty Cycle Update Register (ch_num = 2)"]
    pub cdtyupd2: CDTYUPD2,
    #[doc = "0x24c - PWM Channel Period Register (ch_num = 2)"]
    pub cprd2: CPRD2,
    #[doc = "0x250 - PWM Channel Period Update Register (ch_num = 2)"]
    pub cprdupd2: CPRDUPD2,
    #[doc = "0x254 - PWM Channel Counter Register (ch_num = 2)"]
    pub ccnt2: CCNT2,
    #[doc = "0x258 - PWM Channel Dead Time Register (ch_num = 2)"]
    pub dt2: DT2,
    #[doc = "0x25c - PWM Channel Dead Time Update Register (ch_num = 2)"]
    pub dtupd2: DTUPD2,
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    pub cmr3: CMR3,
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    pub cdty3: CDTY3,
    #[doc = "0x268 - PWM Channel Duty Cycle Update Register (ch_num = 3)"]
    pub cdtyupd3: CDTYUPD3,
    #[doc = "0x26c - PWM Channel Period Register (ch_num = 3)"]
    pub cprd3: CPRD3,
    #[doc = "0x270 - PWM Channel Period Update Register (ch_num = 3)"]
    pub cprdupd3: CPRDUPD3,
    #[doc = "0x274 - PWM Channel Counter Register (ch_num = 3)"]
    pub ccnt3: CCNT3,
    #[doc = "0x278 - PWM Channel Dead Time Register (ch_num = 3)"]
    pub dt3: DT3,
    #[doc = "0x27c - PWM Channel Dead Time Update Register (ch_num = 3)"]
    pub dtupd3: DTUPD3,
    #[doc = "0x280 - PWM Channel Mode Register (ch_num = 4)"]
    pub cmr4: CMR4,
    #[doc = "0x284 - PWM Channel Duty Cycle Register (ch_num = 4)"]
    pub cdty4: CDTY4,
    #[doc = "0x288 - PWM Channel Duty Cycle Update Register (ch_num = 4)"]
    pub cdtyupd4: CDTYUPD4,
    #[doc = "0x28c - PWM Channel Period Register (ch_num = 4)"]
    pub cprd4: CPRD4,
    #[doc = "0x290 - PWM Channel Period Update Register (ch_num = 4)"]
    pub cprdupd4: CPRDUPD4,
    #[doc = "0x294 - PWM Channel Counter Register (ch_num = 4)"]
    pub ccnt4: CCNT4,
    #[doc = "0x298 - PWM Channel Dead Time Register (ch_num = 4)"]
    pub dt4: DT4,
    #[doc = "0x29c - PWM Channel Dead Time Update Register (ch_num = 4)"]
    pub dtupd4: DTUPD4,
    #[doc = "0x2a0 - PWM Channel Mode Register (ch_num = 5)"]
    pub cmr5: CMR5,
    #[doc = "0x2a4 - PWM Channel Duty Cycle Register (ch_num = 5)"]
    pub cdty5: CDTY5,
    #[doc = "0x2a8 - PWM Channel Duty Cycle Update Register (ch_num = 5)"]
    pub cdtyupd5: CDTYUPD5,
    #[doc = "0x2ac - PWM Channel Period Register (ch_num = 5)"]
    pub cprd5: CPRD5,
    #[doc = "0x2b0 - PWM Channel Period Update Register (ch_num = 5)"]
    pub cprdupd5: CPRDUPD5,
    #[doc = "0x2b4 - PWM Channel Counter Register (ch_num = 5)"]
    pub ccnt5: CCNT5,
    #[doc = "0x2b8 - PWM Channel Dead Time Register (ch_num = 5)"]
    pub dt5: DT5,
    #[doc = "0x2bc - PWM Channel Dead Time Update Register (ch_num = 5)"]
    pub dtupd5: DTUPD5,
    #[doc = "0x2c0 - PWM Channel Mode Register (ch_num = 6)"]
    pub cmr6: CMR6,
    #[doc = "0x2c4 - PWM Channel Duty Cycle Register (ch_num = 6)"]
    pub cdty6: CDTY6,
    #[doc = "0x2c8 - PWM Channel Duty Cycle Update Register (ch_num = 6)"]
    pub cdtyupd6: CDTYUPD6,
    #[doc = "0x2cc - PWM Channel Period Register (ch_num = 6)"]
    pub cprd6: CPRD6,
    #[doc = "0x2d0 - PWM Channel Period Update Register (ch_num = 6)"]
    pub cprdupd6: CPRDUPD6,
    #[doc = "0x2d4 - PWM Channel Counter Register (ch_num = 6)"]
    pub ccnt6: CCNT6,
    #[doc = "0x2d8 - PWM Channel Dead Time Register (ch_num = 6)"]
    pub dt6: DT6,
    #[doc = "0x2dc - PWM Channel Dead Time Update Register (ch_num = 6)"]
    pub dtupd6: DTUPD6,
    #[doc = "0x2e0 - PWM Channel Mode Register (ch_num = 7)"]
    pub cmr7: CMR7,
    #[doc = "0x2e4 - PWM Channel Duty Cycle Register (ch_num = 7)"]
    pub cdty7: CDTY7,
    #[doc = "0x2e8 - PWM Channel Duty Cycle Update Register (ch_num = 7)"]
    pub cdtyupd7: CDTYUPD7,
    #[doc = "0x2ec - PWM Channel Period Register (ch_num = 7)"]
    pub cprd7: CPRD7,
    #[doc = "0x2f0 - PWM Channel Period Update Register (ch_num = 7)"]
    pub cprdupd7: CPRDUPD7,
    #[doc = "0x2f4 - PWM Channel Counter Register (ch_num = 7)"]
    pub ccnt7: CCNT7,
    #[doc = "0x2f8 - PWM Channel Dead Time Register (ch_num = 7)"]
    pub dt7: DT7,
    #[doc = "0x2fc - PWM Channel Dead Time Update Register (ch_num = 7)"]
    pub dtupd7: DTUPD7,
}
#[doc = "PWM Clock Register"]
pub struct CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "PWM Enable Register"]
pub struct ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "PWM Disable Register"]
pub struct DIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "PWM Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "PWM Interrupt Enable Register 1"]
pub struct IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "PWM Interrupt Disable Register 1"]
pub struct IDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "PWM Interrupt Mask Register 1"]
pub struct IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "PWM Interrupt Status Register 1"]
pub struct ISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "PWM Sync Channels Mode Register"]
pub struct SCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "PWM Sync Channels Update Control Register"]
pub struct SCUC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "PWM Sync Channels Update Period Register"]
pub struct SCUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub struct SCUPUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "PWM Interrupt Enable Register 2"]
pub struct IER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "PWM Interrupt Disable Register 2"]
pub struct IDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "PWM Interrupt Mask Register 2"]
pub struct IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "PWM Interrupt Status Register 2"]
pub struct ISR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "PWM Output Override Value Register"]
pub struct OOV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "PWM Output Selection Register"]
pub struct OS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "PWM Output Selection Set Register"]
pub struct OSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "PWM Output Selection Clear Register"]
pub struct OSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "PWM Output Selection Set Update Register"]
pub struct OSSUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "PWM Output Selection Clear Update Register"]
pub struct OSCUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "PWM Fault Mode Register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "PWM Fault Status Register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "PWM Fault Clear Register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "PWM Fault Protection Value Register"]
pub struct FPV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Value Register"]
pub mod fpv;
#[doc = "PWM Fault Protection Enable Register 1"]
pub struct FPE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Enable Register 1"]
pub mod fpe1;
#[doc = "PWM Fault Protection Enable Register 2"]
pub struct FPE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Fault Protection Enable Register 2"]
pub mod fpe2;
#[doc = "PWM Event Line 0 Mode Register"]
pub struct ELMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Event Line 0 Mode Register"]
pub mod elmr;
#[doc = "PWM Stepper Motor Mode Register"]
pub struct SMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "PWM Write Protect Control Register"]
pub struct WPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Write Protect Control Register"]
pub mod wpcr;
#[doc = "PWM Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Write Protect Status Register"]
pub mod wpsr;
#[doc = "PWM Comparison 0 Value Register"]
pub struct CMPV0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv0;
#[doc = "PWM Comparison 0 Value Update Register"]
pub struct CMPVUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd0;
#[doc = "PWM Comparison 0 Mode Register"]
pub struct CMPM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm0;
#[doc = "PWM Comparison 0 Mode Update Register"]
pub struct CMPMUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd0;
#[doc = "PWM Comparison 1 Value Register"]
pub struct CMPV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 1 Value Register"]
pub mod cmpv1;
#[doc = "PWM Comparison 1 Value Update Register"]
pub struct CMPVUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 1 Value Update Register"]
pub mod cmpvupd1;
#[doc = "PWM Comparison 1 Mode Register"]
pub struct CMPM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 1 Mode Register"]
pub mod cmpm1;
#[doc = "PWM Comparison 1 Mode Update Register"]
pub struct CMPMUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 1 Mode Update Register"]
pub mod cmpmupd1;
#[doc = "PWM Comparison 2 Value Register"]
pub struct CMPV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 2 Value Register"]
pub mod cmpv2;
#[doc = "PWM Comparison 2 Value Update Register"]
pub struct CMPVUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 2 Value Update Register"]
pub mod cmpvupd2;
#[doc = "PWM Comparison 2 Mode Register"]
pub struct CMPM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 2 Mode Register"]
pub mod cmpm2;
#[doc = "PWM Comparison 2 Mode Update Register"]
pub struct CMPMUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 2 Mode Update Register"]
pub mod cmpmupd2;
#[doc = "PWM Comparison 3 Value Register"]
pub struct CMPV3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 3 Value Register"]
pub mod cmpv3;
#[doc = "PWM Comparison 3 Value Update Register"]
pub struct CMPVUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 3 Value Update Register"]
pub mod cmpvupd3;
#[doc = "PWM Comparison 3 Mode Register"]
pub struct CMPM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 3 Mode Register"]
pub mod cmpm3;
#[doc = "PWM Comparison 3 Mode Update Register"]
pub struct CMPMUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 3 Mode Update Register"]
pub mod cmpmupd3;
#[doc = "PWM Comparison 4 Value Register"]
pub struct CMPV4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 4 Value Register"]
pub mod cmpv4;
#[doc = "PWM Comparison 4 Value Update Register"]
pub struct CMPVUPD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 4 Value Update Register"]
pub mod cmpvupd4;
#[doc = "PWM Comparison 4 Mode Register"]
pub struct CMPM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 4 Mode Register"]
pub mod cmpm4;
#[doc = "PWM Comparison 4 Mode Update Register"]
pub struct CMPMUPD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 4 Mode Update Register"]
pub mod cmpmupd4;
#[doc = "PWM Comparison 5 Value Register"]
pub struct CMPV5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 5 Value Register"]
pub mod cmpv5;
#[doc = "PWM Comparison 5 Value Update Register"]
pub struct CMPVUPD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 5 Value Update Register"]
pub mod cmpvupd5;
#[doc = "PWM Comparison 5 Mode Register"]
pub struct CMPM5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 5 Mode Register"]
pub mod cmpm5;
#[doc = "PWM Comparison 5 Mode Update Register"]
pub struct CMPMUPD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 5 Mode Update Register"]
pub mod cmpmupd5;
#[doc = "PWM Comparison 6 Value Register"]
pub struct CMPV6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 6 Value Register"]
pub mod cmpv6;
#[doc = "PWM Comparison 6 Value Update Register"]
pub struct CMPVUPD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 6 Value Update Register"]
pub mod cmpvupd6;
#[doc = "PWM Comparison 6 Mode Register"]
pub struct CMPM6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 6 Mode Register"]
pub mod cmpm6;
#[doc = "PWM Comparison 6 Mode Update Register"]
pub struct CMPMUPD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 6 Mode Update Register"]
pub mod cmpmupd6;
#[doc = "PWM Comparison 7 Value Register"]
pub struct CMPV7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 7 Value Register"]
pub mod cmpv7;
#[doc = "PWM Comparison 7 Value Update Register"]
pub struct CMPVUPD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 7 Value Update Register"]
pub mod cmpvupd7;
#[doc = "PWM Comparison 7 Mode Register"]
pub struct CMPM7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 7 Mode Register"]
pub mod cmpm7;
#[doc = "PWM Comparison 7 Mode Update Register"]
pub struct CMPMUPD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Comparison 7 Mode Update Register"]
pub mod cmpmupd7;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub struct CMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub struct CDTY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub struct CDTYUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod cdtyupd0;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub struct CPRD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub struct CPRDUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod cprdupd0;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub struct CCNT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub struct DT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod dt0;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub struct DTUPD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod dtupd0;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub struct CMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub struct CDTY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)"]
pub struct CDTYUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)"]
pub mod cdtyupd1;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub struct CPRD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "PWM Channel Period Update Register (ch_num = 1)"]
pub struct CPRDUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 1)"]
pub mod cprdupd1;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub struct CCNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "PWM Channel Dead Time Register (ch_num = 1)"]
pub struct DT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 1)"]
pub mod dt1;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)"]
pub struct DTUPD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)"]
pub mod dtupd1;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub struct CMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub struct CDTY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)"]
pub struct CDTYUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)"]
pub mod cdtyupd2;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub struct CPRD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "PWM Channel Period Update Register (ch_num = 2)"]
pub struct CPRDUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 2)"]
pub mod cprdupd2;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub struct CCNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "PWM Channel Dead Time Register (ch_num = 2)"]
pub struct DT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 2)"]
pub mod dt2;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)"]
pub struct DTUPD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)"]
pub mod dtupd2;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub struct CMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub struct CDTY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)"]
pub struct CDTYUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)"]
pub mod cdtyupd3;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub struct CPRD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "PWM Channel Period Update Register (ch_num = 3)"]
pub struct CPRDUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 3)"]
pub mod cprdupd3;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub struct CCNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "PWM Channel Dead Time Register (ch_num = 3)"]
pub struct DT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 3)"]
pub mod dt3;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)"]
pub struct DTUPD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)"]
pub mod dtupd3;
#[doc = "PWM Channel Mode Register (ch_num = 4)"]
pub struct CMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 4)"]
pub mod cmr4;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 4)"]
pub struct CDTY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 4)"]
pub mod cdty4;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 4)"]
pub struct CDTYUPD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 4)"]
pub mod cdtyupd4;
#[doc = "PWM Channel Period Register (ch_num = 4)"]
pub struct CPRD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 4)"]
pub mod cprd4;
#[doc = "PWM Channel Period Update Register (ch_num = 4)"]
pub struct CPRDUPD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 4)"]
pub mod cprdupd4;
#[doc = "PWM Channel Counter Register (ch_num = 4)"]
pub struct CCNT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 4)"]
pub mod ccnt4;
#[doc = "PWM Channel Dead Time Register (ch_num = 4)"]
pub struct DT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 4)"]
pub mod dt4;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 4)"]
pub struct DTUPD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 4)"]
pub mod dtupd4;
#[doc = "PWM Channel Mode Register (ch_num = 5)"]
pub struct CMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 5)"]
pub mod cmr5;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 5)"]
pub struct CDTY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 5)"]
pub mod cdty5;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 5)"]
pub struct CDTYUPD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 5)"]
pub mod cdtyupd5;
#[doc = "PWM Channel Period Register (ch_num = 5)"]
pub struct CPRD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 5)"]
pub mod cprd5;
#[doc = "PWM Channel Period Update Register (ch_num = 5)"]
pub struct CPRDUPD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 5)"]
pub mod cprdupd5;
#[doc = "PWM Channel Counter Register (ch_num = 5)"]
pub struct CCNT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 5)"]
pub mod ccnt5;
#[doc = "PWM Channel Dead Time Register (ch_num = 5)"]
pub struct DT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 5)"]
pub mod dt5;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 5)"]
pub struct DTUPD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 5)"]
pub mod dtupd5;
#[doc = "PWM Channel Mode Register (ch_num = 6)"]
pub struct CMR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 6)"]
pub mod cmr6;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 6)"]
pub struct CDTY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 6)"]
pub mod cdty6;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 6)"]
pub struct CDTYUPD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 6)"]
pub mod cdtyupd6;
#[doc = "PWM Channel Period Register (ch_num = 6)"]
pub struct CPRD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 6)"]
pub mod cprd6;
#[doc = "PWM Channel Period Update Register (ch_num = 6)"]
pub struct CPRDUPD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 6)"]
pub mod cprdupd6;
#[doc = "PWM Channel Counter Register (ch_num = 6)"]
pub struct CCNT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 6)"]
pub mod ccnt6;
#[doc = "PWM Channel Dead Time Register (ch_num = 6)"]
pub struct DT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 6)"]
pub mod dt6;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 6)"]
pub struct DTUPD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 6)"]
pub mod dtupd6;
#[doc = "PWM Channel Mode Register (ch_num = 7)"]
pub struct CMR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Mode Register (ch_num = 7)"]
pub mod cmr7;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 7)"]
pub struct CDTY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 7)"]
pub mod cdty7;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 7)"]
pub struct CDTYUPD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 7)"]
pub mod cdtyupd7;
#[doc = "PWM Channel Period Register (ch_num = 7)"]
pub struct CPRD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Register (ch_num = 7)"]
pub mod cprd7;
#[doc = "PWM Channel Period Update Register (ch_num = 7)"]
pub struct CPRDUPD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Period Update Register (ch_num = 7)"]
pub mod cprdupd7;
#[doc = "PWM Channel Counter Register (ch_num = 7)"]
pub struct CCNT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Counter Register (ch_num = 7)"]
pub mod ccnt7;
#[doc = "PWM Channel Dead Time Register (ch_num = 7)"]
pub struct DT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Register (ch_num = 7)"]
pub mod dt7;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 7)"]
pub struct DTUPD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 7)"]
pub mod dtupd7;
#[doc = "Transmit Pointer Register"]
pub struct TPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "Transmit Counter Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "Transmit Next Pointer Register"]
pub struct TNPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "Transmit Next Counter Register"]
pub struct TNCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "Transfer Control Register"]
pub struct PTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "Transfer Status Register"]
pub struct PTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Status Register"]
pub mod ptsr;
