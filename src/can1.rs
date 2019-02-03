#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x08 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x0c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x10 - Status Register"]
    pub sr: SR,
    #[doc = "0x14 - Baudrate Register"]
    pub br: BR,
    #[doc = "0x18 - Timer Register"]
    pub tim: TIM,
    #[doc = "0x1c - Timestamp Register"]
    pub timestp: TIMESTP,
    #[doc = "0x20 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x24 - Transfer Command Register"]
    pub tcr: TCR,
    #[doc = "0x28 - Abort Command Register"]
    pub acr: ACR,
    _reserved11: [u8; 184usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved13: [u8; 276usize],
    #[doc = "0x200 - Mailbox Mode Register (MB = 0)"]
    pub mmr0: MMR0,
    #[doc = "0x204 - Mailbox Acceptance Mask Register (MB = 0)"]
    pub mam0: MAM0,
    #[doc = "0x208 - Mailbox ID Register (MB = 0)"]
    pub mid0: MID0,
    #[doc = "0x20c - Mailbox Family ID Register (MB = 0)"]
    pub mfid0: MFID0,
    #[doc = "0x210 - Mailbox Status Register (MB = 0)"]
    pub msr0: MSR0,
    #[doc = "0x214 - Mailbox Data Low Register (MB = 0)"]
    pub mdl0: MDL0,
    #[doc = "0x218 - Mailbox Data High Register (MB = 0)"]
    pub mdh0: MDH0,
    #[doc = "0x21c - Mailbox Control Register (MB = 0)"]
    pub mcr0: MCR0,
    #[doc = "0x220 - Mailbox Mode Register (MB = 1)"]
    pub mmr1: MMR1,
    #[doc = "0x224 - Mailbox Acceptance Mask Register (MB = 1)"]
    pub mam1: MAM1,
    #[doc = "0x228 - Mailbox ID Register (MB = 1)"]
    pub mid1: MID1,
    #[doc = "0x22c - Mailbox Family ID Register (MB = 1)"]
    pub mfid1: MFID1,
    #[doc = "0x230 - Mailbox Status Register (MB = 1)"]
    pub msr1: MSR1,
    #[doc = "0x234 - Mailbox Data Low Register (MB = 1)"]
    pub mdl1: MDL1,
    #[doc = "0x238 - Mailbox Data High Register (MB = 1)"]
    pub mdh1: MDH1,
    #[doc = "0x23c - Mailbox Control Register (MB = 1)"]
    pub mcr1: MCR1,
    #[doc = "0x240 - Mailbox Mode Register (MB = 2)"]
    pub mmr2: MMR2,
    #[doc = "0x244 - Mailbox Acceptance Mask Register (MB = 2)"]
    pub mam2: MAM2,
    #[doc = "0x248 - Mailbox ID Register (MB = 2)"]
    pub mid2: MID2,
    #[doc = "0x24c - Mailbox Family ID Register (MB = 2)"]
    pub mfid2: MFID2,
    #[doc = "0x250 - Mailbox Status Register (MB = 2)"]
    pub msr2: MSR2,
    #[doc = "0x254 - Mailbox Data Low Register (MB = 2)"]
    pub mdl2: MDL2,
    #[doc = "0x258 - Mailbox Data High Register (MB = 2)"]
    pub mdh2: MDH2,
    #[doc = "0x25c - Mailbox Control Register (MB = 2)"]
    pub mcr2: MCR2,
    #[doc = "0x260 - Mailbox Mode Register (MB = 3)"]
    pub mmr3: MMR3,
    #[doc = "0x264 - Mailbox Acceptance Mask Register (MB = 3)"]
    pub mam3: MAM3,
    #[doc = "0x268 - Mailbox ID Register (MB = 3)"]
    pub mid3: MID3,
    #[doc = "0x26c - Mailbox Family ID Register (MB = 3)"]
    pub mfid3: MFID3,
    #[doc = "0x270 - Mailbox Status Register (MB = 3)"]
    pub msr3: MSR3,
    #[doc = "0x274 - Mailbox Data Low Register (MB = 3)"]
    pub mdl3: MDL3,
    #[doc = "0x278 - Mailbox Data High Register (MB = 3)"]
    pub mdh3: MDH3,
    #[doc = "0x27c - Mailbox Control Register (MB = 3)"]
    pub mcr3: MCR3,
    #[doc = "0x280 - Mailbox Mode Register (MB = 4)"]
    pub mmr4: MMR4,
    #[doc = "0x284 - Mailbox Acceptance Mask Register (MB = 4)"]
    pub mam4: MAM4,
    #[doc = "0x288 - Mailbox ID Register (MB = 4)"]
    pub mid4: MID4,
    #[doc = "0x28c - Mailbox Family ID Register (MB = 4)"]
    pub mfid4: MFID4,
    #[doc = "0x290 - Mailbox Status Register (MB = 4)"]
    pub msr4: MSR4,
    #[doc = "0x294 - Mailbox Data Low Register (MB = 4)"]
    pub mdl4: MDL4,
    #[doc = "0x298 - Mailbox Data High Register (MB = 4)"]
    pub mdh4: MDH4,
    #[doc = "0x29c - Mailbox Control Register (MB = 4)"]
    pub mcr4: MCR4,
    #[doc = "0x2a0 - Mailbox Mode Register (MB = 5)"]
    pub mmr5: MMR5,
    #[doc = "0x2a4 - Mailbox Acceptance Mask Register (MB = 5)"]
    pub mam5: MAM5,
    #[doc = "0x2a8 - Mailbox ID Register (MB = 5)"]
    pub mid5: MID5,
    #[doc = "0x2ac - Mailbox Family ID Register (MB = 5)"]
    pub mfid5: MFID5,
    #[doc = "0x2b0 - Mailbox Status Register (MB = 5)"]
    pub msr5: MSR5,
    #[doc = "0x2b4 - Mailbox Data Low Register (MB = 5)"]
    pub mdl5: MDL5,
    #[doc = "0x2b8 - Mailbox Data High Register (MB = 5)"]
    pub mdh5: MDH5,
    #[doc = "0x2bc - Mailbox Control Register (MB = 5)"]
    pub mcr5: MCR5,
    #[doc = "0x2c0 - Mailbox Mode Register (MB = 6)"]
    pub mmr6: MMR6,
    #[doc = "0x2c4 - Mailbox Acceptance Mask Register (MB = 6)"]
    pub mam6: MAM6,
    #[doc = "0x2c8 - Mailbox ID Register (MB = 6)"]
    pub mid6: MID6,
    #[doc = "0x2cc - Mailbox Family ID Register (MB = 6)"]
    pub mfid6: MFID6,
    #[doc = "0x2d0 - Mailbox Status Register (MB = 6)"]
    pub msr6: MSR6,
    #[doc = "0x2d4 - Mailbox Data Low Register (MB = 6)"]
    pub mdl6: MDL6,
    #[doc = "0x2d8 - Mailbox Data High Register (MB = 6)"]
    pub mdh6: MDH6,
    #[doc = "0x2dc - Mailbox Control Register (MB = 6)"]
    pub mcr6: MCR6,
    #[doc = "0x2e0 - Mailbox Mode Register (MB = 7)"]
    pub mmr7: MMR7,
    #[doc = "0x2e4 - Mailbox Acceptance Mask Register (MB = 7)"]
    pub mam7: MAM7,
    #[doc = "0x2e8 - Mailbox ID Register (MB = 7)"]
    pub mid7: MID7,
    #[doc = "0x2ec - Mailbox Family ID Register (MB = 7)"]
    pub mfid7: MFID7,
    #[doc = "0x2f0 - Mailbox Status Register (MB = 7)"]
    pub msr7: MSR7,
    #[doc = "0x2f4 - Mailbox Data Low Register (MB = 7)"]
    pub mdl7: MDL7,
    #[doc = "0x2f8 - Mailbox Data High Register (MB = 7)"]
    pub mdh7: MDH7,
    #[doc = "0x2fc - Mailbox Control Register (MB = 7)"]
    pub mcr7: MCR7,
}
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Baudrate Register"]
pub struct BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "Timer Register"]
pub struct TIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Register"]
pub mod tim;
#[doc = "Timestamp Register"]
pub struct TIMESTP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "Error Counter Register"]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Transfer Command Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "Abort Command Register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub struct MMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub struct MAM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "Mailbox ID Register (MB = 0)"]
pub struct MID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub struct MFID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "Mailbox Status Register (MB = 0)"]
pub struct MSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub struct MDL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub struct MDH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "Mailbox Control Register (MB = 0)"]
pub struct MCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "Mailbox Mode Register (MB = 1)"]
pub struct MMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub struct MAM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "Mailbox ID Register (MB = 1)"]
pub struct MID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub struct MFID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "Mailbox Status Register (MB = 1)"]
pub struct MSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub struct MDL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "Mailbox Data High Register (MB = 1)"]
pub struct MDH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "Mailbox Control Register (MB = 1)"]
pub struct MCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "Mailbox Mode Register (MB = 2)"]
pub struct MMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub struct MAM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "Mailbox ID Register (MB = 2)"]
pub struct MID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub struct MFID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "Mailbox Status Register (MB = 2)"]
pub struct MSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub struct MDL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "Mailbox Data High Register (MB = 2)"]
pub struct MDH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "Mailbox Control Register (MB = 2)"]
pub struct MCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "Mailbox Mode Register (MB = 3)"]
pub struct MMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub struct MAM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "Mailbox ID Register (MB = 3)"]
pub struct MID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub struct MFID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "Mailbox Status Register (MB = 3)"]
pub struct MSR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub struct MDL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "Mailbox Data High Register (MB = 3)"]
pub struct MDH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "Mailbox Control Register (MB = 3)"]
pub struct MCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "Mailbox Mode Register (MB = 4)"]
pub struct MMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub struct MAM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "Mailbox ID Register (MB = 4)"]
pub struct MID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub struct MFID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "Mailbox Status Register (MB = 4)"]
pub struct MSR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub struct MDL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "Mailbox Data High Register (MB = 4)"]
pub struct MDH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "Mailbox Control Register (MB = 4)"]
pub struct MCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "Mailbox Mode Register (MB = 5)"]
pub struct MMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub struct MAM5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "Mailbox ID Register (MB = 5)"]
pub struct MID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub struct MFID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "Mailbox Status Register (MB = 5)"]
pub struct MSR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub struct MDL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "Mailbox Data High Register (MB = 5)"]
pub struct MDH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "Mailbox Control Register (MB = 5)"]
pub struct MCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "Mailbox Mode Register (MB = 6)"]
pub struct MMR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub struct MAM6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "Mailbox ID Register (MB = 6)"]
pub struct MID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub struct MFID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "Mailbox Status Register (MB = 6)"]
pub struct MSR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub struct MDL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "Mailbox Data High Register (MB = 6)"]
pub struct MDH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "Mailbox Control Register (MB = 6)"]
pub struct MCR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "Mailbox Mode Register (MB = 7)"]
pub struct MMR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub struct MAM7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "Mailbox ID Register (MB = 7)"]
pub struct MID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub struct MFID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "Mailbox Status Register (MB = 7)"]
pub struct MSR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub struct MDL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "Mailbox Data High Register (MB = 7)"]
pub struct MDH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "Mailbox Control Register (MB = 7)"]
pub struct MCR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
