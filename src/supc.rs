#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub smmr: SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub mr: MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub wumr: WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub wuir: WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub sr: SR,
}
#[doc = "Supply Controller Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Supply Controller Control Register"]
pub mod cr;
#[doc = "Supply Controller Supply Monitor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](smmr) module"]
pub type SMMR = crate::Reg<u32, _SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMMR;
#[doc = "`read()` method returns [smmr::R](smmr::R) reader structure"]
impl crate::Readable for SMMR {}
#[doc = "`write(|w| ..)` method takes [smmr::W](smmr::W) writer structure"]
impl crate::Writable for SMMR {}
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod smmr;
#[doc = "Supply Controller Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Supply Controller Mode Register"]
pub mod mr;
#[doc = "Supply Controller Wake-up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wumr](wumr) module"]
pub type WUMR = crate::Reg<u32, _WUMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUMR;
#[doc = "`read()` method returns [wumr::R](wumr::R) reader structure"]
impl crate::Readable for WUMR {}
#[doc = "`write(|w| ..)` method takes [wumr::W](wumr::W) writer structure"]
impl crate::Writable for WUMR {}
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod wumr;
#[doc = "Supply Controller Wake-up Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuir](wuir) module"]
pub type WUIR = crate::Reg<u32, _WUIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUIR;
#[doc = "`read()` method returns [wuir::R](wuir::R) reader structure"]
impl crate::Readable for WUIR {}
#[doc = "`write(|w| ..)` method takes [wuir::W](wuir::W) writer structure"]
impl crate::Writable for WUIR {}
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod wuir;
#[doc = "Supply Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Supply Controller Status Register"]
pub mod sr;
