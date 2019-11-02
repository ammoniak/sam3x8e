#[doc = "Reader of register LCDR"]
pub type R = crate::R<u32, super::LCDR>;
#[doc = "Reader of field `LDATA`"]
pub type LDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `CHNB`"]
pub type CHNB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Channel Number"]
    #[inline(always)]
    pub fn chnb(&self) -> CHNB_R {
        CHNB_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
