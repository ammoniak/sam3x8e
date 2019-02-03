#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC NFC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - SMC NFC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SMC NFC Status Register"]
    pub sr: SR,
    #[doc = "0x0c - SMC NFC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x10 - SMC NFC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x14 - SMC NFC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x18 - SMC NFC Address Cycle Zero Register"]
    pub addr: ADDR,
    #[doc = "0x1c - SMC Bank Address Register"]
    pub bank: BANK,
    #[doc = "0x20 - SMC ECC Control Register"]
    pub ecc_ctrl: ECC_CTRL,
    #[doc = "0x24 - SMC ECC Mode Register"]
    pub ecc_md: ECC_MD,
    #[doc = "0x28 - SMC ECC Status 1 Register"]
    pub ecc_sr1: ECC_SR1,
    #[doc = "SMC ECC Parity 0 Register"]
    pub ecc_pr0: ECC_PR0_UNION,
    #[doc = "SMC ECC parity 1 Register"]
    pub ecc_pr1: ECC_PR1_UNION,
    #[doc = "0x34 - SMC ECC status 2 Register"]
    pub ecc_sr2: ECC_SR2,
    #[doc = "SMC ECC parity 2 Register"]
    pub ecc_pr2: ECC_PR2_UNION,
    #[doc = "SMC ECC parity 3 Register"]
    pub ecc_pr3: ECC_PR3_UNION,
    #[doc = "SMC ECC parity 4 Register"]
    pub ecc_pr4: ECC_PR4_UNION,
    #[doc = "SMC ECC parity 5 Register"]
    pub ecc_pr5: ECC_PR5_UNION,
    #[doc = "SMC ECC parity 6 Register"]
    pub ecc_pr6: ECC_PR6_UNION,
    #[doc = "SMC ECC parity 7 Register"]
    pub ecc_pr7: ECC_PR7_UNION,
    #[doc = "0x50 - SMC ECC parity 8 Register"]
    pub ecc_pr8: ECC_PR8,
    #[doc = "0x54 - SMC ECC parity 9 Register"]
    pub ecc_pr9: ECC_PR9,
    #[doc = "0x58 - SMC ECC parity 10 Register"]
    pub ecc_pr10: ECC_PR10,
    #[doc = "0x5c - SMC ECC parity 11 Register"]
    pub ecc_pr11: ECC_PR11,
    #[doc = "0x60 - SMC ECC parity 12 Register"]
    pub ecc_pr12: ECC_PR12,
    #[doc = "0x64 - SMC ECC parity 13 Register"]
    pub ecc_pr13: ECC_PR13,
    #[doc = "0x68 - SMC ECC parity 14 Register"]
    pub ecc_pr14: ECC_PR14,
    #[doc = "0x6c - SMC ECC parity 15 Register"]
    pub ecc_pr15: ECC_PR15,
    #[doc = "0x70 - SMC Setup Register (CS_number = 0)"]
    pub setup0: SETUP0,
    #[doc = "0x74 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: PULSE0,
    #[doc = "0x78 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: CYCLE0,
    #[doc = "0x7c - SMC Timings Register (CS_number = 0)"]
    pub timings0: TIMINGS0,
    #[doc = "0x80 - SMC Mode Register (CS_number = 0)"]
    pub mode0: MODE0,
    #[doc = "0x84 - SMC Setup Register (CS_number = 1)"]
    pub setup1: SETUP1,
    #[doc = "0x88 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: PULSE1,
    #[doc = "0x8c - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: CYCLE1,
    #[doc = "0x90 - SMC Timings Register (CS_number = 1)"]
    pub timings1: TIMINGS1,
    #[doc = "0x94 - SMC Mode Register (CS_number = 1)"]
    pub mode1: MODE1,
    #[doc = "0x98 - SMC Setup Register (CS_number = 2)"]
    pub setup2: SETUP2,
    #[doc = "0x9c - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: PULSE2,
    #[doc = "0xa0 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: CYCLE2,
    #[doc = "0xa4 - SMC Timings Register (CS_number = 2)"]
    pub timings2: TIMINGS2,
    #[doc = "0xa8 - SMC Mode Register (CS_number = 2)"]
    pub mode2: MODE2,
    #[doc = "0xac - SMC Setup Register (CS_number = 3)"]
    pub setup3: SETUP3,
    #[doc = "0xb0 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: PULSE3,
    #[doc = "0xb4 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: CYCLE3,
    #[doc = "0xb8 - SMC Timings Register (CS_number = 3)"]
    pub timings3: TIMINGS3,
    #[doc = "0xbc - SMC Mode Register (CS_number = 3)"]
    pub mode3: MODE3,
    #[doc = "0xc0 - SMC Setup Register (CS_number = 4)"]
    pub setup4: SETUP4,
    #[doc = "0xc4 - SMC Pulse Register (CS_number = 4)"]
    pub pulse4: PULSE4,
    #[doc = "0xc8 - SMC Cycle Register (CS_number = 4)"]
    pub cycle4: CYCLE4,
    #[doc = "0xcc - SMC Timings Register (CS_number = 4)"]
    pub timings4: TIMINGS4,
    #[doc = "0xd0 - SMC Mode Register (CS_number = 4)"]
    pub mode4: MODE4,
    #[doc = "0xd4 - SMC Setup Register (CS_number = 5)"]
    pub setup5: SETUP5,
    #[doc = "0xd8 - SMC Pulse Register (CS_number = 5)"]
    pub pulse5: PULSE5,
    #[doc = "0xdc - SMC Cycle Register (CS_number = 5)"]
    pub cycle5: CYCLE5,
    #[doc = "0xe0 - SMC Timings Register (CS_number = 5)"]
    pub timings5: TIMINGS5,
    #[doc = "0xe4 - SMC Mode Register (CS_number = 5)"]
    pub mode5: MODE5,
    #[doc = "0xe8 - SMC Setup Register (CS_number = 6)"]
    pub setup6: SETUP6,
    #[doc = "0xec - SMC Pulse Register (CS_number = 6)"]
    pub pulse6: PULSE6,
    #[doc = "0xf0 - SMC Cycle Register (CS_number = 6)"]
    pub cycle6: CYCLE6,
    #[doc = "0xf4 - SMC Timings Register (CS_number = 6)"]
    pub timings6: TIMINGS6,
    #[doc = "0xf8 - SMC Mode Register (CS_number = 6)"]
    pub mode6: MODE6,
    #[doc = "0xfc - SMC Setup Register (CS_number = 7)"]
    pub setup7: SETUP7,
    #[doc = "0x100 - SMC Pulse Register (CS_number = 7)"]
    pub pulse7: PULSE7,
    #[doc = "0x104 - SMC Cycle Register (CS_number = 7)"]
    pub cycle7: CYCLE7,
    #[doc = "0x108 - SMC Timings Register (CS_number = 7)"]
    pub timings7: TIMINGS7,
    #[doc = "0x10c - SMC Mode Register (CS_number = 7)"]
    pub mode7: MODE7,
    #[doc = "0x110 - SMC OCMS Register"]
    pub ocms: OCMS,
    #[doc = "0x114 - SMC OCMS KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x118 - SMC OCMS KEY2 Register"]
    pub key2: KEY2,
    _reserved71: [u8; 200usize],
    #[doc = "0x1e4 - Write Protection Control Register"]
    pub wpcr: WPCR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SMC ECC Parity 0 Register"]
