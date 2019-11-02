#[doc = "Reader of register ECC_PR0"]
pub type R = crate::R<u32, super::ECC_PR0>;
#[doc = "Reader of field `BITADDR`"]
pub type BITADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WORDADDR`"]
pub type WORDADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Bit Address"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BITADDR_R {
        BITADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Word Address"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WORDADDR_R {
        WORDADDR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
