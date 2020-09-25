#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub exid: EXID,
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](cidr) module"]
pub type CIDR = crate::Reg<u32, _CIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR;
#[doc = "`read()` method returns [cidr::R](cidr::R) reader structure"]
impl crate::Readable for CIDR {}
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exid](exid) module"]
pub type EXID = crate::Reg<u32, _EXID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXID;
#[doc = "`read()` method returns [exid::R](exid::R) reader structure"]
impl crate::Readable for EXID {}
#[doc = "Chip ID Extension Register"]
pub mod exid;
