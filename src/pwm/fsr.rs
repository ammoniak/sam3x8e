#[doc = "Reader of register FSR"]
pub type R = crate::R<u32, super::FSR>;
#[doc = "Reader of field `FIV`"]
pub type FIV_R = crate::R<u8, u8>;
#[doc = "Reader of field `FS`"]
pub type FS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Fault Input Value (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fiv(&self) -> FIV_R {
        FIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Status (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
