#[doc = r"Register block"]
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
    _reserved_11_ecc_pr: [u8; 4usize],
    _reserved_12_ecc_pr: [u8; 4usize],
    #[doc = "0x34 - SMC ECC status 2 Register"]
    pub ecc_sr2: ECC_SR2,
    _reserved_14_ecc_pr: [u8; 4usize],
    _reserved_15_ecc_pr: [u8; 4usize],
    _reserved_16_ecc_pr: [u8; 4usize],
    _reserved_17_ecc_pr: [u8; 4usize],
    _reserved_18_ecc_pr: [u8; 4usize],
    _reserved_19_ecc_pr: [u8; 4usize],
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
impl RegisterBlock {
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0_w8bit(&self) -> &ECC_PR0_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const ECC_PR0_W8BIT) }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0_w8bit_mut(&self) -> &mut ECC_PR0_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut ECC_PR0_W8BIT) }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0_w9bit(&self) -> &ECC_PR0_W9BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const ECC_PR0_W9BIT) }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0_w9bit_mut(&self) -> &mut ECC_PR0_W9BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut ECC_PR0_W9BIT) }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0(&self) -> &ECC_PR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const ECC_PR0) }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0_mut(&self) -> &mut ECC_PR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut ECC_PR0) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1_w8bit(&self) -> &ECC_PR1_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const ECC_PR1_W8BIT) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1_w8bit_mut(&self) -> &mut ECC_PR1_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut ECC_PR1_W8BIT) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1_w9bit(&self) -> &ECC_PR1_W9BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const ECC_PR1_W9BIT) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1_w9bit_mut(&self) -> &mut ECC_PR1_W9BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut ECC_PR1_W9BIT) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1(&self) -> &ECC_PR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const ECC_PR1) }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1_mut(&self) -> &mut ECC_PR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut ECC_PR1) }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn ecc_pr2_w8bit(&self) -> &ECC_PR2_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const ECC_PR2_W8BIT) }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn ecc_pr2_w8bit_mut(&self) -> &mut ECC_PR2_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut ECC_PR2_W8BIT) }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn ecc_pr2(&self) -> &ECC_PR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const ECC_PR2) }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn ecc_pr2_mut(&self) -> &mut ECC_PR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut ECC_PR2) }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn ecc_pr3_w8bit(&self) -> &ECC_PR3_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const ECC_PR3_W8BIT) }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn ecc_pr3_w8bit_mut(&self) -> &mut ECC_PR3_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut ECC_PR3_W8BIT) }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn ecc_pr3(&self) -> &ECC_PR3 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const ECC_PR3) }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn ecc_pr3_mut(&self) -> &mut ECC_PR3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut ECC_PR3) }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn ecc_pr4_w8bit(&self) -> &ECC_PR4_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const ECC_PR4_W8BIT) }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn ecc_pr4_w8bit_mut(&self) -> &mut ECC_PR4_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut ECC_PR4_W8BIT) }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn ecc_pr4(&self) -> &ECC_PR4 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const ECC_PR4) }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn ecc_pr4_mut(&self) -> &mut ECC_PR4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut ECC_PR4) }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn ecc_pr5_w8bit(&self) -> &ECC_PR5_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const ECC_PR5_W8BIT) }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn ecc_pr5_w8bit_mut(&self) -> &mut ECC_PR5_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut ECC_PR5_W8BIT) }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn ecc_pr5(&self) -> &ECC_PR5 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const ECC_PR5) }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn ecc_pr5_mut(&self) -> &mut ECC_PR5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut ECC_PR5) }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn ecc_pr6_w8bit(&self) -> &ECC_PR6_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const ECC_PR6_W8BIT) }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn ecc_pr6_w8bit_mut(&self) -> &mut ECC_PR6_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut ECC_PR6_W8BIT) }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn ecc_pr6(&self) -> &ECC_PR6 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const ECC_PR6) }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn ecc_pr6_mut(&self) -> &mut ECC_PR6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut ECC_PR6) }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn ecc_pr7_w8bit(&self) -> &ECC_PR7_W8BIT {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const ECC_PR7_W8BIT) }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn ecc_pr7_w8bit_mut(&self) -> &mut ECC_PR7_W8BIT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut ECC_PR7_W8BIT) }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn ecc_pr7(&self) -> &ECC_PR7 {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const ECC_PR7) }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn ecc_pr7_mut(&self) -> &mut ECC_PR7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut ECC_PR7) }
    }
}
#[doc = "SMC NFC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "SMC NFC Configuration Register"]
pub mod cfg;
#[doc = "SMC NFC Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SMC NFC Control Register"]
pub mod ctrl;
#[doc = "SMC NFC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "SMC NFC Status Register"]
pub mod sr;
#[doc = "SMC NFC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "SMC NFC Interrupt Enable Register"]
pub mod ier;
#[doc = "SMC NFC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "SMC NFC Interrupt Disable Register"]
pub mod idr;
#[doc = "SMC NFC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "SMC NFC Interrupt Mask Register"]
pub mod imr;
#[doc = "SMC NFC Address Cycle Zero Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "SMC NFC Address Cycle Zero Register"]
pub mod addr;
#[doc = "SMC Bank Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank](bank) module"]
pub type BANK = crate::Reg<u32, _BANK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BANK;
#[doc = "`read()` method returns [bank::R](bank::R) reader structure"]
impl crate::Readable for BANK {}
#[doc = "`write(|w| ..)` method takes [bank::W](bank::W) writer structure"]
impl crate::Writable for BANK {}
#[doc = "SMC Bank Address Register"]
pub mod bank;
#[doc = "SMC ECC Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_ctrl](ecc_ctrl) module"]
pub type ECC_CTRL = crate::Reg<u32, _ECC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_CTRL;
#[doc = "`write(|w| ..)` method takes [ecc_ctrl::W](ecc_ctrl::W) writer structure"]
impl crate::Writable for ECC_CTRL {}
#[doc = "SMC ECC Control Register"]
pub mod ecc_ctrl;
#[doc = "SMC ECC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_md](ecc_md) module"]
pub type ECC_MD = crate::Reg<u32, _ECC_MD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_MD;
#[doc = "`read()` method returns [ecc_md::R](ecc_md::R) reader structure"]
impl crate::Readable for ECC_MD {}
#[doc = "`write(|w| ..)` method takes [ecc_md::W](ecc_md::W) writer structure"]
impl crate::Writable for ECC_MD {}
#[doc = "SMC ECC Mode Register"]
pub mod ecc_md;
#[doc = "SMC ECC Status 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_sr1](ecc_sr1) module"]
pub type ECC_SR1 = crate::Reg<u32, _ECC_SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_SR1;
#[doc = "`read()` method returns [ecc_sr1::R](ecc_sr1::R) reader structure"]
impl crate::Readable for ECC_SR1 {}
#[doc = "SMC ECC Status 1 Register"]
pub mod ecc_sr1;
#[doc = "SMC ECC Parity 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr0](ecc_pr0) module"]
pub type ECC_PR0 = crate::Reg<u32, _ECC_PR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR0;
#[doc = "`read()` method returns [ecc_pr0::R](ecc_pr0::R) reader structure"]
impl crate::Readable for ECC_PR0 {}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0;
#[doc = "SMC ECC Parity 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr0_w9bit](ecc_pr0_w9bit) module"]
pub type ECC_PR0_W9BIT = crate::Reg<u32, _ECC_PR0_W9BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR0_W9BIT;
#[doc = "`read()` method returns [ecc_pr0_w9bit::R](ecc_pr0_w9bit::R) reader structure"]
impl crate::Readable for ECC_PR0_W9BIT {}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0_w9bit;
#[doc = "SMC ECC Parity 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr0_w8bit](ecc_pr0_w8bit) module"]
pub type ECC_PR0_W8BIT = crate::Reg<u32, _ECC_PR0_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR0_W8BIT;
#[doc = "`read()` method returns [ecc_pr0_w8bit::R](ecc_pr0_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR0_W8BIT {}
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0_w8bit;
#[doc = "SMC ECC parity 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr1](ecc_pr1) module"]
pub type ECC_PR1 = crate::Reg<u32, _ECC_PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR1;
#[doc = "`read()` method returns [ecc_pr1::R](ecc_pr1::R) reader structure"]
impl crate::Readable for ECC_PR1 {}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1;
#[doc = "SMC ECC parity 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr1_w9bit](ecc_pr1_w9bit) module"]
pub type ECC_PR1_W9BIT = crate::Reg<u32, _ECC_PR1_W9BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR1_W9BIT;
#[doc = "`read()` method returns [ecc_pr1_w9bit::R](ecc_pr1_w9bit::R) reader structure"]
impl crate::Readable for ECC_PR1_W9BIT {}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1_w9bit;
#[doc = "SMC ECC parity 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr1_w8bit](ecc_pr1_w8bit) module"]
pub type ECC_PR1_W8BIT = crate::Reg<u32, _ECC_PR1_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR1_W8BIT;
#[doc = "`read()` method returns [ecc_pr1_w8bit::R](ecc_pr1_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR1_W8BIT {}
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1_w8bit;
#[doc = "SMC ECC status 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_sr2](ecc_sr2) module"]
pub type ECC_SR2 = crate::Reg<u32, _ECC_SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_SR2;
#[doc = "`read()` method returns [ecc_sr2::R](ecc_sr2::R) reader structure"]
impl crate::Readable for ECC_SR2 {}
#[doc = "SMC ECC status 2 Register"]
pub mod ecc_sr2;
#[doc = "SMC ECC parity 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr2](ecc_pr2) module"]
pub type ECC_PR2 = crate::Reg<u32, _ECC_PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR2;
#[doc = "`read()` method returns [ecc_pr2::R](ecc_pr2::R) reader structure"]
impl crate::Readable for ECC_PR2 {}
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2;
#[doc = "SMC ECC parity 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr2_w8bit](ecc_pr2_w8bit) module"]
pub type ECC_PR2_W8BIT = crate::Reg<u32, _ECC_PR2_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR2_W8BIT;
#[doc = "`read()` method returns [ecc_pr2_w8bit::R](ecc_pr2_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR2_W8BIT {}
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2_w8bit;
#[doc = "SMC ECC parity 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr3](ecc_pr3) module"]
pub type ECC_PR3 = crate::Reg<u32, _ECC_PR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR3;
#[doc = "`read()` method returns [ecc_pr3::R](ecc_pr3::R) reader structure"]
impl crate::Readable for ECC_PR3 {}
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3;
#[doc = "SMC ECC parity 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr3_w8bit](ecc_pr3_w8bit) module"]
pub type ECC_PR3_W8BIT = crate::Reg<u32, _ECC_PR3_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR3_W8BIT;
#[doc = "`read()` method returns [ecc_pr3_w8bit::R](ecc_pr3_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR3_W8BIT {}
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3_w8bit;
#[doc = "SMC ECC parity 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr4](ecc_pr4) module"]
pub type ECC_PR4 = crate::Reg<u32, _ECC_PR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR4;
#[doc = "`read()` method returns [ecc_pr4::R](ecc_pr4::R) reader structure"]
impl crate::Readable for ECC_PR4 {}
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4;
#[doc = "SMC ECC parity 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr4_w8bit](ecc_pr4_w8bit) module"]
pub type ECC_PR4_W8BIT = crate::Reg<u32, _ECC_PR4_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR4_W8BIT;
#[doc = "`read()` method returns [ecc_pr4_w8bit::R](ecc_pr4_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR4_W8BIT {}
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4_w8bit;
#[doc = "SMC ECC parity 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr5](ecc_pr5) module"]
pub type ECC_PR5 = crate::Reg<u32, _ECC_PR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR5;
#[doc = "`read()` method returns [ecc_pr5::R](ecc_pr5::R) reader structure"]
impl crate::Readable for ECC_PR5 {}
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5;
#[doc = "SMC ECC parity 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr5_w8bit](ecc_pr5_w8bit) module"]
pub type ECC_PR5_W8BIT = crate::Reg<u32, _ECC_PR5_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR5_W8BIT;
#[doc = "`read()` method returns [ecc_pr5_w8bit::R](ecc_pr5_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR5_W8BIT {}
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5_w8bit;
#[doc = "SMC ECC parity 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr6](ecc_pr6) module"]
pub type ECC_PR6 = crate::Reg<u32, _ECC_PR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR6;
#[doc = "`read()` method returns [ecc_pr6::R](ecc_pr6::R) reader structure"]
impl crate::Readable for ECC_PR6 {}
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6;
#[doc = "SMC ECC parity 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr6_w8bit](ecc_pr6_w8bit) module"]
pub type ECC_PR6_W8BIT = crate::Reg<u32, _ECC_PR6_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR6_W8BIT;
#[doc = "`read()` method returns [ecc_pr6_w8bit::R](ecc_pr6_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR6_W8BIT {}
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6_w8bit;
#[doc = "SMC ECC parity 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr7](ecc_pr7) module"]
pub type ECC_PR7 = crate::Reg<u32, _ECC_PR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR7;
#[doc = "`read()` method returns [ecc_pr7::R](ecc_pr7::R) reader structure"]
impl crate::Readable for ECC_PR7 {}
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7;
#[doc = "SMC ECC parity 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr7_w8bit](ecc_pr7_w8bit) module"]
pub type ECC_PR7_W8BIT = crate::Reg<u32, _ECC_PR7_W8BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR7_W8BIT;
#[doc = "`read()` method returns [ecc_pr7_w8bit::R](ecc_pr7_w8bit::R) reader structure"]
impl crate::Readable for ECC_PR7_W8BIT {}
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7_w8bit;
#[doc = "SMC ECC parity 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr8](ecc_pr8) module"]
pub type ECC_PR8 = crate::Reg<u32, _ECC_PR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR8;
#[doc = "`read()` method returns [ecc_pr8::R](ecc_pr8::R) reader structure"]
impl crate::Readable for ECC_PR8 {}
#[doc = "SMC ECC parity 8 Register"]
pub mod ecc_pr8;
#[doc = "SMC ECC parity 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr9](ecc_pr9) module"]
pub type ECC_PR9 = crate::Reg<u32, _ECC_PR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR9;
#[doc = "`read()` method returns [ecc_pr9::R](ecc_pr9::R) reader structure"]
impl crate::Readable for ECC_PR9 {}
#[doc = "SMC ECC parity 9 Register"]
pub mod ecc_pr9;
#[doc = "SMC ECC parity 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr10](ecc_pr10) module"]
pub type ECC_PR10 = crate::Reg<u32, _ECC_PR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR10;
#[doc = "`read()` method returns [ecc_pr10::R](ecc_pr10::R) reader structure"]
impl crate::Readable for ECC_PR10 {}
#[doc = "SMC ECC parity 10 Register"]
pub mod ecc_pr10;
#[doc = "SMC ECC parity 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr11](ecc_pr11) module"]
pub type ECC_PR11 = crate::Reg<u32, _ECC_PR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR11;
#[doc = "`read()` method returns [ecc_pr11::R](ecc_pr11::R) reader structure"]
impl crate::Readable for ECC_PR11 {}
#[doc = "SMC ECC parity 11 Register"]
pub mod ecc_pr11;
#[doc = "SMC ECC parity 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr12](ecc_pr12) module"]
pub type ECC_PR12 = crate::Reg<u32, _ECC_PR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR12;
#[doc = "`read()` method returns [ecc_pr12::R](ecc_pr12::R) reader structure"]
impl crate::Readable for ECC_PR12 {}
#[doc = "SMC ECC parity 12 Register"]
pub mod ecc_pr12;
#[doc = "SMC ECC parity 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr13](ecc_pr13) module"]
pub type ECC_PR13 = crate::Reg<u32, _ECC_PR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR13;
#[doc = "`read()` method returns [ecc_pr13::R](ecc_pr13::R) reader structure"]
impl crate::Readable for ECC_PR13 {}
#[doc = "SMC ECC parity 13 Register"]
pub mod ecc_pr13;
#[doc = "SMC ECC parity 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr14](ecc_pr14) module"]
pub type ECC_PR14 = crate::Reg<u32, _ECC_PR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR14;
#[doc = "`read()` method returns [ecc_pr14::R](ecc_pr14::R) reader structure"]
impl crate::Readable for ECC_PR14 {}
#[doc = "SMC ECC parity 14 Register"]
pub mod ecc_pr14;
#[doc = "SMC ECC parity 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr15](ecc_pr15) module"]
pub type ECC_PR15 = crate::Reg<u32, _ECC_PR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_PR15;
#[doc = "`read()` method returns [ecc_pr15::R](ecc_pr15::R) reader structure"]
impl crate::Readable for ECC_PR15 {}
#[doc = "SMC ECC parity 15 Register"]
pub mod ecc_pr15;
#[doc = "SMC Setup Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup0](setup0) module"]
pub type SETUP0 = crate::Reg<u32, _SETUP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP0;
#[doc = "`read()` method returns [setup0::R](setup0::R) reader structure"]
impl crate::Readable for SETUP0 {}
#[doc = "`write(|w| ..)` method takes [setup0::W](setup0::W) writer structure"]
impl crate::Writable for SETUP0 {}
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "SMC Pulse Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse0](pulse0) module"]
pub type PULSE0 = crate::Reg<u32, _PULSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE0;
#[doc = "`read()` method returns [pulse0::R](pulse0::R) reader structure"]
impl crate::Readable for PULSE0 {}
#[doc = "`write(|w| ..)` method takes [pulse0::W](pulse0::W) writer structure"]
impl crate::Writable for PULSE0 {}
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "SMC Cycle Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle0](cycle0) module"]
pub type CYCLE0 = crate::Reg<u32, _CYCLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE0;
#[doc = "`read()` method returns [cycle0::R](cycle0::R) reader structure"]
impl crate::Readable for CYCLE0 {}
#[doc = "`write(|w| ..)` method takes [cycle0::W](cycle0::W) writer structure"]
impl crate::Writable for CYCLE0 {}
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "SMC Timings Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings0](timings0) module"]
pub type TIMINGS0 = crate::Reg<u32, _TIMINGS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS0;
#[doc = "`read()` method returns [timings0::R](timings0::R) reader structure"]
impl crate::Readable for TIMINGS0 {}
#[doc = "`write(|w| ..)` method takes [timings0::W](timings0::W) writer structure"]
impl crate::Writable for TIMINGS0 {}
#[doc = "SMC Timings Register (CS_number = 0)"]
pub mod timings0;
#[doc = "SMC Mode Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode0](mode0) module"]
pub type MODE0 = crate::Reg<u32, _MODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE0;
#[doc = "`read()` method returns [mode0::R](mode0::R) reader structure"]
impl crate::Readable for MODE0 {}
#[doc = "`write(|w| ..)` method takes [mode0::W](mode0::W) writer structure"]
impl crate::Writable for MODE0 {}
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SMC Setup Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup1](setup1) module"]
pub type SETUP1 = crate::Reg<u32, _SETUP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP1;
#[doc = "`read()` method returns [setup1::R](setup1::R) reader structure"]
impl crate::Readable for SETUP1 {}
#[doc = "`write(|w| ..)` method takes [setup1::W](setup1::W) writer structure"]
impl crate::Writable for SETUP1 {}
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "SMC Pulse Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse1](pulse1) module"]
pub type PULSE1 = crate::Reg<u32, _PULSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE1;
#[doc = "`read()` method returns [pulse1::R](pulse1::R) reader structure"]
impl crate::Readable for PULSE1 {}
#[doc = "`write(|w| ..)` method takes [pulse1::W](pulse1::W) writer structure"]
impl crate::Writable for PULSE1 {}
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "SMC Cycle Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle1](cycle1) module"]
pub type CYCLE1 = crate::Reg<u32, _CYCLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE1;
#[doc = "`read()` method returns [cycle1::R](cycle1::R) reader structure"]
impl crate::Readable for CYCLE1 {}
#[doc = "`write(|w| ..)` method takes [cycle1::W](cycle1::W) writer structure"]
impl crate::Writable for CYCLE1 {}
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "SMC Timings Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings1](timings1) module"]
pub type TIMINGS1 = crate::Reg<u32, _TIMINGS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS1;
#[doc = "`read()` method returns [timings1::R](timings1::R) reader structure"]
impl crate::Readable for TIMINGS1 {}
#[doc = "`write(|w| ..)` method takes [timings1::W](timings1::W) writer structure"]
impl crate::Writable for TIMINGS1 {}
#[doc = "SMC Timings Register (CS_number = 1)"]
pub mod timings1;
#[doc = "SMC Mode Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode1](mode1) module"]
pub type MODE1 = crate::Reg<u32, _MODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE1;
#[doc = "`read()` method returns [mode1::R](mode1::R) reader structure"]
impl crate::Readable for MODE1 {}
#[doc = "`write(|w| ..)` method takes [mode1::W](mode1::W) writer structure"]
impl crate::Writable for MODE1 {}
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SMC Setup Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup2](setup2) module"]
pub type SETUP2 = crate::Reg<u32, _SETUP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP2;
#[doc = "`read()` method returns [setup2::R](setup2::R) reader structure"]
impl crate::Readable for SETUP2 {}
#[doc = "`write(|w| ..)` method takes [setup2::W](setup2::W) writer structure"]
impl crate::Writable for SETUP2 {}
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "SMC Pulse Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse2](pulse2) module"]
pub type PULSE2 = crate::Reg<u32, _PULSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE2;
#[doc = "`read()` method returns [pulse2::R](pulse2::R) reader structure"]
impl crate::Readable for PULSE2 {}
#[doc = "`write(|w| ..)` method takes [pulse2::W](pulse2::W) writer structure"]
impl crate::Writable for PULSE2 {}
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "SMC Cycle Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle2](cycle2) module"]
pub type CYCLE2 = crate::Reg<u32, _CYCLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE2;
#[doc = "`read()` method returns [cycle2::R](cycle2::R) reader structure"]
impl crate::Readable for CYCLE2 {}
#[doc = "`write(|w| ..)` method takes [cycle2::W](cycle2::W) writer structure"]
impl crate::Writable for CYCLE2 {}
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "SMC Timings Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings2](timings2) module"]
pub type TIMINGS2 = crate::Reg<u32, _TIMINGS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS2;
#[doc = "`read()` method returns [timings2::R](timings2::R) reader structure"]
impl crate::Readable for TIMINGS2 {}
#[doc = "`write(|w| ..)` method takes [timings2::W](timings2::W) writer structure"]
impl crate::Writable for TIMINGS2 {}
#[doc = "SMC Timings Register (CS_number = 2)"]
pub mod timings2;
#[doc = "SMC Mode Register (CS_number = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode2](mode2) module"]
pub type MODE2 = crate::Reg<u32, _MODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE2;
#[doc = "`read()` method returns [mode2::R](mode2::R) reader structure"]
impl crate::Readable for MODE2 {}
#[doc = "`write(|w| ..)` method takes [mode2::W](mode2::W) writer structure"]
impl crate::Writable for MODE2 {}
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SMC Setup Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup3](setup3) module"]
pub type SETUP3 = crate::Reg<u32, _SETUP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP3;
#[doc = "`read()` method returns [setup3::R](setup3::R) reader structure"]
impl crate::Readable for SETUP3 {}
#[doc = "`write(|w| ..)` method takes [setup3::W](setup3::W) writer structure"]
impl crate::Writable for SETUP3 {}
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "SMC Pulse Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse3](pulse3) module"]
pub type PULSE3 = crate::Reg<u32, _PULSE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE3;
#[doc = "`read()` method returns [pulse3::R](pulse3::R) reader structure"]
impl crate::Readable for PULSE3 {}
#[doc = "`write(|w| ..)` method takes [pulse3::W](pulse3::W) writer structure"]
impl crate::Writable for PULSE3 {}
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "SMC Cycle Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle3](cycle3) module"]
pub type CYCLE3 = crate::Reg<u32, _CYCLE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE3;
#[doc = "`read()` method returns [cycle3::R](cycle3::R) reader structure"]
impl crate::Readable for CYCLE3 {}
#[doc = "`write(|w| ..)` method takes [cycle3::W](cycle3::W) writer structure"]
impl crate::Writable for CYCLE3 {}
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "SMC Timings Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings3](timings3) module"]
pub type TIMINGS3 = crate::Reg<u32, _TIMINGS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS3;
#[doc = "`read()` method returns [timings3::R](timings3::R) reader structure"]
impl crate::Readable for TIMINGS3 {}
#[doc = "`write(|w| ..)` method takes [timings3::W](timings3::W) writer structure"]
impl crate::Writable for TIMINGS3 {}
#[doc = "SMC Timings Register (CS_number = 3)"]
pub mod timings3;
#[doc = "SMC Mode Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode3](mode3) module"]
pub type MODE3 = crate::Reg<u32, _MODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE3;
#[doc = "`read()` method returns [mode3::R](mode3::R) reader structure"]
impl crate::Readable for MODE3 {}
#[doc = "`write(|w| ..)` method takes [mode3::W](mode3::W) writer structure"]
impl crate::Writable for MODE3 {}
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "SMC Setup Register (CS_number = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup4](setup4) module"]
pub type SETUP4 = crate::Reg<u32, _SETUP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP4;
#[doc = "`read()` method returns [setup4::R](setup4::R) reader structure"]
impl crate::Readable for SETUP4 {}
#[doc = "`write(|w| ..)` method takes [setup4::W](setup4::W) writer structure"]
impl crate::Writable for SETUP4 {}
#[doc = "SMC Setup Register (CS_number = 4)"]
pub mod setup4;
#[doc = "SMC Pulse Register (CS_number = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse4](pulse4) module"]
pub type PULSE4 = crate::Reg<u32, _PULSE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE4;
#[doc = "`read()` method returns [pulse4::R](pulse4::R) reader structure"]
impl crate::Readable for PULSE4 {}
#[doc = "`write(|w| ..)` method takes [pulse4::W](pulse4::W) writer structure"]
impl crate::Writable for PULSE4 {}
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub mod pulse4;
#[doc = "SMC Cycle Register (CS_number = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle4](cycle4) module"]
pub type CYCLE4 = crate::Reg<u32, _CYCLE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE4;
#[doc = "`read()` method returns [cycle4::R](cycle4::R) reader structure"]
impl crate::Readable for CYCLE4 {}
#[doc = "`write(|w| ..)` method takes [cycle4::W](cycle4::W) writer structure"]
impl crate::Writable for CYCLE4 {}
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub mod cycle4;
#[doc = "SMC Timings Register (CS_number = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings4](timings4) module"]
pub type TIMINGS4 = crate::Reg<u32, _TIMINGS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS4;
#[doc = "`read()` method returns [timings4::R](timings4::R) reader structure"]
impl crate::Readable for TIMINGS4 {}
#[doc = "`write(|w| ..)` method takes [timings4::W](timings4::W) writer structure"]
impl crate::Writable for TIMINGS4 {}
#[doc = "SMC Timings Register (CS_number = 4)"]
pub mod timings4;
#[doc = "SMC Mode Register (CS_number = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode4](mode4) module"]
pub type MODE4 = crate::Reg<u32, _MODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE4;
#[doc = "`read()` method returns [mode4::R](mode4::R) reader structure"]
impl crate::Readable for MODE4 {}
#[doc = "`write(|w| ..)` method takes [mode4::W](mode4::W) writer structure"]
impl crate::Writable for MODE4 {}
#[doc = "SMC Mode Register (CS_number = 4)"]
pub mod mode4;
#[doc = "SMC Setup Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup5](setup5) module"]
pub type SETUP5 = crate::Reg<u32, _SETUP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP5;
#[doc = "`read()` method returns [setup5::R](setup5::R) reader structure"]
impl crate::Readable for SETUP5 {}
#[doc = "`write(|w| ..)` method takes [setup5::W](setup5::W) writer structure"]
impl crate::Writable for SETUP5 {}
#[doc = "SMC Setup Register (CS_number = 5)"]
pub mod setup5;
#[doc = "SMC Pulse Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse5](pulse5) module"]
pub type PULSE5 = crate::Reg<u32, _PULSE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE5;
#[doc = "`read()` method returns [pulse5::R](pulse5::R) reader structure"]
impl crate::Readable for PULSE5 {}
#[doc = "`write(|w| ..)` method takes [pulse5::W](pulse5::W) writer structure"]
impl crate::Writable for PULSE5 {}
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub mod pulse5;
#[doc = "SMC Cycle Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle5](cycle5) module"]
pub type CYCLE5 = crate::Reg<u32, _CYCLE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE5;
#[doc = "`read()` method returns [cycle5::R](cycle5::R) reader structure"]
impl crate::Readable for CYCLE5 {}
#[doc = "`write(|w| ..)` method takes [cycle5::W](cycle5::W) writer structure"]
impl crate::Writable for CYCLE5 {}
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub mod cycle5;
#[doc = "SMC Timings Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings5](timings5) module"]
pub type TIMINGS5 = crate::Reg<u32, _TIMINGS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS5;
#[doc = "`read()` method returns [timings5::R](timings5::R) reader structure"]
impl crate::Readable for TIMINGS5 {}
#[doc = "`write(|w| ..)` method takes [timings5::W](timings5::W) writer structure"]
impl crate::Writable for TIMINGS5 {}
#[doc = "SMC Timings Register (CS_number = 5)"]
pub mod timings5;
#[doc = "SMC Mode Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode5](mode5) module"]
pub type MODE5 = crate::Reg<u32, _MODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE5;
#[doc = "`read()` method returns [mode5::R](mode5::R) reader structure"]
impl crate::Readable for MODE5 {}
#[doc = "`write(|w| ..)` method takes [mode5::W](mode5::W) writer structure"]
impl crate::Writable for MODE5 {}
#[doc = "SMC Mode Register (CS_number = 5)"]
pub mod mode5;
#[doc = "SMC Setup Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup6](setup6) module"]
pub type SETUP6 = crate::Reg<u32, _SETUP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP6;
#[doc = "`read()` method returns [setup6::R](setup6::R) reader structure"]
impl crate::Readable for SETUP6 {}
#[doc = "`write(|w| ..)` method takes [setup6::W](setup6::W) writer structure"]
impl crate::Writable for SETUP6 {}
#[doc = "SMC Setup Register (CS_number = 6)"]
pub mod setup6;
#[doc = "SMC Pulse Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse6](pulse6) module"]
pub type PULSE6 = crate::Reg<u32, _PULSE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE6;
#[doc = "`read()` method returns [pulse6::R](pulse6::R) reader structure"]
impl crate::Readable for PULSE6 {}
#[doc = "`write(|w| ..)` method takes [pulse6::W](pulse6::W) writer structure"]
impl crate::Writable for PULSE6 {}
#[doc = "SMC Pulse Register (CS_number = 6)"]
pub mod pulse6;
#[doc = "SMC Cycle Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle6](cycle6) module"]
pub type CYCLE6 = crate::Reg<u32, _CYCLE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE6;
#[doc = "`read()` method returns [cycle6::R](cycle6::R) reader structure"]
impl crate::Readable for CYCLE6 {}
#[doc = "`write(|w| ..)` method takes [cycle6::W](cycle6::W) writer structure"]
impl crate::Writable for CYCLE6 {}
#[doc = "SMC Cycle Register (CS_number = 6)"]
pub mod cycle6;
#[doc = "SMC Timings Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings6](timings6) module"]
pub type TIMINGS6 = crate::Reg<u32, _TIMINGS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS6;
#[doc = "`read()` method returns [timings6::R](timings6::R) reader structure"]
impl crate::Readable for TIMINGS6 {}
#[doc = "`write(|w| ..)` method takes [timings6::W](timings6::W) writer structure"]
impl crate::Writable for TIMINGS6 {}
#[doc = "SMC Timings Register (CS_number = 6)"]
pub mod timings6;
#[doc = "SMC Mode Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode6](mode6) module"]
pub type MODE6 = crate::Reg<u32, _MODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE6;
#[doc = "`read()` method returns [mode6::R](mode6::R) reader structure"]
impl crate::Readable for MODE6 {}
#[doc = "`write(|w| ..)` method takes [mode6::W](mode6::W) writer structure"]
impl crate::Writable for MODE6 {}
#[doc = "SMC Mode Register (CS_number = 6)"]
pub mod mode6;
#[doc = "SMC Setup Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup7](setup7) module"]
pub type SETUP7 = crate::Reg<u32, _SETUP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP7;
#[doc = "`read()` method returns [setup7::R](setup7::R) reader structure"]
impl crate::Readable for SETUP7 {}
#[doc = "`write(|w| ..)` method takes [setup7::W](setup7::W) writer structure"]
impl crate::Writable for SETUP7 {}
#[doc = "SMC Setup Register (CS_number = 7)"]
pub mod setup7;
#[doc = "SMC Pulse Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse7](pulse7) module"]
pub type PULSE7 = crate::Reg<u32, _PULSE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSE7;
#[doc = "`read()` method returns [pulse7::R](pulse7::R) reader structure"]
impl crate::Readable for PULSE7 {}
#[doc = "`write(|w| ..)` method takes [pulse7::W](pulse7::W) writer structure"]
impl crate::Writable for PULSE7 {}
#[doc = "SMC Pulse Register (CS_number = 7)"]
pub mod pulse7;
#[doc = "SMC Cycle Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle7](cycle7) module"]
pub type CYCLE7 = crate::Reg<u32, _CYCLE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCLE7;
#[doc = "`read()` method returns [cycle7::R](cycle7::R) reader structure"]
impl crate::Readable for CYCLE7 {}
#[doc = "`write(|w| ..)` method takes [cycle7::W](cycle7::W) writer structure"]
impl crate::Writable for CYCLE7 {}
#[doc = "SMC Cycle Register (CS_number = 7)"]
pub mod cycle7;
#[doc = "SMC Timings Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings7](timings7) module"]
pub type TIMINGS7 = crate::Reg<u32, _TIMINGS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGS7;
#[doc = "`read()` method returns [timings7::R](timings7::R) reader structure"]
impl crate::Readable for TIMINGS7 {}
#[doc = "`write(|w| ..)` method takes [timings7::W](timings7::W) writer structure"]
impl crate::Writable for TIMINGS7 {}
#[doc = "SMC Timings Register (CS_number = 7)"]
pub mod timings7;
#[doc = "SMC Mode Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode7](mode7) module"]
pub type MODE7 = crate::Reg<u32, _MODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE7;
#[doc = "`read()` method returns [mode7::R](mode7::R) reader structure"]
impl crate::Readable for MODE7 {}
#[doc = "`write(|w| ..)` method takes [mode7::W](mode7::W) writer structure"]
impl crate::Writable for MODE7 {}
#[doc = "SMC Mode Register (CS_number = 7)"]
pub mod mode7;
#[doc = "SMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](ocms) module"]
pub type OCMS = crate::Reg<u32, _OCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMS;
#[doc = "`read()` method returns [ocms::R](ocms::R) reader structure"]
impl crate::Readable for OCMS {}
#[doc = "`write(|w| ..)` method takes [ocms::W](ocms::W) writer structure"]
impl crate::Writable for OCMS {}
#[doc = "SMC OCMS Register"]
pub mod ocms;
#[doc = "SMC OCMS KEY1 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "SMC OCMS KEY2 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "Write Protection Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](wpcr) module"]
pub type WPCR = crate::Reg<u32, _WPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR;
#[doc = "`write(|w| ..)` method takes [wpcr::W](wpcr::W) writer structure"]
impl crate::Writable for WPCR {}
#[doc = "Write Protection Control Register"]
pub mod wpcr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
