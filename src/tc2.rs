#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr0: CCR0,
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    pub cmr0: CMR0,
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr0: SMMR0,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv0: CV0,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra0: RA0,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb0: RB0,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc0: RC0,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr0: SR0,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier0: IER0,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr0: IDR0,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr0: IMR0,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    pub ccr1: CCR1,
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    pub cmr1: CMR1,
    #[doc = "0x48 - Stepper Motor Mode Register (channel = 1)"]
    pub smmr1: SMMR1,
    _reserved2: [u8; 4usize],
    #[doc = "0x50 - Counter Value (channel = 1)"]
    pub cv1: CV1,
    #[doc = "0x54 - Register A (channel = 1)"]
    pub ra1: RA1,
    #[doc = "0x58 - Register B (channel = 1)"]
    pub rb1: RB1,
    #[doc = "0x5c - Register C (channel = 1)"]
    pub rc1: RC1,
    #[doc = "0x60 - Status Register (channel = 1)"]
    pub sr1: SR1,
    #[doc = "0x64 - Interrupt Enable Register (channel = 1)"]
    pub ier1: IER1,
    #[doc = "0x68 - Interrupt Disable Register (channel = 1)"]
    pub idr1: IDR1,
    #[doc = "0x6c - Interrupt Mask Register (channel = 1)"]
    pub imr1: IMR1,
    _reserved3: [u8; 16usize],
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    pub ccr2: CCR2,
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    pub cmr2: CMR2,
    #[doc = "0x88 - Stepper Motor Mode Register (channel = 2)"]
    pub smmr2: SMMR2,
    _reserved4: [u8; 4usize],
    #[doc = "0x90 - Counter Value (channel = 2)"]
    pub cv2: CV2,
    #[doc = "0x94 - Register A (channel = 2)"]
    pub ra2: RA2,
    #[doc = "0x98 - Register B (channel = 2)"]
    pub rb2: RB2,
    #[doc = "0x9c - Register C (channel = 2)"]
    pub rc2: RC2,
    #[doc = "0xa0 - Status Register (channel = 2)"]
    pub sr2: SR2,
    #[doc = "0xa4 - Interrupt Enable Register (channel = 2)"]
    pub ier2: IER2,
    #[doc = "0xa8 - Interrupt Disable Register (channel = 2)"]
    pub idr2: IDR2,
    #[doc = "0xac - Interrupt Mask Register (channel = 2)"]
    pub imr2: IMR2,
    _reserved5: [u8; 16usize],
    #[doc = "0xc0 - Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - Block Mode Register"]
    pub bmr: BMR,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub qier: QIER,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub qidr: QIDR,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub qimr: QIMR,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub qisr: QISR,
    #[doc = "0xd8 - Fault Mode Register"]
    pub fmr: FMR,
    _reserved6: [u8; 8usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
}
#[doc = "Channel Control Register (channel = 0)"]
pub struct CCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "Channel Mode Register (channel = 0)"]
pub struct CMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "Channel Mode Register (channel = 0)"]
pub struct CMR0_WAVE_EQ_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub struct SMMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr0;
#[doc = "Counter Value (channel = 0)"]
pub struct CV0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "Register A (channel = 0)"]
pub struct RA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "Register B (channel = 0)"]
pub struct RB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "Register C (channel = 0)"]
pub struct RC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "Status Register (channel = 0)"]
pub struct SR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub struct IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub struct IDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub struct IMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "Channel Control Register (channel = 1)"]
pub struct CCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "Channel Mode Register (channel = 1)"]
pub struct CMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "Channel Mode Register (channel = 1)"]
pub struct CMR1_WAVE_EQ_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 1)"]
pub struct SMMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stepper Motor Mode Register (channel = 1)"]
pub mod smmr1;
#[doc = "Counter Value (channel = 1)"]
pub struct CV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "Register A (channel = 1)"]
pub struct RA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "Register B (channel = 1)"]
pub struct RB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "Register C (channel = 1)"]
pub struct RC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "Status Register (channel = 1)"]
pub struct SR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "Interrupt Enable Register (channel = 1)"]
pub struct IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "Interrupt Disable Register (channel = 1)"]
pub struct IDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "Interrupt Mask Register (channel = 1)"]
pub struct IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "Channel Control Register (channel = 2)"]
pub struct CCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "Channel Mode Register (channel = 2)"]
pub struct CMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "Channel Mode Register (channel = 2)"]
pub struct CMR2_WAVE_EQ_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 2)"]
pub struct SMMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stepper Motor Mode Register (channel = 2)"]
pub mod smmr2;
#[doc = "Counter Value (channel = 2)"]
pub struct CV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "Register A (channel = 2)"]
pub struct RA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "Register B (channel = 2)"]
pub struct RB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "Register C (channel = 2)"]
pub struct RC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "Status Register (channel = 2)"]
pub struct SR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "Interrupt Enable Register (channel = 2)"]
pub struct IER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "Interrupt Disable Register (channel = 2)"]
pub struct IDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "Interrupt Mask Register (channel = 2)"]
pub struct IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "Block Control Register"]
pub struct BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "Block Mode Register"]
pub struct BMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QDEC Interrupt Enable Register"]
pub struct QIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QDEC Interrupt Disable Register"]
pub struct QIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QDEC Interrupt Mask Register"]
pub struct QIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QDEC Interrupt Status Register"]
pub struct QISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "Fault Mode Register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
