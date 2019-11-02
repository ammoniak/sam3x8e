#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register"]
    pub gpbr: [GPBR; 8],
}
#[doc = "General Purpose Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpbr](gpbr) module"]
pub type GPBR = crate::Reg<u32, _GPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPBR;
#[doc = "`read()` method returns [gpbr::R](gpbr::R) reader structure"]
impl crate::Readable for GPBR {}
#[doc = "`write(|w| ..)` method takes [gpbr::W](gpbr::W) writer structure"]
impl crate::Writable for GPBR {}
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
