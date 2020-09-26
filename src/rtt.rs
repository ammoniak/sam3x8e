#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Alarm Register"]
    pub ar: AR,
    #[doc = "0x08 - Value Register"]
    pub vr: VR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
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
#[doc = "Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](ar) module"]
pub type AR = crate::Reg<u32, _AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR;
#[doc = "`read()` method returns [ar::R](ar::R) reader structure"]
impl crate::Readable for AR {}
#[doc = "`write(|w| ..)` method takes [ar::W](ar::W) writer structure"]
impl crate::Writable for AR {}
#[doc = "Alarm Register"]
pub mod ar;
#[doc = "Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](vr) module"]
pub type VR = crate::Reg<u32, _VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VR;
#[doc = "`read()` method returns [vr::R](vr::R) reader structure"]
impl crate::Readable for VR {}
#[doc = "Value Register"]
pub mod vr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
