#[doc = r"Register block"]
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
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Baudrate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [br](br) module"]
pub type BR = crate::Reg<u32, _BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR;
#[doc = "`read()` method returns [br::R](br::R) reader structure"]
impl crate::Readable for BR {}
#[doc = "`write(|w| ..)` method takes [br::W](br::W) writer structure"]
impl crate::Writable for BR {}
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tim](tim) module"]
pub type TIM = crate::Reg<u32, _TIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM;
#[doc = "`read()` method returns [tim::R](tim::R) reader structure"]
impl crate::Readable for TIM {}
#[doc = "Timer Register"]
pub mod tim;
#[doc = "Timestamp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestp](timestp) module"]
pub type TIMESTP = crate::Reg<u32, _TIMESTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTP;
#[doc = "`read()` method returns [timestp::R](timestp::R) reader structure"]
impl crate::Readable for TIMESTP {}
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Transfer Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "Abort Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Abort Command Register"]
pub mod acr;
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
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "Mailbox Mode Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr0](mmr0) module"]
pub type MMR0 = crate::Reg<u32, _MMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR0;
#[doc = "`read()` method returns [mmr0::R](mmr0::R) reader structure"]
impl crate::Readable for MMR0 {}
#[doc = "`write(|w| ..)` method takes [mmr0::W](mmr0::W) writer structure"]
impl crate::Writable for MMR0 {}
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam0](mam0) module"]
pub type MAM0 = crate::Reg<u32, _MAM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM0;
#[doc = "`read()` method returns [mam0::R](mam0::R) reader structure"]
impl crate::Readable for MAM0 {}
#[doc = "`write(|w| ..)` method takes [mam0::W](mam0::W) writer structure"]
impl crate::Writable for MAM0 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "Mailbox ID Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid0](mid0) module"]
pub type MID0 = crate::Reg<u32, _MID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID0;
#[doc = "`read()` method returns [mid0::R](mid0::R) reader structure"]
impl crate::Readable for MID0 {}
#[doc = "`write(|w| ..)` method takes [mid0::W](mid0::W) writer structure"]
impl crate::Writable for MID0 {}
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "Mailbox Family ID Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid0](mfid0) module"]
pub type MFID0 = crate::Reg<u32, _MFID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID0;
#[doc = "`read()` method returns [mfid0::R](mfid0::R) reader structure"]
impl crate::Readable for MFID0 {}
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "Mailbox Status Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr0](msr0) module"]
pub type MSR0 = crate::Reg<u32, _MSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR0;
#[doc = "`read()` method returns [msr0::R](msr0::R) reader structure"]
impl crate::Readable for MSR0 {}
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "Mailbox Data Low Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl0](mdl0) module"]
pub type MDL0 = crate::Reg<u32, _MDL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL0;
#[doc = "`read()` method returns [mdl0::R](mdl0::R) reader structure"]
impl crate::Readable for MDL0 {}
#[doc = "`write(|w| ..)` method takes [mdl0::W](mdl0::W) writer structure"]
impl crate::Writable for MDL0 {}
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "Mailbox Data High Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh0](mdh0) module"]
pub type MDH0 = crate::Reg<u32, _MDH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH0;
#[doc = "`read()` method returns [mdh0::R](mdh0::R) reader structure"]
impl crate::Readable for MDH0 {}
#[doc = "`write(|w| ..)` method takes [mdh0::W](mdh0::W) writer structure"]
impl crate::Writable for MDH0 {}
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "Mailbox Control Register (MB = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr0](mcr0) module"]
pub type MCR0 = crate::Reg<u32, _MCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR0;
#[doc = "`write(|w| ..)` method takes [mcr0::W](mcr0::W) writer structure"]
impl crate::Writable for MCR0 {}
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "Mailbox Mode Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr1](mmr1) module"]
pub type MMR1 = crate::Reg<u32, _MMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR1;
#[doc = "`read()` method returns [mmr1::R](mmr1::R) reader structure"]
impl crate::Readable for MMR1 {}
#[doc = "`write(|w| ..)` method takes [mmr1::W](mmr1::W) writer structure"]
impl crate::Writable for MMR1 {}
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam1](mam1) module"]
pub type MAM1 = crate::Reg<u32, _MAM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM1;
#[doc = "`read()` method returns [mam1::R](mam1::R) reader structure"]
impl crate::Readable for MAM1 {}
#[doc = "`write(|w| ..)` method takes [mam1::W](mam1::W) writer structure"]
impl crate::Writable for MAM1 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "Mailbox ID Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid1](mid1) module"]
pub type MID1 = crate::Reg<u32, _MID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID1;
#[doc = "`read()` method returns [mid1::R](mid1::R) reader structure"]
impl crate::Readable for MID1 {}
#[doc = "`write(|w| ..)` method takes [mid1::W](mid1::W) writer structure"]
impl crate::Writable for MID1 {}
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "Mailbox Family ID Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid1](mfid1) module"]
pub type MFID1 = crate::Reg<u32, _MFID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID1;
#[doc = "`read()` method returns [mfid1::R](mfid1::R) reader structure"]
impl crate::Readable for MFID1 {}
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "Mailbox Status Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr1](msr1) module"]
pub type MSR1 = crate::Reg<u32, _MSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR1;
#[doc = "`read()` method returns [msr1::R](msr1::R) reader structure"]
impl crate::Readable for MSR1 {}
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "Mailbox Data Low Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl1](mdl1) module"]
pub type MDL1 = crate::Reg<u32, _MDL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL1;
#[doc = "`read()` method returns [mdl1::R](mdl1::R) reader structure"]
impl crate::Readable for MDL1 {}
#[doc = "`write(|w| ..)` method takes [mdl1::W](mdl1::W) writer structure"]
impl crate::Writable for MDL1 {}
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "Mailbox Data High Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh1](mdh1) module"]
pub type MDH1 = crate::Reg<u32, _MDH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH1;
#[doc = "`read()` method returns [mdh1::R](mdh1::R) reader structure"]
impl crate::Readable for MDH1 {}
#[doc = "`write(|w| ..)` method takes [mdh1::W](mdh1::W) writer structure"]
impl crate::Writable for MDH1 {}
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "Mailbox Control Register (MB = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr1](mcr1) module"]
pub type MCR1 = crate::Reg<u32, _MCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR1;
#[doc = "`write(|w| ..)` method takes [mcr1::W](mcr1::W) writer structure"]
impl crate::Writable for MCR1 {}
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "Mailbox Mode Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr2](mmr2) module"]
pub type MMR2 = crate::Reg<u32, _MMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR2;
#[doc = "`read()` method returns [mmr2::R](mmr2::R) reader structure"]
impl crate::Readable for MMR2 {}
#[doc = "`write(|w| ..)` method takes [mmr2::W](mmr2::W) writer structure"]
impl crate::Writable for MMR2 {}
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam2](mam2) module"]
pub type MAM2 = crate::Reg<u32, _MAM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM2;
#[doc = "`read()` method returns [mam2::R](mam2::R) reader structure"]
impl crate::Readable for MAM2 {}
#[doc = "`write(|w| ..)` method takes [mam2::W](mam2::W) writer structure"]
impl crate::Writable for MAM2 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "Mailbox ID Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid2](mid2) module"]
pub type MID2 = crate::Reg<u32, _MID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID2;
#[doc = "`read()` method returns [mid2::R](mid2::R) reader structure"]
impl crate::Readable for MID2 {}
#[doc = "`write(|w| ..)` method takes [mid2::W](mid2::W) writer structure"]
impl crate::Writable for MID2 {}
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "Mailbox Family ID Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid2](mfid2) module"]
pub type MFID2 = crate::Reg<u32, _MFID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID2;
#[doc = "`read()` method returns [mfid2::R](mfid2::R) reader structure"]
impl crate::Readable for MFID2 {}
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "Mailbox Status Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr2](msr2) module"]
pub type MSR2 = crate::Reg<u32, _MSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR2;
#[doc = "`read()` method returns [msr2::R](msr2::R) reader structure"]
impl crate::Readable for MSR2 {}
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "Mailbox Data Low Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl2](mdl2) module"]
pub type MDL2 = crate::Reg<u32, _MDL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL2;
#[doc = "`read()` method returns [mdl2::R](mdl2::R) reader structure"]
impl crate::Readable for MDL2 {}
#[doc = "`write(|w| ..)` method takes [mdl2::W](mdl2::W) writer structure"]
impl crate::Writable for MDL2 {}
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "Mailbox Data High Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh2](mdh2) module"]
pub type MDH2 = crate::Reg<u32, _MDH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH2;
#[doc = "`read()` method returns [mdh2::R](mdh2::R) reader structure"]
impl crate::Readable for MDH2 {}
#[doc = "`write(|w| ..)` method takes [mdh2::W](mdh2::W) writer structure"]
impl crate::Writable for MDH2 {}
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "Mailbox Control Register (MB = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr2](mcr2) module"]
pub type MCR2 = crate::Reg<u32, _MCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR2;
#[doc = "`write(|w| ..)` method takes [mcr2::W](mcr2::W) writer structure"]
impl crate::Writable for MCR2 {}
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "Mailbox Mode Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr3](mmr3) module"]
pub type MMR3 = crate::Reg<u32, _MMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR3;
#[doc = "`read()` method returns [mmr3::R](mmr3::R) reader structure"]
impl crate::Readable for MMR3 {}
#[doc = "`write(|w| ..)` method takes [mmr3::W](mmr3::W) writer structure"]
impl crate::Writable for MMR3 {}
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam3](mam3) module"]
pub type MAM3 = crate::Reg<u32, _MAM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM3;
#[doc = "`read()` method returns [mam3::R](mam3::R) reader structure"]
impl crate::Readable for MAM3 {}
#[doc = "`write(|w| ..)` method takes [mam3::W](mam3::W) writer structure"]
impl crate::Writable for MAM3 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "Mailbox ID Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid3](mid3) module"]
pub type MID3 = crate::Reg<u32, _MID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID3;
#[doc = "`read()` method returns [mid3::R](mid3::R) reader structure"]
impl crate::Readable for MID3 {}
#[doc = "`write(|w| ..)` method takes [mid3::W](mid3::W) writer structure"]
impl crate::Writable for MID3 {}
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "Mailbox Family ID Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid3](mfid3) module"]
pub type MFID3 = crate::Reg<u32, _MFID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID3;
#[doc = "`read()` method returns [mfid3::R](mfid3::R) reader structure"]
impl crate::Readable for MFID3 {}
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "Mailbox Status Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr3](msr3) module"]
pub type MSR3 = crate::Reg<u32, _MSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR3;
#[doc = "`read()` method returns [msr3::R](msr3::R) reader structure"]
impl crate::Readable for MSR3 {}
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "Mailbox Data Low Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl3](mdl3) module"]
pub type MDL3 = crate::Reg<u32, _MDL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL3;
#[doc = "`read()` method returns [mdl3::R](mdl3::R) reader structure"]
impl crate::Readable for MDL3 {}
#[doc = "`write(|w| ..)` method takes [mdl3::W](mdl3::W) writer structure"]
impl crate::Writable for MDL3 {}
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "Mailbox Data High Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh3](mdh3) module"]
pub type MDH3 = crate::Reg<u32, _MDH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH3;
#[doc = "`read()` method returns [mdh3::R](mdh3::R) reader structure"]
impl crate::Readable for MDH3 {}
#[doc = "`write(|w| ..)` method takes [mdh3::W](mdh3::W) writer structure"]
impl crate::Writable for MDH3 {}
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "Mailbox Control Register (MB = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr3](mcr3) module"]
pub type MCR3 = crate::Reg<u32, _MCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR3;
#[doc = "`write(|w| ..)` method takes [mcr3::W](mcr3::W) writer structure"]
impl crate::Writable for MCR3 {}
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "Mailbox Mode Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr4](mmr4) module"]
pub type MMR4 = crate::Reg<u32, _MMR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR4;
#[doc = "`read()` method returns [mmr4::R](mmr4::R) reader structure"]
impl crate::Readable for MMR4 {}
#[doc = "`write(|w| ..)` method takes [mmr4::W](mmr4::W) writer structure"]
impl crate::Writable for MMR4 {}
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam4](mam4) module"]
pub type MAM4 = crate::Reg<u32, _MAM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM4;
#[doc = "`read()` method returns [mam4::R](mam4::R) reader structure"]
impl crate::Readable for MAM4 {}
#[doc = "`write(|w| ..)` method takes [mam4::W](mam4::W) writer structure"]
impl crate::Writable for MAM4 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "Mailbox ID Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid4](mid4) module"]
pub type MID4 = crate::Reg<u32, _MID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID4;
#[doc = "`read()` method returns [mid4::R](mid4::R) reader structure"]
impl crate::Readable for MID4 {}
#[doc = "`write(|w| ..)` method takes [mid4::W](mid4::W) writer structure"]
impl crate::Writable for MID4 {}
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "Mailbox Family ID Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid4](mfid4) module"]
pub type MFID4 = crate::Reg<u32, _MFID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID4;
#[doc = "`read()` method returns [mfid4::R](mfid4::R) reader structure"]
impl crate::Readable for MFID4 {}
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "Mailbox Status Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr4](msr4) module"]
pub type MSR4 = crate::Reg<u32, _MSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR4;
#[doc = "`read()` method returns [msr4::R](msr4::R) reader structure"]
impl crate::Readable for MSR4 {}
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "Mailbox Data Low Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl4](mdl4) module"]
pub type MDL4 = crate::Reg<u32, _MDL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL4;
#[doc = "`read()` method returns [mdl4::R](mdl4::R) reader structure"]
impl crate::Readable for MDL4 {}
#[doc = "`write(|w| ..)` method takes [mdl4::W](mdl4::W) writer structure"]
impl crate::Writable for MDL4 {}
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "Mailbox Data High Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh4](mdh4) module"]
pub type MDH4 = crate::Reg<u32, _MDH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH4;
#[doc = "`read()` method returns [mdh4::R](mdh4::R) reader structure"]
impl crate::Readable for MDH4 {}
#[doc = "`write(|w| ..)` method takes [mdh4::W](mdh4::W) writer structure"]
impl crate::Writable for MDH4 {}
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "Mailbox Control Register (MB = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr4](mcr4) module"]
pub type MCR4 = crate::Reg<u32, _MCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR4;
#[doc = "`write(|w| ..)` method takes [mcr4::W](mcr4::W) writer structure"]
impl crate::Writable for MCR4 {}
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "Mailbox Mode Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr5](mmr5) module"]
pub type MMR5 = crate::Reg<u32, _MMR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR5;
#[doc = "`read()` method returns [mmr5::R](mmr5::R) reader structure"]
impl crate::Readable for MMR5 {}
#[doc = "`write(|w| ..)` method takes [mmr5::W](mmr5::W) writer structure"]
impl crate::Writable for MMR5 {}
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam5](mam5) module"]
pub type MAM5 = crate::Reg<u32, _MAM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM5;
#[doc = "`read()` method returns [mam5::R](mam5::R) reader structure"]
impl crate::Readable for MAM5 {}
#[doc = "`write(|w| ..)` method takes [mam5::W](mam5::W) writer structure"]
impl crate::Writable for MAM5 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "Mailbox ID Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid5](mid5) module"]
pub type MID5 = crate::Reg<u32, _MID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID5;
#[doc = "`read()` method returns [mid5::R](mid5::R) reader structure"]
impl crate::Readable for MID5 {}
#[doc = "`write(|w| ..)` method takes [mid5::W](mid5::W) writer structure"]
impl crate::Writable for MID5 {}
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "Mailbox Family ID Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid5](mfid5) module"]
pub type MFID5 = crate::Reg<u32, _MFID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID5;
#[doc = "`read()` method returns [mfid5::R](mfid5::R) reader structure"]
impl crate::Readable for MFID5 {}
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "Mailbox Status Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr5](msr5) module"]
pub type MSR5 = crate::Reg<u32, _MSR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR5;
#[doc = "`read()` method returns [msr5::R](msr5::R) reader structure"]
impl crate::Readable for MSR5 {}
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "Mailbox Data Low Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl5](mdl5) module"]
pub type MDL5 = crate::Reg<u32, _MDL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL5;
#[doc = "`read()` method returns [mdl5::R](mdl5::R) reader structure"]
impl crate::Readable for MDL5 {}
#[doc = "`write(|w| ..)` method takes [mdl5::W](mdl5::W) writer structure"]
impl crate::Writable for MDL5 {}
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "Mailbox Data High Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh5](mdh5) module"]
pub type MDH5 = crate::Reg<u32, _MDH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH5;
#[doc = "`read()` method returns [mdh5::R](mdh5::R) reader structure"]
impl crate::Readable for MDH5 {}
#[doc = "`write(|w| ..)` method takes [mdh5::W](mdh5::W) writer structure"]
impl crate::Writable for MDH5 {}
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "Mailbox Control Register (MB = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr5](mcr5) module"]
pub type MCR5 = crate::Reg<u32, _MCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR5;
#[doc = "`write(|w| ..)` method takes [mcr5::W](mcr5::W) writer structure"]
impl crate::Writable for MCR5 {}
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "Mailbox Mode Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr6](mmr6) module"]
pub type MMR6 = crate::Reg<u32, _MMR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR6;
#[doc = "`read()` method returns [mmr6::R](mmr6::R) reader structure"]
impl crate::Readable for MMR6 {}
#[doc = "`write(|w| ..)` method takes [mmr6::W](mmr6::W) writer structure"]
impl crate::Writable for MMR6 {}
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam6](mam6) module"]
pub type MAM6 = crate::Reg<u32, _MAM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM6;
#[doc = "`read()` method returns [mam6::R](mam6::R) reader structure"]
impl crate::Readable for MAM6 {}
#[doc = "`write(|w| ..)` method takes [mam6::W](mam6::W) writer structure"]
impl crate::Writable for MAM6 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "Mailbox ID Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid6](mid6) module"]
pub type MID6 = crate::Reg<u32, _MID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID6;
#[doc = "`read()` method returns [mid6::R](mid6::R) reader structure"]
impl crate::Readable for MID6 {}
#[doc = "`write(|w| ..)` method takes [mid6::W](mid6::W) writer structure"]
impl crate::Writable for MID6 {}
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "Mailbox Family ID Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid6](mfid6) module"]
pub type MFID6 = crate::Reg<u32, _MFID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID6;
#[doc = "`read()` method returns [mfid6::R](mfid6::R) reader structure"]
impl crate::Readable for MFID6 {}
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "Mailbox Status Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr6](msr6) module"]
pub type MSR6 = crate::Reg<u32, _MSR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR6;
#[doc = "`read()` method returns [msr6::R](msr6::R) reader structure"]
impl crate::Readable for MSR6 {}
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "Mailbox Data Low Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl6](mdl6) module"]
pub type MDL6 = crate::Reg<u32, _MDL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL6;
#[doc = "`read()` method returns [mdl6::R](mdl6::R) reader structure"]
impl crate::Readable for MDL6 {}
#[doc = "`write(|w| ..)` method takes [mdl6::W](mdl6::W) writer structure"]
impl crate::Writable for MDL6 {}
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "Mailbox Data High Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh6](mdh6) module"]
pub type MDH6 = crate::Reg<u32, _MDH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH6;
#[doc = "`read()` method returns [mdh6::R](mdh6::R) reader structure"]
impl crate::Readable for MDH6 {}
#[doc = "`write(|w| ..)` method takes [mdh6::W](mdh6::W) writer structure"]
impl crate::Writable for MDH6 {}
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "Mailbox Control Register (MB = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr6](mcr6) module"]
pub type MCR6 = crate::Reg<u32, _MCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR6;
#[doc = "`write(|w| ..)` method takes [mcr6::W](mcr6::W) writer structure"]
impl crate::Writable for MCR6 {}
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "Mailbox Mode Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmr7](mmr7) module"]
pub type MMR7 = crate::Reg<u32, _MMR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMR7;
#[doc = "`read()` method returns [mmr7::R](mmr7::R) reader structure"]
impl crate::Readable for MMR7 {}
#[doc = "`write(|w| ..)` method takes [mmr7::W](mmr7::W) writer structure"]
impl crate::Writable for MMR7 {}
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mam7](mam7) module"]
pub type MAM7 = crate::Reg<u32, _MAM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAM7;
#[doc = "`read()` method returns [mam7::R](mam7::R) reader structure"]
impl crate::Readable for MAM7 {}
#[doc = "`write(|w| ..)` method takes [mam7::W](mam7::W) writer structure"]
impl crate::Writable for MAM7 {}
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "Mailbox ID Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mid7](mid7) module"]
pub type MID7 = crate::Reg<u32, _MID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID7;
#[doc = "`read()` method returns [mid7::R](mid7::R) reader structure"]
impl crate::Readable for MID7 {}
#[doc = "`write(|w| ..)` method takes [mid7::W](mid7::W) writer structure"]
impl crate::Writable for MID7 {}
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "Mailbox Family ID Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfid7](mfid7) module"]
pub type MFID7 = crate::Reg<u32, _MFID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFID7;
#[doc = "`read()` method returns [mfid7::R](mfid7::R) reader structure"]
impl crate::Readable for MFID7 {}
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "Mailbox Status Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr7](msr7) module"]
pub type MSR7 = crate::Reg<u32, _MSR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR7;
#[doc = "`read()` method returns [msr7::R](msr7::R) reader structure"]
impl crate::Readable for MSR7 {}
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "Mailbox Data Low Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdl7](mdl7) module"]
pub type MDL7 = crate::Reg<u32, _MDL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDL7;
#[doc = "`read()` method returns [mdl7::R](mdl7::R) reader structure"]
impl crate::Readable for MDL7 {}
#[doc = "`write(|w| ..)` method takes [mdl7::W](mdl7::W) writer structure"]
impl crate::Writable for MDL7 {}
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "Mailbox Data High Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdh7](mdh7) module"]
pub type MDH7 = crate::Reg<u32, _MDH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDH7;
#[doc = "`read()` method returns [mdh7::R](mdh7::R) reader structure"]
impl crate::Readable for MDH7 {}
#[doc = "`write(|w| ..)` method takes [mdh7::W](mdh7::W) writer structure"]
impl crate::Writable for MDH7 {}
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "Mailbox Control Register (MB = 7)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr7](mcr7) module"]
pub type MCR7 = crate::Reg<u32, _MCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR7;
#[doc = "`write(|w| ..)` method takes [mcr7::W](mcr7::W) writer structure"]
impl crate::Writable for MCR7 {}
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
