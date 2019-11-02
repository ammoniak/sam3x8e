#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr0: CCR0,
    _reserved_1_cmr: [u8; 4usize],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr0: SMMR0,
    _reserved3: [u8; 4usize],
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
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    pub ccr1: CCR1,
    _reserved_12_cmr: [u8; 4usize],
    #[doc = "0x48 - Stepper Motor Mode Register (channel = 1)"]
    pub smmr1: SMMR1,
    _reserved14: [u8; 4usize],
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
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    pub ccr2: CCR2,
    _reserved_23_cmr: [u8; 4usize],
    #[doc = "0x88 - Stepper Motor Mode Register (channel = 2)"]
    pub smmr2: SMMR2,
    _reserved25: [u8; 4usize],
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
    _reserved33: [u8; 16usize],
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
    _reserved40: [u8; 8usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn cmr0_wave_eq_1(&self) -> &CMR0_WAVE_EQ_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CMR0_WAVE_EQ_1) }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn cmr0_wave_eq_1_mut(&self) -> &mut CMR0_WAVE_EQ_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut CMR0_WAVE_EQ_1) }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn cmr0(&self) -> &CMR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CMR0) }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn cmr0_mut(&self) -> &mut CMR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut CMR0) }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn cmr1_wave_eq_1(&self) -> &CMR1_WAVE_EQ_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const CMR1_WAVE_EQ_1) }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn cmr1_wave_eq_1_mut(&self) -> &mut CMR1_WAVE_EQ_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut CMR1_WAVE_EQ_1) }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn cmr1(&self) -> &CMR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const CMR1) }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn cmr1_mut(&self) -> &mut CMR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut CMR1) }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn cmr2_wave_eq_1(&self) -> &CMR2_WAVE_EQ_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CMR2_WAVE_EQ_1) }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn cmr2_wave_eq_1_mut(&self) -> &mut CMR2_WAVE_EQ_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CMR2_WAVE_EQ_1) }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn cmr2(&self) -> &CMR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CMR2) }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn cmr2_mut(&self) -> &mut CMR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CMR2) }
    }
}
#[doc = "Channel Control Register (channel = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr0](ccr0) module"]
pub type CCR0 = crate::Reg<u32, _CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR0;
#[doc = "`write(|w| ..)` method takes [ccr0::W](ccr0::W) writer structure"]
impl crate::Writable for CCR0 {}
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr0](cmr0) module"]
pub type CMR0 = crate::Reg<u32, _CMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR0;
#[doc = "`read()` method returns [cmr0::R](cmr0::R) reader structure"]
impl crate::Readable for CMR0 {}
#[doc = "`write(|w| ..)` method takes [cmr0::W](cmr0::W) writer structure"]
impl crate::Writable for CMR0 {}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr0_wave_eq_1](cmr0_wave_eq_1) module"]
pub type CMR0_WAVE_EQ_1 = crate::Reg<u32, _CMR0_WAVE_EQ_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR0_WAVE_EQ_1;
#[doc = "`read()` method returns [cmr0_wave_eq_1::R](cmr0_wave_eq_1::R) reader structure"]
impl crate::Readable for CMR0_WAVE_EQ_1 {}
#[doc = "`write(|w| ..)` method takes [cmr0_wave_eq_1::W](cmr0_wave_eq_1::W) writer structure"]
impl crate::Writable for CMR0_WAVE_EQ_1 {}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smmr0](smmr0) module"]
pub type SMMR0 = crate::Reg<u32, _SMMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR0;
#[doc = "`read()` method returns [smmr0::R](smmr0::R) reader structure"]
impl crate::Readable for SMMR0 {}
#[doc = "`write(|w| ..)` method takes [smmr0::W](smmr0::W) writer structure"]
impl crate::Writable for SMMR0 {}
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr0;
#[doc = "Counter Value (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv0](cv0) module"]
pub type CV0 = crate::Reg<u32, _CV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV0;
#[doc = "`read()` method returns [cv0::R](cv0::R) reader structure"]
impl crate::Readable for CV0 {}
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "Register A (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ra0](ra0) module"]
pub type RA0 = crate::Reg<u32, _RA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA0;
#[doc = "`read()` method returns [ra0::R](ra0::R) reader structure"]
impl crate::Readable for RA0 {}
#[doc = "`write(|w| ..)` method takes [ra0::W](ra0::W) writer structure"]
impl crate::Writable for RA0 {}
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "Register B (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rb0](rb0) module"]
pub type RB0 = crate::Reg<u32, _RB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RB0;
#[doc = "`read()` method returns [rb0::R](rb0::R) reader structure"]
impl crate::Readable for RB0 {}
#[doc = "`write(|w| ..)` method takes [rb0::W](rb0::W) writer structure"]
impl crate::Writable for RB0 {}
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "Register C (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rc0](rc0) module"]
pub type RC0 = crate::Reg<u32, _RC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC0;
#[doc = "`read()` method returns [rc0::R](rc0::R) reader structure"]
impl crate::Readable for RC0 {}
#[doc = "`write(|w| ..)` method takes [rc0::W](rc0::W) writer structure"]
impl crate::Writable for RC0 {}
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "Status Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr0](sr0) module"]
pub type SR0 = crate::Reg<u32, _SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR0;
#[doc = "`read()` method returns [sr0::R](sr0::R) reader structure"]
impl crate::Readable for SR0 {}
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "Interrupt Enable Register (channel = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier0](ier0) module"]
pub type IER0 = crate::Reg<u32, _IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER0;
#[doc = "`write(|w| ..)` method takes [ier0::W](ier0::W) writer structure"]
impl crate::Writable for IER0 {}
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "Interrupt Disable Register (channel = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr0](idr0) module"]
pub type IDR0 = crate::Reg<u32, _IDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR0;
#[doc = "`write(|w| ..)` method takes [idr0::W](idr0::W) writer structure"]
impl crate::Writable for IDR0 {}
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "Interrupt Mask Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr0](imr0) module"]
pub type IMR0 = crate::Reg<u32, _IMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR0;
#[doc = "`read()` method returns [imr0::R](imr0::R) reader structure"]
impl crate::Readable for IMR0 {}
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "Channel Control Register (channel = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr1](ccr1) module"]
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
#[doc = "`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure"]
impl crate::Writable for CCR1 {}
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "Channel Mode Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr1](cmr1) module"]
pub type CMR1 = crate::Reg<u32, _CMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR1;
#[doc = "`read()` method returns [cmr1::R](cmr1::R) reader structure"]
impl crate::Readable for CMR1 {}
#[doc = "`write(|w| ..)` method takes [cmr1::W](cmr1::W) writer structure"]
impl crate::Writable for CMR1 {}
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "Channel Mode Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr1_wave_eq_1](cmr1_wave_eq_1) module"]
pub type CMR1_WAVE_EQ_1 = crate::Reg<u32, _CMR1_WAVE_EQ_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR1_WAVE_EQ_1;
#[doc = "`read()` method returns [cmr1_wave_eq_1::R](cmr1_wave_eq_1::R) reader structure"]
impl crate::Readable for CMR1_WAVE_EQ_1 {}
#[doc = "`write(|w| ..)` method takes [cmr1_wave_eq_1::W](cmr1_wave_eq_1::W) writer structure"]
impl crate::Writable for CMR1_WAVE_EQ_1 {}
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smmr1](smmr1) module"]
pub type SMMR1 = crate::Reg<u32, _SMMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR1;
#[doc = "`read()` method returns [smmr1::R](smmr1::R) reader structure"]
impl crate::Readable for SMMR1 {}
#[doc = "`write(|w| ..)` method takes [smmr1::W](smmr1::W) writer structure"]
impl crate::Writable for SMMR1 {}
#[doc = "Stepper Motor Mode Register (channel = 1)"]
pub mod smmr1;
#[doc = "Counter Value (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv1](cv1) module"]
pub type CV1 = crate::Reg<u32, _CV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV1;
#[doc = "`read()` method returns [cv1::R](cv1::R) reader structure"]
impl crate::Readable for CV1 {}
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "Register A (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ra1](ra1) module"]
pub type RA1 = crate::Reg<u32, _RA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA1;
#[doc = "`read()` method returns [ra1::R](ra1::R) reader structure"]
impl crate::Readable for RA1 {}
#[doc = "`write(|w| ..)` method takes [ra1::W](ra1::W) writer structure"]
impl crate::Writable for RA1 {}
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "Register B (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rb1](rb1) module"]
pub type RB1 = crate::Reg<u32, _RB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RB1;
#[doc = "`read()` method returns [rb1::R](rb1::R) reader structure"]
impl crate::Readable for RB1 {}
#[doc = "`write(|w| ..)` method takes [rb1::W](rb1::W) writer structure"]
impl crate::Writable for RB1 {}
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "Register C (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rc1](rc1) module"]
pub type RC1 = crate::Reg<u32, _RC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC1;
#[doc = "`read()` method returns [rc1::R](rc1::R) reader structure"]
impl crate::Readable for RC1 {}
#[doc = "`write(|w| ..)` method takes [rc1::W](rc1::W) writer structure"]
impl crate::Writable for RC1 {}
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "Status Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "Interrupt Enable Register (channel = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "Interrupt Disable Register (channel = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr1](idr1) module"]
pub type IDR1 = crate::Reg<u32, _IDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR1;
#[doc = "`write(|w| ..)` method takes [idr1::W](idr1::W) writer structure"]
impl crate::Writable for IDR1 {}
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "Interrupt Mask Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "Channel Control Register (channel = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr2](ccr2) module"]
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
#[doc = "`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure"]
impl crate::Writable for CCR2 {}
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "Channel Mode Register (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr2](cmr2) module"]
pub type CMR2 = crate::Reg<u32, _CMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR2;
#[doc = "`read()` method returns [cmr2::R](cmr2::R) reader structure"]
impl crate::Readable for CMR2 {}
#[doc = "`write(|w| ..)` method takes [cmr2::W](cmr2::W) writer structure"]
impl crate::Writable for CMR2 {}
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "Channel Mode Register (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmr2_wave_eq_1](cmr2_wave_eq_1) module"]
pub type CMR2_WAVE_EQ_1 = crate::Reg<u32, _CMR2_WAVE_EQ_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR2_WAVE_EQ_1;
#[doc = "`read()` method returns [cmr2_wave_eq_1::R](cmr2_wave_eq_1::R) reader structure"]
impl crate::Readable for CMR2_WAVE_EQ_1 {}
#[doc = "`write(|w| ..)` method takes [cmr2_wave_eq_1::W](cmr2_wave_eq_1::W) writer structure"]
impl crate::Writable for CMR2_WAVE_EQ_1 {}
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2_wave_eq_1;
#[doc = "Stepper Motor Mode Register (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smmr2](smmr2) module"]
pub type SMMR2 = crate::Reg<u32, _SMMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR2;
#[doc = "`read()` method returns [smmr2::R](smmr2::R) reader structure"]
impl crate::Readable for SMMR2 {}
#[doc = "`write(|w| ..)` method takes [smmr2::W](smmr2::W) writer structure"]
impl crate::Writable for SMMR2 {}
#[doc = "Stepper Motor Mode Register (channel = 2)"]
pub mod smmr2;
#[doc = "Counter Value (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv2](cv2) module"]
pub type CV2 = crate::Reg<u32, _CV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV2;
#[doc = "`read()` method returns [cv2::R](cv2::R) reader structure"]
impl crate::Readable for CV2 {}
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "Register A (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ra2](ra2) module"]
pub type RA2 = crate::Reg<u32, _RA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA2;
#[doc = "`read()` method returns [ra2::R](ra2::R) reader structure"]
impl crate::Readable for RA2 {}
#[doc = "`write(|w| ..)` method takes [ra2::W](ra2::W) writer structure"]
impl crate::Writable for RA2 {}
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "Register B (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rb2](rb2) module"]
pub type RB2 = crate::Reg<u32, _RB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RB2;
#[doc = "`read()` method returns [rb2::R](rb2::R) reader structure"]
impl crate::Readable for RB2 {}
#[doc = "`write(|w| ..)` method takes [rb2::W](rb2::W) writer structure"]
impl crate::Writable for RB2 {}
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "Register C (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rc2](rc2) module"]
pub type RC2 = crate::Reg<u32, _RC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC2;
#[doc = "`read()` method returns [rc2::R](rc2::R) reader structure"]
impl crate::Readable for RC2 {}
#[doc = "`write(|w| ..)` method takes [rc2::W](rc2::W) writer structure"]
impl crate::Writable for RC2 {}
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "Status Register (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr2](sr2) module"]
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
#[doc = "`read()` method returns [sr2::R](sr2::R) reader structure"]
impl crate::Readable for SR2 {}
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "Interrupt Enable Register (channel = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier2](ier2) module"]
pub type IER2 = crate::Reg<u32, _IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER2;
#[doc = "`write(|w| ..)` method takes [ier2::W](ier2::W) writer structure"]
impl crate::Writable for IER2 {}
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "Interrupt Disable Register (channel = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr2](idr2) module"]
pub type IDR2 = crate::Reg<u32, _IDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR2;
#[doc = "`write(|w| ..)` method takes [idr2::W](idr2::W) writer structure"]
impl crate::Writable for IDR2 {}
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "Interrupt Mask Register (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "Block Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bcr](bcr) module"]
pub type BCR = crate::Reg<u32, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
#[doc = "`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure"]
impl crate::Writable for BCR {}
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bmr](bmr) module"]
pub type BMR = crate::Reg<u32, _BMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMR;
#[doc = "`read()` method returns [bmr::R](bmr::R) reader structure"]
impl crate::Readable for BMR {}
#[doc = "`write(|w| ..)` method takes [bmr::W](bmr::W) writer structure"]
impl crate::Writable for BMR {}
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QDEC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qier](qier) module"]
pub type QIER = crate::Reg<u32, _QIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIER;
#[doc = "`write(|w| ..)` method takes [qier::W](qier::W) writer structure"]
impl crate::Writable for QIER {}
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QDEC Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qidr](qidr) module"]
pub type QIDR = crate::Reg<u32, _QIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIDR;
#[doc = "`write(|w| ..)` method takes [qidr::W](qidr::W) writer structure"]
impl crate::Writable for QIDR {}
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QDEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qimr](qimr) module"]
pub type QIMR = crate::Reg<u32, _QIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QIMR;
#[doc = "`read()` method returns [qimr::R](qimr::R) reader structure"]
impl crate::Readable for QIMR {}
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QDEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qisr](qisr) module"]
pub type QISR = crate::Reg<u32, _QISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QISR;
#[doc = "`read()` method returns [qisr::R](qisr::R) reader structure"]
impl crate::Readable for QISR {}
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmr](fmr) module"]
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
#[doc = "`read()` method returns [fmr::R](fmr::R) reader structure"]
impl crate::Readable for FMR {}
#[doc = "`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure"]
impl crate::Writable for FMR {}
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
