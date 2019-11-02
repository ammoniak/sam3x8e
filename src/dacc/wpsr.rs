#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Reader of field `WPROTERR`"]
pub type WPROTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WPROTADDR`"]
pub type WPROTADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Write protection error"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Write protection error address"]
    #[inline(always)]
    pub fn wprotaddr(&self) -> WPROTADDR_R {
        WPROTADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
