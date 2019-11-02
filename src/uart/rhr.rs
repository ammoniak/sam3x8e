#[doc = "Reader of register RHR"]
pub type R = crate::R<u32, super::RHR>;
#[doc = "Reader of field `RXCHR`"]
pub type RXCHR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0xff) as u8)
    }
}