#[repr(C)]
pub union ECC_PR0_UNION {
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    pub ecc_pr0_w8bit: ECC_PR0_W8BIT,
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    pub ecc_pr0_w9bit: ECC_PR0_W9BIT,
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    pub ecc_pr0: ECC_PR0,
}
#[doc = "SMC ECC parity 1 Register"]
#[repr(C)]
pub union ECC_PR1_UNION {
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    pub ecc_pr1_w8bit: ECC_PR1_W8BIT,
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    pub ecc_pr1_w9bit: ECC_PR1_W9BIT,
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    pub ecc_pr1: ECC_PR1,
}
#[doc = "SMC ECC parity 2 Register"]
#[repr(C)]
pub union ECC_PR2_UNION {
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    pub ecc_pr2_w8bit: ECC_PR2_W8BIT,
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    pub ecc_pr2: ECC_PR2,
}
#[doc = "SMC ECC parity 3 Register"]
#[repr(C)]
pub union ECC_PR3_UNION {
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    pub ecc_pr3_w8bit: ECC_PR3_W8BIT,
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    pub ecc_pr3: ECC_PR3,
}
#[doc = "SMC ECC parity 4 Register"]
#[repr(C)]
pub union ECC_PR4_UNION {
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    pub ecc_pr4_w8bit: ECC_PR4_W8BIT,
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    pub ecc_pr4: ECC_PR4,
}
#[doc = "SMC ECC parity 5 Register"]
#[repr(C)]
pub union ECC_PR5_UNION {
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    pub ecc_pr5_w8bit: ECC_PR5_W8BIT,
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    pub ecc_pr5: ECC_PR5,
}
#[doc = "SMC ECC parity 6 Register"]
#[repr(C)]
pub union ECC_PR6_UNION {
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    pub ecc_pr6_w8bit: ECC_PR6_W8BIT,
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    pub ecc_pr6: ECC_PR6,
}
#[doc = "SMC ECC parity 7 Register"]
#[repr(C)]
pub union ECC_PR7_UNION {
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    pub ecc_pr7_w8bit: ECC_PR7_W8BIT,
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    pub ecc_pr7: ECC_PR7,
}
#[doc = "SMC NFC Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Configuration Register"]
pub mod cfg;
#[doc = "SMC NFC Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Control Register"]
pub mod ctrl;
#[doc = "SMC NFC Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Status Register"]
pub mod sr;
#[doc = "SMC NFC Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Interrupt Enable Register"]
pub mod ier;
#[doc = "SMC NFC Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Interrupt Disable Register"]
pub mod idr;
#[doc = "SMC NFC Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Interrupt Mask Register"]
pub mod imr;
#[doc = "SMC NFC Address Cycle Zero Register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NFC Address Cycle Zero Register"]
pub mod addr;
#[doc = "SMC Bank Address Register"]
pub struct BANK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Bank Address Register"]
pub mod bank;
#[doc = "SMC ECC Control Register"]
pub struct ECC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Control Register"]
pub mod ecc_ctrl;
#[doc = "SMC ECC Mode Register"]
pub struct ECC_MD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Mode Register"]
pub mod ecc_md;
#[doc = "SMC ECC Status 1 Register"]
pub struct ECC_SR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Status 1 Register"]
pub mod ecc_sr1;
#[doc = "SMC ECC Parity 0 Register"]
pub struct ECC_PR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0;
#[doc = "SMC ECC Parity 0 Register"]
pub struct ECC_PR0_W9BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0_w9bit;
#[doc = "SMC ECC Parity 0 Register"]
pub struct ECC_PR0_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0_w8bit;
#[doc = "SMC ECC parity 1 Register"]
pub struct ECC_PR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1;
#[doc = "SMC ECC parity 1 Register"]
pub struct ECC_PR1_W9BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1_w9bit;
#[doc = "SMC ECC parity 1 Register"]
pub struct ECC_PR1_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1_w8bit;
#[doc = "SMC ECC status 2 Register"]
pub struct ECC_SR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC status 2 Register"]
pub mod ecc_sr2;
#[doc = "SMC ECC parity 2 Register"]
pub struct ECC_PR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2;
#[doc = "SMC ECC parity 2 Register"]
pub struct ECC_PR2_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2_w8bit;
#[doc = "SMC ECC parity 3 Register"]
pub struct ECC_PR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3;
#[doc = "SMC ECC parity 3 Register"]
pub struct ECC_PR3_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3_w8bit;
#[doc = "SMC ECC parity 4 Register"]
pub struct ECC_PR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4;
#[doc = "SMC ECC parity 4 Register"]
pub struct ECC_PR4_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4_w8bit;
#[doc = "SMC ECC parity 5 Register"]
pub struct ECC_PR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5;
#[doc = "SMC ECC parity 5 Register"]
pub struct ECC_PR5_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5_w8bit;
#[doc = "SMC ECC parity 6 Register"]
pub struct ECC_PR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6;
#[doc = "SMC ECC parity 6 Register"]
pub struct ECC_PR6_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6_w8bit;
#[doc = "SMC ECC parity 7 Register"]
pub struct ECC_PR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7;
#[doc = "SMC ECC parity 7 Register"]
pub struct ECC_PR7_W8BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7_w8bit;
#[doc = "SMC ECC parity 8 Register"]
pub struct ECC_PR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 8 Register"]
pub mod ecc_pr8;
#[doc = "SMC ECC parity 9 Register"]
pub struct ECC_PR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 9 Register"]
pub mod ecc_pr9;
#[doc = "SMC ECC parity 10 Register"]
pub struct ECC_PR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 10 Register"]
pub mod ecc_pr10;
#[doc = "SMC ECC parity 11 Register"]
pub struct ECC_PR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 11 Register"]
pub mod ecc_pr11;
#[doc = "SMC ECC parity 12 Register"]
pub struct ECC_PR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 12 Register"]
pub mod ecc_pr12;
#[doc = "SMC ECC parity 13 Register"]
pub struct ECC_PR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 13 Register"]
pub mod ecc_pr13;
#[doc = "SMC ECC parity 14 Register"]
pub struct ECC_PR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 14 Register"]
pub mod ecc_pr14;
#[doc = "SMC ECC parity 15 Register"]
pub struct ECC_PR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC ECC parity 15 Register"]
pub mod ecc_pr15;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub struct SETUP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub struct PULSE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub struct CYCLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "SMC Timings Register (CS_number = 0)"]
pub struct TIMINGS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 0)"]
pub mod timings0;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub struct MODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub struct SETUP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub struct PULSE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub struct CYCLE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "SMC Timings Register (CS_number = 1)"]
pub struct TIMINGS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 1)"]
pub mod timings1;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub struct MODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub struct SETUP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub struct PULSE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub struct CYCLE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "SMC Timings Register (CS_number = 2)"]
pub struct TIMINGS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 2)"]
pub mod timings2;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub struct MODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub struct SETUP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub struct PULSE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub struct CYCLE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "SMC Timings Register (CS_number = 3)"]
pub struct TIMINGS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 3)"]
pub mod timings3;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub struct MODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "SMC Setup Register (CS_number = 4)"]
pub struct SETUP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 4)"]
pub mod setup4;
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub struct PULSE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub mod pulse4;
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub struct CYCLE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub mod cycle4;
#[doc = "SMC Timings Register (CS_number = 4)"]
pub struct TIMINGS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 4)"]
pub mod timings4;
#[doc = "SMC Mode Register (CS_number = 4)"]
pub struct MODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 4)"]
pub mod mode4;
#[doc = "SMC Setup Register (CS_number = 5)"]
pub struct SETUP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 5)"]
pub mod setup5;
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub struct PULSE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub mod pulse5;
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub struct CYCLE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub mod cycle5;
#[doc = "SMC Timings Register (CS_number = 5)"]
pub struct TIMINGS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 5)"]
pub mod timings5;
#[doc = "SMC Mode Register (CS_number = 5)"]
pub struct MODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 5)"]
pub mod mode5;
#[doc = "SMC Setup Register (CS_number = 6)"]
pub struct SETUP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 6)"]
pub mod setup6;
#[doc = "SMC Pulse Register (CS_number = 6)"]
pub struct PULSE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 6)"]
pub mod pulse6;
#[doc = "SMC Cycle Register (CS_number = 6)"]
pub struct CYCLE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 6)"]
pub mod cycle6;
#[doc = "SMC Timings Register (CS_number = 6)"]
pub struct TIMINGS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 6)"]
pub mod timings6;
#[doc = "SMC Mode Register (CS_number = 6)"]
pub struct MODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 6)"]
pub mod mode6;
#[doc = "SMC Setup Register (CS_number = 7)"]
pub struct SETUP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Setup Register (CS_number = 7)"]
pub mod setup7;
#[doc = "SMC Pulse Register (CS_number = 7)"]
pub struct PULSE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Pulse Register (CS_number = 7)"]
pub mod pulse7;
#[doc = "SMC Cycle Register (CS_number = 7)"]
pub struct CYCLE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Cycle Register (CS_number = 7)"]
pub mod cycle7;
#[doc = "SMC Timings Register (CS_number = 7)"]
pub struct TIMINGS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Timings Register (CS_number = 7)"]
pub mod timings7;
#[doc = "SMC Mode Register (CS_number = 7)"]
pub struct MODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Mode Register (CS_number = 7)"]
pub mod mode7;
#[doc = "SMC OCMS Register"]
pub struct OCMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC OCMS Register"]
pub mod ocms;
#[doc = "SMC OCMS KEY1 Register"]
pub struct KEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "SMC OCMS KEY2 Register"]
pub struct KEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "Write Protection Control Register"]
pub struct WPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Control Register"]
pub mod wpcr;
#[doc = "Write Protection Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
